use crate::engine::timing::hit_window::{HitRule, HitWindow, OrderedHitWindows};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EtternaJudgement {
    Marvelous,
    Perfect,
    Great,
    Good,
    Bad,
    Boo,
    Miss,
}

pub type EtternaHitWindows = OrderedHitWindows<EtternaJudgement, 6>;

/// Creates a HitWindow based on Etterna Judge Level (J4 = Standard = 4).
pub const fn create_etterna_windows(judge_level: i64) -> EtternaHitWindows {
    // Formula mathematically applied:
    // Scale for J9 = 0.2 (20 / 100)
    // Scale for others = 1.0 - ((J - 4) / 6.0)
    // In integer math (scale_x100), J=4 -> 100, J=5 -> 83, etc.
    // scale_x100 = 100 - (((judge_level - 4) * 100) / 6)
    let scale_x100 = if judge_level == 9 {
        20
    } else {
        100 - (((judge_level - 4) * 100) / 6)
    };

    // Base ms * 1000 = microseconds
    let base_marv_us = 22_500;
    let base_perf_us = 45_000;
    let base_great_us = 90_000;
    let base_good_us = 135_000;
    let base_bad_us = 180_000;

    // Etterna special rule: Bad never goes below 180ms
    let mut bad_calculated = (base_bad_us * scale_x100) / 100;
    if bad_calculated < 180_000 {
        bad_calculated = 180_000;
    }

    OrderedHitWindows {
        rules: [
            HitRule {
                window: HitWindow::symmetric((base_marv_us * scale_x100) / 100),
                judgement: EtternaJudgement::Marvelous,
            },
            HitRule {
                window: HitWindow::symmetric((base_perf_us * scale_x100) / 100),
                judgement: EtternaJudgement::Perfect,
            },
            HitRule {
                window: HitWindow::symmetric((base_great_us * scale_x100) / 100),
                judgement: EtternaJudgement::Great,
            },
            HitRule {
                window: HitWindow::symmetric((base_good_us * scale_x100) / 100),
                judgement: EtternaJudgement::Good,
            },
            HitRule {
                window: HitWindow::symmetric(bad_calculated),
                judgement: EtternaJudgement::Bad,
            },
            HitRule {
                window: HitWindow::new(-180_000, 225_000),
                judgement: EtternaJudgement::Boo,
            },
        ],
        miss_judgement: EtternaJudgement::Miss,
        miss_after: Some(225_000), // Etterna miss is beyond boo
    }
}

impl Default for EtternaHitWindows {
    fn default() -> Self {
        create_etterna_windows(4) // J4 is standard
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::timing::hit_window::HitWindows;

    #[test]
    fn test_etterna_judgement_j4() {
        // J4 has scale=1.0 so standard values apply
        let windows = create_etterna_windows(4);

        // Marvelous
        assert_eq!(windows.judge(22_500), Some(EtternaJudgement::Marvelous));
        assert_eq!(windows.judge(-22_500), Some(EtternaJudgement::Marvelous));
        
        // Perfect
        assert_eq!(windows.judge(22_501), Some(EtternaJudgement::Perfect));
        assert_eq!(windows.judge(-45_000), Some(EtternaJudgement::Perfect));

        // Great
        assert_eq!(windows.judge(90_000), Some(EtternaJudgement::Great));

        // Good
        assert_eq!(windows.judge(-135_000), Some(EtternaJudgement::Good));

        // Bad
        assert_eq!(windows.judge(180_000), Some(EtternaJudgement::Bad));
        assert_eq!(windows.judge(-180_000), Some(EtternaJudgement::Bad));

        // Boo (asymmetric example: late boo but no early boo beyond -180ms)
        assert_eq!(windows.judge(180_001), Some(EtternaJudgement::Boo));
        assert_eq!(windows.judge(225_000), Some(EtternaJudgement::Boo));

        // Early beyond bad
        assert_eq!(windows.judge(-180_001), None); // Boo early limit is -180ms, so this is None

        // Late beyond boo
        assert_eq!(windows.judge(225_001), Some(EtternaJudgement::Miss));
    }
}
