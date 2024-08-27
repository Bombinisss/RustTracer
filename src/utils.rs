use rand::Rng;
pub fn random_double() -> f64 {
    // Returns a random real in [0, 1).
    let mut rng = rand::thread_rng();
    rng.random::<f64>()
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    // Returns a random real in [min, max).
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}

pub fn _random_int_range(min: i32, max: i32) -> i32 {
    random_double_range((min) as f64, (max + 1) as f64) as i32
}

// Maps normalized UV coordinates to the specified UV range for a face
pub fn map_uv_to_range(u: f64, v: f64, uv_range: &((f64, f64), (f64, f64))) -> (f64, f64) {
    let (u_min, v_min) = uv_range.0;
    let (u_max, v_max) = uv_range.1;

    // Normalize UV coordinates to the range [0, 1]
    let u_normalized = u;
    let v_normalized = v;

    // Map to the specified UV range
    let u_mapped = u_min + (u_normalized * (u_max - u_min));
    let v_mapped = v_min + (v_normalized * (v_max - v_min));

    (u_mapped, v_mapped)
}

#[derive(Debug, Clone, Copy)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub const fn new(min: f64, max: f64) -> Interval {
        Interval { min, max }
    }

    pub fn new_from_interval(a: &Interval, b: &Interval) -> Interval {
        let min = if a.min <= b.min { a.min } else { b.min };
        let max = if a.max >= b.max { a.max } else { b.max };

        Interval { min, max }
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

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            return self.min;
        }
        if x > self.max {
            return self.max;
        }
        x
    }

    pub fn expand(&self, delta: f64) -> Interval {
        let padding = delta / 2.0;
        Interval::new(self.min - padding, self.max + padding)
    }

    pub const EMPTY: Interval = Interval::new(f64::INFINITY, f64::NEG_INFINITY);
    pub const UNIVERSE: Interval = Interval::new(f64::NEG_INFINITY, f64::INFINITY);
}

impl Default for Interval {
    fn default() -> Self {
        Interval { min: 0.0, max: 0.0 }
    }
}
