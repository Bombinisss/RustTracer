use crate::aabb::Aabb;
use crate::material::Material;
use crate::ray::Ray;
use crate::utils::Interval;
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
