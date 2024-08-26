use crate::aabb::Aabb;
use crate::hittables::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::utils::Interval;
use crate::vec3::Vec3;

pub struct Cube {
    center: Vec3,
    size: f64,
    material: Material,
    bbox: Aabb,
}

impl Cube {
    pub fn new(center: Vec3, size: f64, material: Material) -> Cube {
        let size = f64::max(0.0, size);
        let half_size = Vec3::new(size / 2.0, size / 2.0, size / 2.0);
        let bbox = Aabb::new_from_vec3(center - half_size, center + half_size);
        Cube {
            center,
            size,
            material,
            bbox,
        }
    }

    fn get_cube_uv(p: Vec3, half_size: f64) -> (f64, f64) {
        let (x, y, z) = (p.x(), p.y(), p.z());

        let u: f64;
        let v: f64;

        // Determine which face the point is on and compute UV coordinates
        if z > 0.0 {
            // Front face
            u = (x + half_size) / (2.0 * half_size);
            v = (y + half_size) / (2.0 * half_size);
        } else if z < 0.0 {
            // Back face
            u = (x + half_size) / (2.0 * half_size);
            v = (half_size - (y + half_size)) / (2.0 * half_size);
        } else if y > 0.0 {
            // Top face
            u = (x + half_size) / (2.0 * half_size);
            v = (half_size - (z + half_size)) / (2.0 * half_size);
        } else if y < 0.0 {
            // Bottom face
            u = (x + half_size) / (2.0 * half_size);
            v = (y + half_size) / (2.0 * half_size);
        } else if x > 0.0 {
            // Right face
            u = (half_size - (z + half_size)) / (2.0 * half_size);
            v = (y + half_size) / (2.0 * half_size);
        } else {
            // Left face
            u = (z + half_size) / (2.0 * half_size);
            v = (y + half_size) / (2.0 * half_size);
        }

        (u, v)
    }
}

impl Hittable for Cube {
    fn hit(&self, r: Ray, ray_t: Interval) -> Option<HitRecord> {
        // Half the size of the cube for calculations
        let half_size = self.size / 2.0;

        // Calculate the minimum and maximum bounds of the cube
        let min_bound = self.center - Vec3::new(half_size, half_size, half_size);
        let max_bound = self.center + Vec3::new(half_size, half_size, half_size);

        // Calculate the inverse of the ray direction
        let inv_d = Vec3::new(
            1.0 / r.direction.x(),
            1.0 / r.direction.y(),
            1.0 / r.direction.z(),
        );

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
        if t_max < t_min || t_max < ray_t.min || t_min > ray_t.max {
            return None;
        }

        // Set the intersection time to t_min if it's within range, otherwise use t_max
        let t = if t_min < ray_t.min { t_max } else { t_min };

        // Check if the valid t is within the ray interval
        if t < ray_t.min || t > ray_t.max {
            return None;
        }

        // Calculate the intersection point
        let p = r.at(t);

        // Determine the outward normal based on the intersection point
        let tolerance = 1e-8;
        let outward_normal = if (p.x() - max_bound.x()).abs() < tolerance {
            Vec3::new(1.0, 0.0, 0.0)
        } else if (p.x() - min_bound.x()).abs() < tolerance {
            Vec3::new(-1.0, 0.0, 0.0)
        } else if (p.y() - max_bound.y()).abs() < tolerance {
            Vec3::new(0.0, 1.0, 0.0)
        } else if (p.y() - min_bound.y()).abs() < tolerance {
            Vec3::new(0.0, -1.0, 0.0)
        } else if (p.z() - max_bound.z()).abs() < tolerance {
            Vec3::new(0.0, 0.0, 1.0)
        } else {
            Vec3::new(0.0, 0.0, -1.0)
        };

        let (u, v) = Cube::get_cube_uv(p, half_size);

        // Create the hit record
        let mut rec = HitRecord {
            p,
            normal: outward_normal,
            t,
            front_face: false,
            material: &self.material,
            u,
            v,
        };

        // Set the face normal in the hit record
        rec.set_face_normal(r, outward_normal);

        Some(rec)
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
