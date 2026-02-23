use crate::engine::timing::hit_window::{HitRule, HitWindow, OrderedHitWindows};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GhJudgement {
    Hit,
    Miss,
}

pub type GhHitWindows = OrderedHitWindows<GhJudgement, 1>;

/// Creates a HitWindow for Guitar Hero based on the window in milliseconds.
/// standard is usually around 140ms total (+/- 70ms).
pub const fn create_gh_windows(window_ms: i64) -> GhHitWindows {
    let window_us = window_ms * 1000;
    OrderedHitWindows {
        rules: [HitRule {
            window: HitWindow::symmetric(window_us),
            judgement: GhJudgement::Hit,
        }],
        miss_judgement: GhJudgement::Miss,
        miss_after: Some(window_us),
    }
}

impl Default for GhHitWindows {
    fn default() -> Self {
        create_gh_windows(100) // Default GH window is typically around 100ms
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::timing::hit_window::HitWindows;

    #[test]
    fn test_gh_judgement() {
        let windows = create_gh_windows(100);

        // Hit
        assert_eq!(windows.judge(0), Some(GhJudgement::Hit));
        assert_eq!(windows.judge(100_000), Some(GhJudgement::Hit));
        assert_eq!(windows.judge(-100_000), Some(GhJudgement::Hit));

        // Miss
        assert_eq!(windows.judge(100_001), Some(GhJudgement::Miss));

        // Ignore early
        assert_eq!(windows.judge(-100_001), None);
    }
}
