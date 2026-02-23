use crate::engine::timing::hit_window::{HitWindow, HitWindows};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OsuJudgement {
    Max,
    Great,
    Good,
    Ok,
    Meh,
    Miss,
}

pub struct OsuHitWindows {
    pub max: HitWindow,
    pub great: HitWindow,
    pub good: HitWindow,
    pub ok: HitWindow,
    pub meh: HitWindow,
}

impl HitWindows for OsuHitWindows {
    type Judgement = OsuJudgement;

    fn judge(&self, delta_us: i32) -> Option<Self::Judgement> {
        if self.max.contains(delta_us) {
            Some(OsuJudgement::Max)
        } else if self.great.contains(delta_us) {
            Some(OsuJudgement::Great)
        } else if self.good.contains(delta_us) {
            Some(OsuJudgement::Good)
        } else if self.ok.contains(delta_us) {
            Some(OsuJudgement::Ok)
        } else if self.meh.contains(delta_us) {
            Some(OsuJudgement::Meh)
        } else if delta_us >= self.meh.early && delta_us <= self.meh.late {
            Some(OsuJudgement::Miss)
        } else if delta_us > self.meh.late {
            Some(OsuJudgement::Miss)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_osu_judgement() {
        let windows = OsuHitWindows {
            max: HitWindow::symmetric(16_000),   // +/- 16ms
            great: HitWindow::symmetric(40_000), // +/- 40ms
            good: HitWindow::symmetric(73_000),  // +/- 73ms
            ok: HitWindow::symmetric(103_000),   // +/- 103ms
            meh: HitWindow::symmetric(127_000),  // +/- 127ms
        };

        // Perfect hitting range
        assert_eq!(windows.judge(0), Some(OsuJudgement::Max));
        assert_eq!(windows.judge(16_000), Some(OsuJudgement::Max));
        assert_eq!(windows.judge(-16_000), Some(OsuJudgement::Max));

        // Great hitting range
        assert_eq!(windows.judge(16_001), Some(OsuJudgement::Great));
        assert_eq!(windows.judge(40_000), Some(OsuJudgement::Great));
        assert_eq!(windows.judge(-40_000), Some(OsuJudgement::Great));

        // Good hitting range
        assert_eq!(windows.judge(40_001), Some(OsuJudgement::Good));
        assert_eq!(windows.judge(73_000), Some(OsuJudgement::Good));

        // Ok hitting range
        assert_eq!(windows.judge(73_001), Some(OsuJudgement::Ok));
        assert_eq!(windows.judge(-103_000), Some(OsuJudgement::Ok));

        // Meh hitting range
        assert_eq!(windows.judge(103_001), Some(OsuJudgement::Meh));
        assert_eq!(windows.judge(127_000), Some(OsuJudgement::Meh));

        // Miss range (late)
        assert_eq!(windows.judge(127_001), Some(OsuJudgement::Miss));
        assert_eq!(windows.judge(500_000), Some(OsuJudgement::Miss));

        // Ignore range (too early)
        assert_eq!(windows.judge(-127_001), None);
        assert_eq!(windows.judge(-500_000), None);
    }
}
