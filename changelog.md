
## [Unreleased] - 2026-02-23
### Added
- `timing` module in `engine` containing a robust hit timing evaluation system.
- `HitWindow` struct representing an asymmetric microsecond timing interval for judgements.
- `HitWindows` trait mapping time deltas to game-specific `Judgement` enums.
- Game-specific judgements and logic for `Osu`, `Etterna`, and `GuitarHero` implementations.

### Changed
- Shifted away from float seconds to integer microseconds (`i32`) to avoid f64 inaccuracies and improve determinism for replay timings.
