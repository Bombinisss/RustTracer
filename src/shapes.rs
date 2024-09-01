use crate::aabb::Aabb;
use crate::hittables::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::utils::{map_uv_to_range, Interval};
use crate::vec3::Vec3;
use std::f64::consts::PI;

pub enum Shapes {
    Sphere(Sphere),
    Cube(Cube),
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

    fn get_cuboid_uv(p_relative_to_center: Vec3, dimensions: Vec3) -> (f64, f64) {
        // Define UV ranges for each face of the cuboid
        let top_uv_range = ((0.25, 0.666666), (0.5, 1.0));
        let bottom_uv_range = ((0.25, 0.0), (0.5, 0.333333));
        let left_uv_range = ((0.0, 0.333333), (0.25, 0.666666));
        let right_uv_range = ((0.5, 0.333333), (0.75, 0.666666));
        let front_uv_range = ((0.25, 0.333333), (0.5, 0.666666));
        let back_uv_range = ((0.75, 0.333333), (1.0, 0.666666));

        // Dimensions
        let half_width = dimensions.x() / 2.0;
        let half_height = dimensions.y() / 2.0;
        let half_depth = dimensions.z() / 2.0;

        // Tolerance to handle floating-point precision issues
        let tolerance = 1e-8;

        // Face mapping and UV calculation
        if (p_relative_to_center.x() - half_width).abs() < tolerance {
            // Right face
            let u = 1.0 - (p_relative_to_center.z() + half_depth) / dimensions.z();
            let v = (p_relative_to_center.y() + half_height) / dimensions.y();
            map_uv_to_range(u, v, &right_uv_range)
        } else if (p_relative_to_center.x() + half_width).abs() < tolerance {
            // Left face
            let u = (p_relative_to_center.z() + half_depth) / dimensions.z();
            let v = (p_relative_to_center.y() + half_height) / dimensions.y();
            map_uv_to_range(u, v, &left_uv_range)
        } else if (p_relative_to_center.y() - half_height).abs() < tolerance {
            // Top face
            let u = (p_relative_to_center.x() + half_width) / dimensions.x();
            let v = 1.0 - (p_relative_to_center.z() + half_depth) / dimensions.z();
            map_uv_to_range(u, v, &top_uv_range)
        } else if (p_relative_to_center.y() + half_height).abs() < tolerance {
            // Bottom face
            let u = (p_relative_to_center.x() + half_width) / dimensions.x();
            let v = (p_relative_to_center.z() + half_depth) / dimensions.z();
            map_uv_to_range(u, v, &bottom_uv_range)
        } else if (p_relative_to_center.z() - half_depth).abs() < tolerance {
            // Front face
            let u = (p_relative_to_center.x() + half_width) / dimensions.x();
            let v = (p_relative_to_center.y() + half_height) / dimensions.y();
            map_uv_to_range(u, v, &front_uv_range)
        } else if (p_relative_to_center.z() + half_depth).abs() < tolerance {
            // Back face
            let u = (p_relative_to_center.x() + half_width) / dimensions.x();
            let v = (p_relative_to_center.y() + half_height) / dimensions.y();
            map_uv_to_range(u, v, &back_uv_range)
        } else {
            (0.0, 0.0)
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

        let p_relative_to_center = p - self.center;
        let (u, v) = Cuboid::get_cuboid_uv(p_relative_to_center, self.dimensions);

        let mut rec = HitRecord {
            p,
            normal: outward_normal,
            t,
            front_face: false,
            material: &self.material,
            u,
            v,
        };

        rec.set_face_normal(r, outward_normal);

        Some(rec)
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}

pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Material,
    bbox: Aabb,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Material) -> Sphere {
        let radius = f64::max(0.0, radius);
        let radius_vec = Vec3::new(radius, radius, radius);
        let bbox = Aabb::new_from_vec3(center - radius_vec, center + radius_vec);
        Sphere {
            center,
            radius,
            material,
            bbox,
        }
    }

