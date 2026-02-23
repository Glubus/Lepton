use crate::engine::timing::hit_window::{HitWindow, HitWindows};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GhJudgement {
    Hit,
    Miss,
}

pub struct GhHitWindows {
    pub hit: HitWindow,
}

impl HitWindows for GhHitWindows {
    type Judgement = GhJudgement;

    fn judge(&self, delta_us: i32) -> Option<Self::Judgement> {
        if self.hit.contains(delta_us) {
            Some(GhJudgement::Hit)
        } else if delta_us >= self.hit.early && delta_us <= self.hit.late {
            Some(GhJudgement::Miss)
        } else if delta_us > self.hit.late {
            Some(GhJudgement::Miss)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gh_judgement() {
        let windows = GhHitWindows {
            hit: HitWindow::symmetric(100_000),
        };

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
