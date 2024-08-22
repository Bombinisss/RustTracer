use crate::vec3::Vec3;
use std::sync::Arc;

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3;
}

pub struct SolidColor {
    albedo: Vec3,
}

impl SolidColor {
    pub fn new(albedo: Vec3) -> SolidColor {
        SolidColor { albedo }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: Vec3) -> Vec3 {
        self.albedo
    }
}

pub struct CheckerTexture {
    inv_scale: f64,
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(inv_scale: f64, even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> CheckerTexture {
        let inv_scale = 1.0 / inv_scale;
        CheckerTexture { inv_scale, even, odd }
    }

    pub fn new_from_rgb(inv_scale: f64, c1: Vec3, c2: Vec3) -> CheckerTexture {
        let inv_scale = 1.0 / inv_scale;
        let even = Arc::new(SolidColor::new(c1));
        let odd = Arc::new(SolidColor::new(c2));

        CheckerTexture { inv_scale, even, odd }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        let x_integer = (self.inv_scale * p.x()).floor() as i32;
        let y_integer = (self.inv_scale * p.y()).floor() as i32;
        let z_integer = (self.inv_scale * p.z()).floor() as i32;

        let is_even = (x_integer + y_integer + z_integer) % 2 == 0;

        if is_even {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}