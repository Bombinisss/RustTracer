use std::f64::consts::PI;
use crate::aabb::Aabb;
use crate::hittables::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::utils::Interval;
use crate::vec3::Vec3;

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

        let u = phi / (2.0*PI);
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
                    let (u,v) = Sphere::get_sphere_uv(outward_normal);
                    
                    return Some(HitRecord {
                        t: *root,
                        p,
                        normal: if front_face { outward_normal } else { -outward_normal },
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