    fn get_sphere_uv(p: Vec3) -> (f64, f64) {
        // p: a given point on the sphere of radius one, centered at the origin.
        // u: returned value [0,1] of angle around the Y axis from X=-1.
        // v: returned value [0,1] of angle from Y=-1 to Y=+1.
        //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
        //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
        //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>

        let theta = f64::acos(-p.y());
        let phi = f64::atan2(-p.z(), p.x()) + PI;

        let u = phi / (2.0 * PI);
        let v = theta / PI;

        (u, v)
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = (half_b * half_b) - (a * c) + f64::EPSILON;

        if discriminant >= 0.0 {
            let sqrtd = discriminant.sqrt();
            let root_a = ((-half_b) - sqrtd) / a;
            let root_b = ((-half_b) + sqrtd) / a;
            for root in [root_a, root_b].iter() {
                if *root < ray_t.max && *root > ray_t.min {
                    let p = ray.at(*root);
                    let outward_normal = (p - self.center) / self.radius;
                    let front_face = ray.direction.dot(&outward_normal) < 0.0;
                    let (u, v) = Sphere::get_sphere_uv(outward_normal);

                    return Some(HitRecord {
                        t: *root,
                        p,
                        normal: if front_face {
                            outward_normal
                        } else {
                            -outward_normal
                        },
                        front_face,
                        material: &self.material,
                        u,
                        v,
                    });
                }
            }
        }
        None
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}

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

    fn get_cube_uv(p_relative_to_center: Vec3, half_size: f64) -> (f64, f64) {
        // Define UV ranges for each face
        let top_uv_range = ((0.25, 0.666666), (0.5, 1.0));
        let bottom_uv_range = ((0.25, 0.0), (0.5, 0.333333));
        let left_uv_range = ((0.0, 0.333333), (0.25, 0.666666));
        let right_uv_range = ((0.5, 0.333333), (0.75, 0.666666));
        let front_uv_range = ((0.25, 0.333333), (0.5, 0.666666));
        let back_uv_range = ((0.75, 0.333333), (1.0, 0.666666));

        // Tolerance to handle floating-point precision issues
        let tolerance = 1e-8;

        // Face mapping and UV calculation
        if (p_relative_to_center.x() - half_size).abs() < tolerance {
            // Right face
            let u = 1.0 - (p_relative_to_center.z() + half_size) / (2.0 * half_size);
            let v = (p_relative_to_center.y() + half_size) / (2.0 * half_size);
            map_uv_to_range(u, v, &right_uv_range)
        } else if (p_relative_to_center.x() + half_size).abs() < tolerance {
            // Left face
            let u = (p_relative_to_center.z() + half_size) / (2.0 * half_size);
            let v = (p_relative_to_center.y() + half_size) / (2.0 * half_size);
            map_uv_to_range(u, v, &left_uv_range)
        } else if (p_relative_to_center.y() - half_size).abs() < tolerance {
            // Top face
            let u = (p_relative_to_center.x() + half_size) / (2.0 * half_size);
            let v = 1.0 - (p_relative_to_center.z() + half_size) / (2.0 * half_size);
            map_uv_to_range(u, v, &top_uv_range)
        } else if (p_relative_to_center.y() + half_size).abs() < tolerance {
            // Bottom face
            let u = (p_relative_to_center.x() + half_size) / (2.0 * half_size);
            let v = (p_relative_to_center.z() + half_size) / (2.0 * half_size);
            map_uv_to_range(u, v, &bottom_uv_range)
        } else if (p_relative_to_center.z() - half_size).abs() < tolerance {
            // Front face
            let u = (p_relative_to_center.x() + half_size) / (2.0 * half_size);
            let v = (p_relative_to_center.y() + half_size) / (2.0 * half_size);
            map_uv_to_range(u, v, &front_uv_range)
        } else if (p_relative_to_center.z() + half_size).abs() < tolerance {
            // Back face
            let u = (p_relative_to_center.x() + half_size) / (2.0 * half_size);
            let v = (p_relative_to_center.y() + half_size) / (2.0 * half_size);
            map_uv_to_range(u, v, &back_uv_range)
        } else {
            (0.0, 0.0)
        }
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

        let p_relative_to_center = p - self.center;
        let (u, v) = Cube::get_cube_uv(p_relative_to_center, half_size);

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

pub struct Quad {
    q: Vec3,
    u: Vec3,
    v: Vec3,
    mat: Material,
    bbox: Aabb,
}

impl Quad {
    pub fn new(q: Vec3, u: Vec3, v: Vec3, mat: Material) -> Self {
        let mut quad = Quad {
            q,
            u,
            v,
            mat,
            bbox: Aabb::default(),
        };
        quad.set_bounding_box();
        quad
    }

    fn set_bounding_box(&mut self) {
        // Compute the bounding box of all four vertices.
        let bbox_diagonal1 = Aabb::new_from_vec3(self.q, self.q + self.u + self.v);
        let bbox_diagonal2 = Aabb::new_from_vec3(self.q + self.u, self.q + self.v);
        self.bbox = Aabb::new_from_aabb(bbox_diagonal1, bbox_diagonal2);
    }
}

impl Hittable for Quad {
    fn hit(&self, _r: Ray, _ray_t: Interval) -> Option<HitRecord> {
        None //TODO
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
