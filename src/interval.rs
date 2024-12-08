use core::f32;

pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    pub fn new() -> Interval {
        Interval {
            min: f32::NEG_INFINITY,
            max: f32::INFINITY,
        }
    }

    pub fn with_values(min: f32, max: f32) -> Interval {
        Interval { min, max }
    }

    pub fn size(&self) -> f32 {
        self.max - self.min
    }

    pub fn contians(&self, value: f32) -> bool {
        self.min <= value && value <= self.max
    }

    pub fn surrounds(&self, value: f32) -> bool {
        self.min < value && value < self.max
    }

    pub const EMPTY: Interval = Interval {
        min: f32::INFINITY,
        max: f32::NEG_INFINITY,
    };

    pub const UNIVERSE: Interval = Interval {
        min: f32::NEG_INFINITY,
        max: f32::INFINITY,
    };
}
