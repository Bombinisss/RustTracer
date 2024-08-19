use crate::aabb::Aabb;
use crate::hittables::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::utils::Interval;
use crate::vec3::Vec3;
use crate::{cube, sphere};

pub enum Shapes {
    Sphere(sphere::Sphere),
    Cube(cube::Cube),
    Cuboid(Cuboid),
}

impl Hittable for Shapes {
    fn hit(&self, r: Ray, ray_t: Interval) -> Option<HitRecord> {
        match self {
            Shapes::Sphere(s) => s.hit(r, ray_t),
            Shapes::Cube(c) => c.hit(r, ray_t),
            Shapes::Cuboid(c) => c.hit(r, ray_t),
        }
    }

    fn bounding_box(&self) -> Aabb {
        match self {
            Shapes::Sphere(s) => s.bounding_box(),
            Shapes::Cube(c) => c.bounding_box(),
            Shapes::Cuboid(c) => c.bounding_box(),
        }
    }
}

pub struct Cuboid {
    center: Vec3,
    dimensions: Vec3, //(width, height, depth)
    material: Material,
    bbox: Aabb,
}

impl Cuboid {
    pub fn new(center: Vec3, dimensions: Vec3, material: Material) -> Cuboid {
        let dimensions = Vec3::new(
            f64::max(0.0, dimensions.x()),
            f64::max(0.0, dimensions.y()),
            f64::max(0.0, dimensions.z()),
        );
        let half_dimensions = dimensions / 2.0;
        let bbox = Aabb::new_from_vec3(center - half_dimensions, center + half_dimensions);
        Cuboid {
            center,
            dimensions,
            material,
            bbox,
        }
    }
}

impl Hittable for Cuboid {
    fn hit(&self, r: Ray, ray_t: Interval) -> Option<HitRecord> {
        // Half the dimensions of the cuboid for calculations
        let half_dimensions = self.dimensions / 2.0;

        // Calculate the minimum and maximum bounds of the cuboid
        let min_bound = self.center - half_dimensions;
        let max_bound = self.center + half_dimensions;

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
        let outward_normal = if (p.x() - max_bound.x()).abs() < f64::EPSILON {
            Vec3::new(1.0, 0.0, 0.0)
        } else if (p.x() - min_bound.x()).abs() < f64::EPSILON {
            Vec3::new(-1.0, 0.0, 0.0)
        } else if (p.y() - max_bound.y()).abs() < f64::EPSILON {
            Vec3::new(0.0, 1.0, 0.0)
        } else if (p.y() - min_bound.y()).abs() < f64::EPSILON {
            Vec3::new(0.0, -1.0, 0.0)
        } else if (p.z() - max_bound.z()).abs() < f64::EPSILON {
            Vec3::new(0.0, 0.0, 1.0)
        } else {
            Vec3::new(0.0, 0.0, -1.0)
        };

        let mut rec = HitRecord {
            p,
            normal: outward_normal,
            t,
            front_face: false,
            material: &self.material,
        };

        rec.set_face_normal(r, outward_normal);

        Some(rec)
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
