use crate::hittables::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::utils::Interval;
use crate::vec3::Vec3;

pub struct Cube {
    center: Vec3,
    size: f64,
}

impl Cube {
    pub fn new(center: Vec3, size: f64) -> Cube {
        let size = f64::max(0.0, size);
        Cube { center, size }
    }
}

impl Hittable for Cube {
    fn hit(&self, r: Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        // Half the size of the cube for calculations
        let half_size = self.size / 2.0;

        // Calculate the minimum and maximum bounds of the cube
        let min_bound = self.center - Vec3::new(half_size, half_size, half_size);
        let max_bound = self.center + Vec3::new(half_size, half_size, half_size);

        // Calculate the inverse of the ray direction
        let inv_d = Vec3::new(1.0 / r.direction.x(), 1.0 / r.direction.y(), 1.0 / r.direction.z());

        // Calculate t0 and t1 for each axis
        let t0s = (min_bound - r.origin) * inv_d;
        let t1s = (max_bound - r.origin) * inv_d;

        // Find the minimum and maximum intersection times
        let t_min_x = t0s.x().min(t1s.x());
        let t_max_x = t0s.x().max(t1s.x());
        let t_min_y = t0s.y().min(t1s.y());
        let t_max_y = t0s.y().max(t1s.y());
        let t_min_z = t0s.z().min(t1s.z());
        let t_max_z = t0s.z().max(t1s.z());

        // Calculate overall t_min and t_max
        let t_min = t_min_x.max(t_min_y).max(t_min_z);
        let t_max = t_max_x.min(t_max_y).min(t_max_z);

        // Check if the intersection times are valid
        if t_max < t_min || t_max < ray_t.min || t_min > ray_t.max{
            return false;
        }

        // Set the intersection time to t_min if it's within range, otherwise use t_max
        rec.t = if t_min < ray_t.min { t_max } else { t_min };

        // Calculate the intersection point
        rec.p = r.at(rec.t);

        // Determine the outward normal based on the intersection point
        let outward_normal = if rec.p.x() >= max_bound.x() - 1e-8 {
            Vec3::new(1.0, 0.0, 0.0)
        } else if rec.p.x() <= min_bound.x() + 1e-8 {
            Vec3::new(-1.0, 0.0, 0.0)
        } else if rec.p.y() >= max_bound.y() - 1e-8 {
            Vec3::new(0.0, 1.0, 0.0)
        } else if rec.p.y() <= min_bound.y() + 1e-8 {
            Vec3::new(0.0, -1.0, 0.0)
        } else if rec.p.z() >= max_bound.z() - 1e-8 {
            Vec3::new(0.0, 0.0, 1.0)
        } else {
            Vec3::new(0.0, 0.0, -1.0)
        };

        // Set the face normal in the hit record
        rec.set_face_normal(r, outward_normal);

        true
    }
}