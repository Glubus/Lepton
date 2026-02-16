//! Parser logic for osu! replay format.

use super::types::{GameMode, KeyMania, OsuReplay, ReplayEventMania};
use crate::error::{LeptonError, LeptonResult};
use std::io::{Cursor, Read, Write};

pub struct OsuParser;

impl OsuParser {
    pub fn parse(data: &[u8]) -> LeptonResult<OsuReplay> {
        let mut reader = Cursor::new(data);

        // 1. Game Mode (u8)
        let mode_byte = read_u8(&mut reader)?;
        let mode = GameMode::from(mode_byte);

        // 2. Version (u32)
        let version = read_u32(&mut reader)?;

        // 3. Beatmap Hash (String)
        let beatmap_hash = read_string(&mut reader)?;

        // 4. Player Name (String)
        let username = read_string(&mut reader)?;

        // 5. Replay Hash (String)
        let replay_hash = read_string(&mut reader)?;

        // 6. Judgements (u16 * 6)
        let count_300 = read_u16(&mut reader)?;
        let count_100 = read_u16(&mut reader)?;
        let count_50 = read_u16(&mut reader)?;
        let count_geki = read_u16(&mut reader)?;
        let count_katu = read_u16(&mut reader)?;
        let count_miss = read_u16(&mut reader)?;

        // 7. Score (u32)
        let score = read_u32(&mut reader)?;

        // 8. Max Combo (u16)
        let max_combo = read_u16(&mut reader)?;

        // 9. Perfect/Full Combo (u8/bool)
        let perfect = read_u8(&mut reader)? != 0;

        // 10. Mods (u32)
        let mods = read_u32(&mut reader)?;

        // 11. Life Bar Graph (String)
        let life_bar_graph = read_string(&mut reader)?;

        // 12. Timestamp (u64)
        let timestamp = read_u64(&mut reader)?;

        // 13. Compressed Replay Data Length (u32)
        let data_len = read_u32(&mut reader)?;

        // 14. Compressed Replay Data (LZMA)
        let mut compressed_data = vec![0u8; data_len as usize];
        reader
            .read_exact(&mut compressed_data)
            .map_err(LeptonError::Io)?;

        // Decompress LZMA
        let mut decompressed_data = Vec::new();
        if data_len > 0 {
            decompressed_data = liblzma::decode_all(&compressed_data[..])
                .map_err(|e| LeptonError::Custom(format!("LZMA Error: {}", e)))?;
        }

        // 15. Replay ID (u64) - Optional/Trailing
        let replay_id = if reader.position() < data.len() as u64 {
            read_u64(&mut reader).unwrap_or(0)
        } else {
            0
        };

        // Parse Replay Data String
        let replay_string = String::from_utf8(decompressed_data)
            .map_err(|e| LeptonError::Custom(format!("Invalid UTF-8 in replay data: {}", e)))?;

        let replay_events = if mode == GameMode::Mania {
            parse_mania_data(&replay_string)?
        } else {
            Vec::new()
        };

        Ok(OsuReplay {
            mode,
            game_version: version,
            beatmap_hash,
            username,
            replay_hash,
            count_300,
            count_100,
            count_50,
            count_geki,
            count_katu,
            count_miss,
            score,
            max_combo,
            perfect,
            mods,
            life_bar_graph,
            timestamp,
            replay_data: replay_events,
            replay_id: replay_id as i64,
        })
    }

    pub fn encode(replay: &OsuReplay) -> LeptonResult<Vec<u8>> {
        let mut writer = Cursor::new(Vec::new());

        // 1. Game Mode (u8)
        write_u8(&mut writer, replay.mode as u8)?;

        // 2. Version (u32)
        write_u32(&mut writer, replay.game_version)?;

        // 3. Beatmap Hash (String)
        write_string(&mut writer, &replay.beatmap_hash)?;

        // 4. Player Name (String)
        write_string(&mut writer, &replay.username)?;

        // 5. Replay Hash (String)
        write_string(&mut writer, &replay.replay_hash)?;

        // 6. Judgements (u16 * 6)
        write_u16(&mut writer, replay.count_300)?;
        write_u16(&mut writer, replay.count_100)?;
        write_u16(&mut writer, replay.count_50)?;
        write_u16(&mut writer, replay.count_geki)?;
        write_u16(&mut writer, replay.count_katu)?;
        write_u16(&mut writer, replay.count_miss)?;

        // 7. Score (u32)
        write_u32(&mut writer, replay.score)?;

        // 8. Max Combo (u16)
        write_u16(&mut writer, replay.max_combo)?;

        // 9. Perfect/Full Combo (u8/bool)
        write_u8(&mut writer, if replay.perfect { 1 } else { 0 })?;

        // 10. Mods (u32)
        write_u32(&mut writer, replay.mods)?;

        // 11. Life Bar Graph (String)
        write_string(&mut writer, &replay.life_bar_graph)?;

        // 12. Timestamp (u64)
        write_u64(&mut writer, replay.timestamp)?;

        // Create Replay Data String
        let replay_string = encode_mania_data(&replay.replay_data);

        // Compress LZMA
        let compressed_data = liblzma::encode_all(replay_string.as_bytes(), 6) // Level 6 default
            .map_err(|e| LeptonError::Custom(format!("LZMA compression error: {}", e)))?;

        // 13. Compressed Replay Data Length (u32)
        write_u32(&mut writer, compressed_data.len() as u32)?;

        // 14. Compressed Replay Data (LZMA)
        writer
            .write_all(&compressed_data)
            .map_err(LeptonError::Io)?;

        // 15. Replay ID (u64)
        write_u64(&mut writer, replay.replay_id as u64)?;

        Ok(writer.into_inner())
    }
}

