use crate::{cube, sphere};
use crate::hittables::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::utils::Interval;

pub enum Shapes {
    Sphere(sphere::Sphere),
    Cube(cube::Cube),
}

impl Hittable for Shapes {
    fn hit(&self, r: Ray, ray_t: Interval) -> Option<HitRecord> {
        match self {
            Shapes::Sphere(s) => s.hit(r, ray_t),
            Shapes::Cube(c) => c.hit(r, ray_t),
        }
    }
}