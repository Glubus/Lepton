pub struct HitWindow {
    pub early: i32,
    pub late: i32,
}

impl HitWindow {
    pub const fn new(early: i32, late: i32) -> Self {
        Self { early, late }
    }

    pub const fn symmetric(radius: i32) -> Self {
        Self {
            early: -radius,
            late: radius,
        }
    }

    pub const fn contains(&self, delta_us: i32) -> bool {
        delta_us >= self.early && delta_us <= self.late
    }

    pub const fn width(&self) -> i32 {
        self.late - self.early
    }
}

pub trait HitWindows {
    type Judgement;
    fn judge(&self, delta_us: i32) -> Option<Self::Judgement>;
}

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

    fn judge(&self, delta_us: i32) -> Option<Self::Judgement> {
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
    fn test_hit_window_symmetric() {
        let window = HitWindow::symmetric(10_000); // 10ms
        assert_eq!(window.early, -10_000);
        assert_eq!(window.late, 10_000);
        assert_eq!(window.width(), 20_000);

        assert!(window.contains(0));
        assert!(window.contains(-10_000));
        assert!(window.contains(10_000));
        assert!(!window.contains(-10_001));
        assert!(!window.contains(10_001));
    }

    #[test]
    fn test_hit_window_asymmetric() {
        let window = HitWindow::new(-5_000, 10_000);
        assert_eq!(window.early, -5_000);
        assert_eq!(window.late, 10_000);
        assert_eq!(window.width(), 15_000);

        assert!(window.contains(0));
        assert!(window.contains(-5_000));
        assert!(window.contains(10_000));
        assert!(!window.contains(-5_001));
        assert!(!window.contains(10_001));
    }

    #[test]
    fn test_osu_judgement() {
        let windows = OsuHitWindows {
            max: HitWindow::symmetric(16_000),      // +/- 16ms
            great: HitWindow::symmetric(40_000),    // +/- 40ms
            good: HitWindow::symmetric(73_000),     // +/- 73ms
            ok: HitWindow::symmetric(103_000),      // +/- 103ms
            meh: HitWindow::symmetric(127_000),     // +/- 127ms
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
