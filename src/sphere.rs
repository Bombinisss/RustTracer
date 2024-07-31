use crate::vec3::Vec3;
use std::f64;
use crate::hittables::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::utils::Interval;

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Sphere {
        let radius = f64::max(0.0, radius);
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let oc = self.center - r.origin;
        let a = r.direction.length_squared();
        let h = Vec3::dot(&r.direction, &oc);
        let c = oc.length_squared() - self.radius*self.radius;
        let discriminant = h*h - a*c;

        if discriminant < 0.0 { return false; }

        let sqrtd = f64::sqrt(discriminant);

        /* Find nearest root that lies in acceptable range */
        let root = (h - sqrtd) / a;
        if root <= ray_t.min || ray_t.max <= root { return false; }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);

        true
    }
}

