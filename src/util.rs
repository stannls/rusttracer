use rand::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

pub const EMPTY_INTERVAL: Interval = Interval{ min: f64::MAX, max: f64::MIN};
pub const UNIVERSE_INTERVAL: Interval = Interval{ min: f64::MIN, max: f64::MAX };

impl Interval {
    pub fn new(min: f64, max: f64) -> Interval {
        Interval { min, max }
    }


    pub fn cost(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, val: f64) -> bool {
        self.min <= val && val <= self.max
    }

    pub fn surrounds(&self, val: f64) -> bool {
        self.min < val && val < self.max
    }

    pub fn clamp(&self, val: f64) -> f64 {
        if val < self.min {
            self.min
        } else if val > self.max {
            self.max
        } else {
            val
        }
    }
}

// Yields a random double in [0, 1)
#[inline(always)]
pub fn random_double() -> f64 {
    rand::thread_rng().gen_range(0.0..1.0)
}

// Yields a random double between [min, max)
#[inline(always)]
pub fn random_between(min: f64, max: f64) -> f64 {
    min + (max - min)*random_double()
}

#[inline(always)]
pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        f64::sqrt(linear_component)
    } else {
        0.0
    }
}
