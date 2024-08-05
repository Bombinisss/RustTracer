use rand::Rng;
fn random_double() -> f64 {
    // Returns a random real in [0, 1).
    let mut rng = rand::thread_rng();
    rng.gen::<f64>()
}

fn random_double_range(min: f64, max: f64) -> f64 {
    // Returns a random real in [min, max).
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}
fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}

pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub const fn new(min: f64, max: f64) -> Interval {
        Interval {min, max}
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub const EMPTY: Interval = Interval::new(f64::INFINITY, f64::NEG_INFINITY);
    pub const UNIVERSE: Interval = Interval::new(f64::NEG_INFINITY, f64::INFINITY);
}