use crate::aabb::Aabb;
use crate::material::Material;
use crate::ray::Ray;
use crate::utils::{degrees_to_radians, Interval};
use crate::vec3::Vec3;
use std::sync::Arc;

pub trait Hittable: Send + Sync {
    fn hit(&self, r: Ray, ray_t: Interval) -> Option<HitRecord>;
    fn bounding_box(&self) -> Aabb;
}

#[derive(Clone, Copy)]
pub struct HitRecord<'material> {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: &'material Material,
    pub u: f64,
    pub v: f64,
}

impl<'material> HitRecord<'material> {
    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) -> () {
        // Sets the hit record normal vector.
        // NOTE: the parameter `outward_normal` is assumed to have unit length.

        self.front_face = Vec3::dot(&r.direction, &outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }

    pub fn new(
        p: Vec3,
        normal: Vec3,
        t: f64,
        front_face: bool,
        material: &'material Material,
        u: f64,
        v: f64,
    ) -> Self {
        Self {
            p,
            normal,
            t,
            front_face,
            material,
            u,
            v,
        }
    }
}

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
    bbox: Aabb,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
            bbox: Default::default(),
        }
    }
    pub fn clear(&mut self) -> () {
        self.objects.clear();
    }
    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object.clone());
        self.bbox = Aabb::new_from_aabb(self.bbox, object.bounding_box());
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut temp_rec = None;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if let Some(hit) = object.hit(r, Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = hit.t;
                temp_rec = Some(hit);
            }
        }

        temp_rec
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}

pub struct Translate {
    object: Arc<dyn Hittable>,
    offset: Vec3,
    bbox: Aabb,
}

impl Translate {
    pub fn new(object: Arc<dyn Hittable>, offset: Vec3) -> Translate {
        let bbox = object.bounding_box() + offset;
        
        Translate {
            object,
            offset,
            bbox,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: Ray, ray_t: Interval) -> Option<HitRecord> {

        let offset_r = Ray::new(r.origin - self.offset, r.direction);

        let hit = self.object.hit(offset_r, ray_t);

        if hit.is_none() {
            return None;
        }

        let mut temp_rec = hit?;
        
        temp_rec.p = temp_rec.p + self.offset;

        Some(temp_rec)
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}

pub struct RotateY {
    object: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Aabb,
}

impl RotateY {
    pub fn new(object: Arc<dyn Hittable>, angle: f64) -> Self {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        
        let mut bbox = object.bounding_box();

        let mut min = Vec3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = Vec3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);
        
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.x.max + (1 - i) as f64 * bbox.x.min;
                    let y = j as f64 * bbox.y.max + (1 - j) as f64 * bbox.y.min;
                    let z = k as f64 * bbox.z.max + (1 - k) as f64 * bbox.z.min;

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(newx, y, newz);
                    
                    min = Vec3::new(
                        min.x().min(tester.x()),
                        min.y().min(tester.y()),
                        min.z().min(tester.z()),
                    );

                    max = Vec3::new(
                        max.x().max(tester.x()),
                        max.y().max(tester.y()),
                        max.z().max(tester.z()),
                    );
                }
            }
        }

        bbox = Aabb::new_from_vec3(min, max);

        RotateY {
            object,
            sin_theta,
            cos_theta,
            bbox,
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: Ray, ray_t: Interval) -> Option<HitRecord> {
        // Transform the ray from world space to object space.

        let origin = Vec3::new(
            (self.cos_theta * r.origin.x()) - (self.sin_theta * r.origin.z()),
            r.origin.y(),
            (self.sin_theta * r.origin.x()) + (self.cos_theta * r.origin.z())
        );

        let direction = Vec3::new(
            (self.cos_theta * r.direction.x()) - (self.sin_theta * r.direction.z()),
            r.direction.y(),
            (self.sin_theta * r.direction.x()) + (self.cos_theta * r.direction.z())
        );

        let rotated_r =  Ray::new(origin, direction);

        // Determine whether an intersection exists in object space (and if so, where).

        let hit = self.object.hit(rotated_r, ray_t);

        if hit.is_none() {
            return None;
        }
        
        let mut temp_rec = hit?;
        
        // Transform the intersection from object space back to world space.

        temp_rec.p = Vec3::new(
            (self.cos_theta * temp_rec.p.x()) + (self.sin_theta * temp_rec.p.z()),
            temp_rec.p.y(),
            (-self.sin_theta * temp_rec.p.x()) + (self.cos_theta * temp_rec.p.z())
        );

        temp_rec.normal = Vec3::new(
            (self.cos_theta * temp_rec.normal.x()) + (self.sin_theta * temp_rec.normal.z()),
            temp_rec.normal.y(),
            (-self.sin_theta * temp_rec.normal.x()) + (self.cos_theta * temp_rec.normal.z())
        );
        
        Some(temp_rec)
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
