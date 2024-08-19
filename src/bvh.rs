use crate::aabb::Aabb;
use crate::hittables::{HitRecord, Hittable, HittableList};
use crate::ray::Ray;
use crate::utils::Interval;
use std::sync::Arc;

pub struct BvhNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: Aabb,
}

impl BvhNode {
    pub fn new_from_list(list: &HittableList) -> Self {
        BvhNode::new(&list.objects, 0, list.objects.len())
    }

    pub fn new(objects: &[Arc<dyn Hittable>], start: usize, end: usize) -> Self {
        // To be implemented later.
        unimplemented!()
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: Ray, ray_t: Interval) -> Option<HitRecord> {
        // Return None if the bounding box isn't hit
        if !self.bbox.hit(r, ray_t) {
            return None;
        }

        // Start with no hit
        let mut temp_rec = None;

        // Check if the left child is hit
        let hit_left = if let Some(left_rec) = self.left.hit(r, ray_t) {
            temp_rec = Some(left_rec);
            true
        } else {
            false
        };

        // Check if the right child is hit
        if let Some(right_rec) = self.right.hit(
            r,
            Interval::new(
                ray_t.min,
                if hit_left {
                    temp_rec.as_ref()?.t
                } else {
                    ray_t.max
                },
            ),
        ) {
            // Compare hits and keep the closer one
            if !hit_left || right_rec.t < temp_rec.as_ref()?.t {
                temp_rec = Some(right_rec);
            }
        };

        // Return the closest hit record, if any
        temp_rec
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
