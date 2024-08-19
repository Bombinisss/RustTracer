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
        BvhNode::new(list.objects.clone(), 0, list.objects.len())
    }

    pub fn new(objects: Vec<Arc<dyn Hittable>>, start: usize, end: usize) -> BvhNode {
        // Build the bounding box of the span of source objects
        let mut bbox = Aabb::EMPTY;
        for i in start..end {
            bbox = Aabb::new_from_aabb(bbox, objects[i].bounding_box());
        }

        // Determine the axis with the largest extent
        let axis = bbox.longest_axis();

        // Define comparators based on the axis
        let comparator: Box<dyn Fn(&Arc<dyn Hittable>, &Arc<dyn Hittable>) -> std::cmp::Ordering> = match axis {
            0 => Box::new(|a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>| {
                a.bounding_box().axis_interval(axis).min.partial_cmp(&b.bounding_box().axis_interval(axis).min).unwrap()
            }),
            1 => Box::new(|a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>| {
                a.bounding_box().axis_interval(axis).min.partial_cmp(&b.bounding_box().axis_interval(axis).min).unwrap()
            }),
            _ => Box::new(|a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>| {
                a.bounding_box().axis_interval(axis).min.partial_cmp(&b.bounding_box().axis_interval(axis).min).unwrap()
            }),
        };

        let object_span = end - start;

        let node = if object_span == 1 {
            BvhNode {
                left: objects[start].clone(),
                right: objects[start].clone(),
                bbox: objects[start].bounding_box(),
            }
        } else if object_span == 2 {
            BvhNode {
                left: objects[start].clone(),
                right: objects[start + 1].clone(),
                bbox: Aabb::new_from_aabb(
                    objects[start].bounding_box(),
                    objects[start + 1].bounding_box(),
                ),
            }
        } else {
            // Sort the objects based on the comparator
            let mut sorted_objects = objects;
            sorted_objects[start..end].sort_by(|a, b| comparator(a, b));

            let mid = start + object_span / 2;
            let left = Arc::new(BvhNode::new(sorted_objects.clone(), start, mid));
            let right = Arc::new(BvhNode::new(sorted_objects.clone(), mid, end));

            BvhNode {
                left: left.clone(),
                right: right.clone(),
                bbox: Aabb::new_from_aabb(left.bounding_box(), right.bounding_box()),
            }
        };

        node
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