// --- Helper Functions ---

fn read_u8(reader: &mut Cursor<&[u8]>) -> LeptonResult<u8> {
    let mut buf = [0u8; 1];
    reader.read_exact(&mut buf).map_err(LeptonError::Io)?;
    Ok(buf[0])
}

fn read_u16(reader: &mut Cursor<&[u8]>) -> LeptonResult<u16> {
    let mut buf = [0u8; 2];
    reader.read_exact(&mut buf).map_err(LeptonError::Io)?;
    Ok(u16::from_le_bytes(buf))
}

fn read_u32(reader: &mut Cursor<&[u8]>) -> LeptonResult<u32> {
    let mut buf = [0u8; 4];
    reader.read_exact(&mut buf).map_err(LeptonError::Io)?;
    Ok(u32::from_le_bytes(buf))
}

fn read_u64(reader: &mut Cursor<&[u8]>) -> LeptonResult<u64> {
    let mut buf = [0u8; 8];
    reader.read_exact(&mut buf).map_err(LeptonError::Io)?;
    Ok(u64::from_le_bytes(buf))
}

fn read_string(reader: &mut Cursor<&[u8]>) -> LeptonResult<String> {
    let b = read_u8(reader)?;
    if b == 0x00 {
        return Ok(String::new());
    }
    if b != 0x0b {
        return Err(LeptonError::Custom(format!(
            "Expected 0x0b for string start, got {:02x}",
            b
        )));
    }

    let len = leb128::read::unsigned(reader).map_err(LeptonError::Leb128Read)?;
    let mut buf = vec![0u8; len as usize];
    reader.read_exact(&mut buf).map_err(LeptonError::Io)?;

    Ok(String::from_utf8(buf).map_err(LeptonError::Utf8)?)
}

fn parse_mania_data(data: &str) -> LeptonResult<Vec<ReplayEventMania>> {
    let mut events = Vec::new();
    for action in data.split(',') {
        if action.is_empty() {
            continue;
        }
        let parts: Vec<&str> = action.split('|').collect();
        if parts.len() < 4 {
            continue;
        }

        let delta = parts[0].parse::<i64>().unwrap_or(0);
        if delta == -12345 {
            continue;
        }

        let keys = parts[3].parse::<u32>().unwrap_or(0);

        events.push(ReplayEventMania {
            time_delta: delta as i32,
            keys: KeyMania(keys),
        });
    }
    Ok(events)
}

fn write_u8(writer: &mut Cursor<Vec<u8>>, val: u8) -> LeptonResult<()> {
    writer.write_all(&[val]).map_err(LeptonError::Io)
}

fn write_u16(writer: &mut Cursor<Vec<u8>>, val: u16) -> LeptonResult<()> {
    writer
        .write_all(&val.to_le_bytes())
        .map_err(LeptonError::Io)
}

fn write_u32(writer: &mut Cursor<Vec<u8>>, val: u32) -> LeptonResult<()> {
    writer
        .write_all(&val.to_le_bytes())
        .map_err(LeptonError::Io)
}

fn write_u64(writer: &mut Cursor<Vec<u8>>, val: u64) -> LeptonResult<()> {
    writer
        .write_all(&val.to_le_bytes())
        .map_err(LeptonError::Io)
}

fn write_string(writer: &mut Cursor<Vec<u8>>, val: &str) -> LeptonResult<()> {
    if val.is_empty() {
        write_u8(writer, 0x00)
    } else {
        write_u8(writer, 0x0b)?;
        let len = val.len() as u64;
        leb128::write::unsigned(writer, len).map_err(|e| LeptonError::Io(e))?;
        writer.write_all(val.as_bytes()).map_err(LeptonError::Io)
    }
}

fn encode_mania_data(events: &[ReplayEventMania]) -> String {
    let mut data = String::new();
    // Default seed
    data.push_str("-12345|0|0|0,");

    for event in events {
        // format: w|x|y|z,
        // x,y are mouse coordinates, usually 0 or 256 for mania?
        // standard osu replays have cursor positions. Mania usually has 0?
        // Let's use 0.
        data.push_str(&format!("{}|0|0|{},", event.time_delta, event.keys.0));
    }
    data
}
