use crate::engine::timing::hit_window::{HitRule, HitWindow, OrderedHitWindows};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GhJudgement {
    Hit,
    Miss,
}

pub type GhHitWindows = OrderedHitWindows<GhJudgement, 1>;

pub const GH_WINDOWS: GhHitWindows = OrderedHitWindows {
    rules: [HitRule {
        window: HitWindow::symmetric(100_000),
        judgement: GhJudgement::Hit,
    }],
    miss_judgement: GhJudgement::Miss,
    miss_after: Some(100_000),
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::timing::hit_window::HitWindows;

    #[test]
    fn test_gh_judgement() {
        let windows = GH_WINDOWS;

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
