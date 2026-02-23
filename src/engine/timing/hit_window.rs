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

pub trait HitWindows {
    type Judgement;
    fn judge(&self, delta_us: i64) -> Option<Self::Judgement>;
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
