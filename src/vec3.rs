use crate::utils::{random_double, random_double_range};
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn new_unit_vector(&self) -> Vec3 {
        let length = self.length();
        Vec3::new(self.x / length, self.y / length, self.z / length)
    }

    pub fn unit_vector(v: Vec3) -> Vec3 {
        v / v.length()
    }

    pub fn random_unit_vector() -> Vec3 {
        return Vec3::unit_vector(Vec3::random_in_unit_sphere());
    }

    pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
        let on_unit_sphere = Vec3::random_unit_vector();
        if Vec3::dot(&on_unit_sphere, normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random_range(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_in_unit_disk() -> Vec3 {
        loop {
            let p = Vec3::new(
                random_double_range(-1.0, 1.0),
                random_double_range(-1.0, 1.0),
                0.0,
            );
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random() -> Vec3 {
        return Vec3::new(random_double(), random_double(), random_double());
    }

    pub fn random_range(min: f64, max: f64) -> Vec3 {
        Vec3::new(
            random_double_range(min, max),
            random_double_range(min, max),
            random_double_range(min, max),
        )
    }

    pub fn near_zero(&self) -> bool {
        self.x.abs() < f64::EPSILON && self.y.abs() < f64::EPSILON && self.z.abs() < f64::EPSILON
    }

    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        return v - 2.0 * Vec3::dot(&v, &n) * n;
    }

    pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = ((-*uv).dot(n)).min(1.0);
        let r_out_perp = (*uv + *n * cos_theta) * etai_over_etat;
        let r_out_parallel = *n * (-1.0 * (1.0 - r_out_perp.length_squared()).abs().sqrt());
        r_out_perp + r_out_parallel
    }

    pub fn normalized(&self) -> Vec3 {
        let len = self.length();
        Vec3 {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x(),
            y: self.y + other.y(),
            z: self.z + other.z(),
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x(),
            y: self.y - other.y(),
            z: self.z - other.z(),
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * other.x(),
            y: self.y * other.y(),
            z: self.z * other.z(),
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self * other.x(),
            y: self * other.y(),
            z: self * other.z(),
        }
    }
}

impl Div<Vec3> for f64 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self / other.x(),
            y: self / other.y(),
            z: self / other.z(),
        }
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x / other.x(),
            y: self.y / other.y(),
            z: self.z / other.z(),
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Vec3 {
        Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Vec3 {
        Vec3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Vec3) -> bool {
        self.x == other.x() && self.y == other.y() && self.z == other.z()
    }
}
