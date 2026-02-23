use crate::engine::timing::hit_window::{HitRule, HitWindow, OrderedHitWindows};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OsuJudgement {
    Marvelous,
    Perfect,
    Great,
    Good,
    Bad,
    Miss,
}

pub type OsuHitWindows = OrderedHitWindows<OsuJudgement, 5>;

const US_PER_MS: f32 = 1000.0;

/// Creates Osu Hit Windows based on the Overall Difficulty (OD).
pub const fn create_osu_windows(od: f32) -> OsuHitWindows {
    // Basic scaling for OD (higher OD = stricter windows)
    let max_us = 16_000;
    // Convert OD to an integer representation early, effectively OD * 10
    // e.g. OD 8.5 -> od_x10 = 85
    let od_x10 = (od * 10.0) as i64;
    // 3.0 ms per OD point = 3000 us per OD point
    // Using od_x10: 3000 us * od = 300 us * od_x10
    let reduction_us = 300 * od_x10;

    let perf_us = 64_000 - reduction_us;
    let great_us = 97_000 - reduction_us;
    let good_us = 127_000 - reduction_us;
    let bad_us = 151_000 - reduction_us;

    OrderedHitWindows {
        rules: [
            HitRule {
                window: HitWindow::symmetric(max_us),
                judgement: OsuJudgement::Marvelous,
            },
            HitRule {
                window: HitWindow::symmetric(perf_us),
                judgement: OsuJudgement::Perfect,
            },
            HitRule {
                window: HitWindow::symmetric(great_us),
                judgement: OsuJudgement::Great,
            },
            HitRule {
                window: HitWindow::symmetric(good_us),
                judgement: OsuJudgement::Good,
            },
            HitRule {
                window: HitWindow::symmetric(bad_us),
                judgement: OsuJudgement::Bad,
            },
        ],
        miss_judgement: OsuJudgement::Miss,
        miss_after: Some(bad_us),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::timing::hit_window::HitWindows;

    #[test]
    fn test_osu_judgement_base() {
        let windows = create_osu_windows(0.0);

        // Perfect hitting range
        assert_eq!(windows.judge(0), Some(OsuJudgement::Marvelous));
        assert_eq!(windows.judge(16_000), Some(OsuJudgement::Marvelous));
        assert_eq!(windows.judge(-16_000), Some(OsuJudgement::Marvelous));

        // Great hitting range
        assert_eq!(windows.judge(16_001), Some(OsuJudgement::Perfect));
        assert_eq!(windows.judge(64_000), Some(OsuJudgement::Perfect));

        // Good hitting range
        assert_eq!(windows.judge(64_001), Some(OsuJudgement::Great));
        assert_eq!(windows.judge(97_000), Some(OsuJudgement::Great));

        // Ok hitting range
        assert_eq!(windows.judge(97_001), Some(OsuJudgement::Good));
        assert_eq!(windows.judge(127_000), Some(OsuJudgement::Good));

        // Meh hitting range
        assert_eq!(windows.judge(127_001), Some(OsuJudgement::Bad));
        assert_eq!(windows.judge(151_000), Some(OsuJudgement::Bad));

        // Miss range (late)
        assert_eq!(windows.judge(151_001), Some(OsuJudgement::Miss));
        assert_eq!(windows.judge(500_000), Some(OsuJudgement::Miss));

        // Ignore range (too early)
        assert_eq!(windows.judge(-151_001), None);
        assert_eq!(windows.judge(-500_000), None);
    }
}
