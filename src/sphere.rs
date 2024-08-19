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
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = (half_b * half_b) - (a * c);

        if discriminant >= 0.0 {
            let sqrtd = discriminant.sqrt();
            let root_a = ((-half_b) - sqrtd) / a;
            let root_b = ((-half_b) + sqrtd) / a;
            for root in [root_a, root_b].iter() {
                if *root < ray_t.max && *root > ray_t.min {
                    let p = ray.at(*root);
                    let normal = (p - self.center) / self.radius;
                    let front_face = ray.direction.dot(&normal) < 0.0;

                    return Some(HitRecord {
                        t: *root,
                        p,
                        normal: if front_face { normal } else { -normal },
                        front_face,
                        material: &self.material,
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
