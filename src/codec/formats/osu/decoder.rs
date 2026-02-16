//! Decoder for osu! replay format.

use super::parser::OsuParser;
use super::types::GameMode;
use crate::codec::traits::Decoder;
use crate::error::{LeptonError, LeptonResult};
use crate::model::{ReplayData, ReplayInput};

pub struct OsuDecoder;

impl Decoder for OsuDecoder {
    fn decode(data: &[u8]) -> LeptonResult<ReplayData> {
        let osu_replay = OsuParser::parse(data)?;

        // Validate Mode
        if osu_replay.mode != GameMode::Mania {
            return Err(LeptonError::Custom(format!(
                "Unsupported game mode: {:?}. Only osu!mania is supported.",
                osu_replay.mode
            )));
        }

        // Convert events
        let mut inputs = Vec::new();
        let mut current_keys = 0u32;
        let mut accumulated_delta_ms = 0i64; // Time since last emitted input in ms

        // osu! timestamps are in ms (technically implementation dependent, but usually ms)
        // Lepton uses micros for deltas.

        for event in osu_replay.replay_data {
            let delta_ms = event.time_delta as i64;
            accumulated_delta_ms += delta_ms;

            let new_keys = event.keys.0;
            let changed_keys = current_keys ^ new_keys;

            if changed_keys == 0 {
                // No key change, just time passing (or mouse movement which is ignored in mania key-only)
                continue;
            }

            // check for columns > 15 (Lepton limit)
            if new_keys > 0xFFFF {
                if (new_keys & !0xFFFF) != 0 {
                    return Err(LeptonError::Custom(
                        "osu!mania key count exceeds Lepton limit (16 keys)".into(),
                    ));
                }
            }

            let mut first_event_in_frame = true;

            for col in 0..16 {
                let mask = 1 << col;
                if (changed_keys & mask) != 0 {
                    let is_press = (new_keys & mask) != 0;

                    let delta_us = if first_event_in_frame {
                        if accumulated_delta_ms < 0 {
                            0
                        } else {
                            (accumulated_delta_ms as u64) * 1000
                        }
                    } else {
                        0
                    };

                    inputs.push(ReplayInput::new(delta_us, col as u8, is_press, false));

                    if first_event_in_frame {
                        accumulated_delta_ms = 0;
                        first_event_in_frame = false;
                    }
                }
            }

            current_keys = new_keys;
        }

        Ok(ReplayData::with_params(
            1.0, // osu! replays are usually 1.0 rate unless modded with DT/HT, but the time deltas are already scaled in some parsers?
            // Actually, in osu!, DT makes time go faster, so deltas are smaller?
            // Or deltas are real time?
            // Usually replay data is stored in "audio time" or "game time".
            // For now, assume 1.0 and let the engine handle speed.
            Some(osu_replay.beatmap_hash),
            inputs,
        ))
    }
}
