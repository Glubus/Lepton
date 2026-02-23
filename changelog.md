# Changelog
All notable changes to the `Lepton` crate will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased] - 2026-02-23

### Added

#### Core Architecture

- `model` module containing internal game data representations.
- `input` handling with bitflags capability supporting up to 16 keys (VSRG standard limit).
- `error.rs` unified error handling enum `LeptonError` including `Io`, `Utf8`, and `Custom` variants.

#### Codec / Formats

- `codec` module for parsing and encoding replay formats.
- `Encoder` and `Decoder` standard traits in `codec::traits`.
- `lep` format: Custom lightweight binary replay format using LEB128 compression.
- `osu` format: Full support for parsing and writing `osu!mania` replay files (`.osr` format).

#### Engine & Timing System

- `timing` module in `engine` containing a robust hit timing evaluation system.
- Generic rule-based `OrderedHitWindows` engine for data-driven hit evaluations without hardcoded logic.
- Game-specific exact timing window formulas based on integer math for:
  - `Osu`: Overall Difficulty (OD) dynamic scaling.
  - `Etterna`: Judge Level (J-level) precision scaling.
  - `GuitarHero` (`CloneHero` style): Dynamic adjustable window (100ms default).
- `HitWindow` struct representing an asymmetric microsecond timing interval for judgements.
- `HitRule` struct to link intervals and judgements chronologically.
- `HitWindows` trait mapping time deltas to game-specific `Judgement` enums.
- Default implementations via `Default` trait for all timing windows.

### Changed
- Shifted away from float seconds to strictly integer microseconds (`i64`) inside the engine to prevent inaccuracies and ensure perfect determinism for replay timings.
- Refactored all game-specific hit windows (`OsuHitWindows`, `EtternaHitWindows`, `GhHitWindows`) to use the generic engine logic instead of procedural `if`/`else` control flows.
