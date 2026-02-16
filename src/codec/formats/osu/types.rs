//! Core types and enums for osu! replay data.
//!
//! Adapted from rosu-replay.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum GameMode {
    Std = 0,
    Taiko = 1,
    Catch = 2,
    Mania = 3,
}

impl From<u8> for GameMode {
    fn from(value: u8) -> Self {
        match value {
            0 => GameMode::Std,
            1 => GameMode::Taiko,
            2 => GameMode::Catch,
            3 => GameMode::Mania,
            _ => GameMode::Std, // Default fallback
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyMania(pub u32);

impl KeyMania {
    pub const K1: Self = Self(1 << 0);
    // ... we can implement more if needed, but value() is enough for bitwise ops

    pub fn value(&self) -> u32 {
        self.0
    }
}

impl From<u32> for KeyMania {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReplayEventMania {
    pub time_delta: i32,
    pub keys: KeyMania,
}

#[derive(Debug, Clone)]
pub struct OsuReplay {
    pub mode: GameMode,
    pub game_version: u32,
    pub beatmap_hash: String,
    pub username: String,
    pub replay_hash: String,
    pub count_300: u16,
    pub count_100: u16,
    pub count_50: u16,
    pub count_geki: u16,
    pub count_katu: u16,
    pub count_miss: u16,
    pub score: u32,
    pub max_combo: u16,
    pub perfect: bool,
    pub mods: u32,
    pub life_bar_graph: String, // Kept as string for simplicity unless needed
    pub timestamp: u64,         // Windows ticks
    pub replay_data: Vec<ReplayEventMania>,
    pub replay_id: i64,
}
