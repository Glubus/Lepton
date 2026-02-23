pub struct HitWindow {
    pub early: i64,
    pub late: i64,
}

impl HitWindow {
    pub const fn new(early: i64, late: i64) -> Self {
        Self { early, late }
    }

    pub const fn symmetric(radius: i64) -> Self {
        Self {
            early: -radius,
            late: radius,
        }
    }

    pub const fn contains(&self, delta_us: i64) -> bool {
        delta_us >= self.early && delta_us <= self.late
    }

    pub const fn width(&self) -> i64 {
        self.late - self.early
    }
}

pub struct HitRule<J> {
    pub window: HitWindow,
    pub judgement: J,
}

pub struct OrderedHitWindows<J, const N: usize> {
    pub rules: [HitRule<J>; N],
    pub miss_judgement: J,
    pub miss_after: Option<i64>,
}

pub trait HitWindows {
    type Judgement;
    fn judge(&self, delta_us: i64) -> Option<Self::Judgement>;
}

impl<J: Copy, const N: usize> HitWindows for OrderedHitWindows<J, N> {
    type Judgement = J;

    fn judge(&self, delta_us: i64) -> Option<Self::Judgement> {
        for rule in &self.rules {
            if rule.window.contains(delta_us) {
                return Some(rule.judgement);
            }
        }

        if let Some(limit) = self.miss_after {
            if delta_us > limit {
                return Some(self.miss_judgement);
            }
        }

        None
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
}
