
## [Unreleased] - 2026-02-23
### Added
- `timing` module in `engine` containing a robust hit timing evaluation system.
- Generic rule-based `OrderedHitWindows` engine for data-driven hit evaluations without hardcoded logic.
- Game-specific exact timing window formulas based on integer math for `Osu` (OD scaling), `Etterna` (J-level scaling), and `GuitarHero` (dynamic window).
- `HitWindow` struct representing an asymmetric microsecond timing interval for judgements.
- `HitRule` struct to link intervals and judgements chronologically.
- `HitWindows` trait mapping time deltas to game-specific `Judgement` enums.
- Default implementations for all timing windows.

### Changed
- Shifted away from float seconds to integer microseconds (`i64`) to avoid inaccuracies and improve determinism for replay timings.
- Refactored `OsuHitWindows`, `EtternaHitWindows`, `GhHitWindows` to use the new generic engine.
