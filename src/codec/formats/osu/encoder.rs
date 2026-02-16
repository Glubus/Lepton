//! Encoder for osu! replay format.

use super::parser::OsuParser;
use super::types::{GameMode, KeyMania, OsuReplay, ReplayEventMania};
use crate::codec::traits::Encoder;
use crate::error::{LeptonError, LeptonResult};
use crate::model::ReplayData;

pub struct OsuEncoder;

impl Encoder for OsuEncoder {
    fn encode(chart: &ReplayData) -> LeptonResult<Vec<u8>> {
        // Convert ReplayData to OsuReplay

        // 1. Reconstruct events
        let mut events = Vec::new();
        let mut current_keys = 0u32;

        for input in &chart.inputs {
            let col = input.column();
            if col > 15 {
                return Err(LeptonError::Custom(
                    "Input column exceeds osu!mania 16k limit".into(),
                ));
            }

            let delta_us = input.delta_us;

            // Apply change
            let mask = 1 << col;
            if input.is_press() {
                current_keys |= mask;
            } else {
                current_keys &= !mask;
            }

            events.push(ReplayEventMania {
                time_delta: (delta_us as i64 / 1000) as i32,
                keys: KeyMania(current_keys),
            });
        }

        let osu_replay = OsuReplay {
            mode: GameMode::Mania,
            game_version: 20240101, // Default or dummy
            beatmap_hash: chart.hash.clone().unwrap_or_default(),
            username: "LeptonUser".to_string(), // Placeholder or from config?
            replay_hash: String::new(),         // Recalculate?
            count_300: 0,
            count_100: 0,
            count_50: 0,
            count_geki: 0,
            count_katu: 0,
            count_miss: 0,
            score: 0,
            max_combo: 0,
            perfect: true,
            mods: 0,
            life_bar_graph: String::new(),
            timestamp: 0,
            replay_data: events,
            replay_id: 0,
        };

        OsuParser::encode(&osu_replay)
    }
}
