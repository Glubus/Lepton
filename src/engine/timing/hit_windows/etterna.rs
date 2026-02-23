use crate::engine::timing::hit_window::{HitWindow, HitWindows};

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

pub struct EtternaHitWindows {
    pub marvelous: HitWindow,
    pub perfect: HitWindow,
    pub great: HitWindow,
    pub good: HitWindow,
    pub bad: HitWindow,
    pub boo: HitWindow,
}

impl HitWindows for EtternaHitWindows {
    type Judgement = EtternaJudgement;

    fn judge(&self, delta_us: i64) -> Option<Self::Judgement> {
        if self.marvelous.contains(delta_us) {
            Some(EtternaJudgement::Marvelous)
        } else if self.perfect.contains(delta_us) {
            Some(EtternaJudgement::Perfect)
        } else if self.great.contains(delta_us) {
            Some(EtternaJudgement::Great)
        } else if self.good.contains(delta_us) {
            Some(EtternaJudgement::Good)
        } else if self.bad.contains(delta_us) {
            Some(EtternaJudgement::Bad)
        } else if self.boo.contains(delta_us) {
            Some(EtternaJudgement::Boo)
        } else if delta_us >= self.boo.early && delta_us <= self.boo.late {
            Some(EtternaJudgement::Miss)
        } else if delta_us > self.boo.late {
            Some(EtternaJudgement::Miss)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_etterna_judgement() {
        let windows = EtternaHitWindows {
            marvelous: HitWindow::symmetric(22_500), // +/- 22.5ms
            perfect: HitWindow::symmetric(45_000),   // +/- 45ms
            great: HitWindow::symmetric(90_000),     // +/- 90ms
            good: HitWindow::symmetric(135_000),     // +/- 135ms
            bad: HitWindow::symmetric(180_000),      // +/- 180ms
            boo: HitWindow::new(-180_000, 225_000),  // Doesn't trigger early, only late is wider
        };

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
