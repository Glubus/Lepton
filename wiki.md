
### 2026-02-23: Separation of Judgement and HitWindows
**Decision**: Split rhythm timing evaluation into two distinct concepts: `Judgement` and `HitWindows`.
**Why**: Different rhythm games (VSRG) use wildly different timing systems. Osu has 6 judgements, Etterna has 7, and Guitar Hero has 2. A single monolithic `HitTiming` type is not sufficient. 
**Impact**: Introduced `HitWindow` (an asymmetric microsecond interval), `HitWindows` trait, and game-specific enums (`OsuJudgement`, `EtternaJudgement`, etc.) in `src/engine/timing.rs`. All time delta calculations must now be done in integers (microseconds).
