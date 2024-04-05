use crate::rtweekend::INFINITY;
use std::ops::Add;

pub const UNIVERSE: Interval = Interval {
    min: -INFINITY,
    max: INFINITY,
};

#[derive(Clone)]
pub struct Interval {
    pub(crate) min: f64,
    pub(crate) max: f64,
}

const BLANK_INTERVAL: Interval = Interval { min: 0.0, max: 0.0 };

impl Interval {
    pub fn blank() -> Interval {
        BLANK_INTERVAL.clone()
    }

    pub fn from(min: f64, max: f64) -> Interval {
        Interval { min, max }
    }

    pub fn from_intervals(a: &Interval, b: &Interval) -> Interval {
        Interval::from(f64::min(a.min, b.min), f64::max(a.max, b.max))
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            return self.min;
        }
        if x > self.max {
            return self.max;
        }
        return x;
    }

    pub fn contains(&self, x: f64) -> bool {
        return self.min <= x && x <= self.max;
    }

    pub fn size(&self) -> f64 {
        return self.max - self.min;
    }

    pub fn expand(&self, delta: f64) -> Interval {
        let padding = delta / 2.0;
        Interval::from(self.min - padding, self.max + padding)
    }
}

impl Add<f64> for &Interval {
    type Output = Interval;

    fn add(self, rhs: f64) -> Self::Output {
        Interval::from(self.min + rhs, self.max + rhs)
    }
}

impl Add<&Interval> for f64 {
    type Output = Interval;

    fn add(self, rhs: &Interval) -> Self::Output {
        rhs + self
    }
}
