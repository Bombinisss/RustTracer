use crate::ray::Ray;
use crate::utils::Interval;
use crate::vec3::Vec3;
#[derive(Debug, Clone, Copy)]
pub struct Aabb {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl Default for Aabb {
    fn default() -> Self {
        Aabb::new(
            Interval::default(),
            Interval::default(),
            Interval::default(),
        )
    }
}

impl Aabb {
    pub const fn new(x: Interval, y: Interval, z: Interval) -> Aabb {
        Aabb { x, y, z }
    }

    pub const EMPTY: Aabb = Aabb::new(Interval::EMPTY, Interval::EMPTY, Interval::EMPTY);

    pub const UNIVERSE: Aabb =
        Aabb::new(Interval::UNIVERSE, Interval::UNIVERSE, Interval::UNIVERSE);

    pub fn longest_axis(&self) -> i32 {
        // Returns the index of the longest axis of the bounding box.

        if self.x.size() > self.y.size() {
            if self.x.size() > self.z.size() {
                0
            } else {
                2
            }
        } else {
            if self.y.size() > self.z.size() {
                1
            } else {
                2
            }
        }
    }

    pub fn new_from_aabb(box0: Aabb, box1: Aabb) -> Aabb {
        let x = Interval::new_from_interval(&box0.x, &box1.x);
        let y = Interval::new_from_interval(&box0.y, &box1.y);
        let z = Interval::new_from_interval(&box0.z, &box1.z);
        Aabb { x, y, z }
    }

    pub fn new_from_vec3(a: Vec3, b: Vec3) -> Aabb {
        let x = if a[0] <= b[0] {
            Interval::new(a[0], b[0])
        } else {
            Interval::new(b[0], a[0])
        };

        let y = if a[1] <= b[1] {
            Interval::new(a[1], b[1])
        } else {
            Interval::new(b[1], a[1])
        };

        let z = if a[2] <= b[2] {
            Interval::new(a[2], b[2])
        } else {
            Interval::new(b[2], a[2])
        };

        Aabb { x, y, z }
    }

    pub fn axis_interval(&self, n: i32) -> Interval {
        if n == 1 {
            return self.y;
        }
        if n == 2 {
            return self.z;
        }
        self.x
    }

    pub fn hit(&self, r: Ray, mut ray_t: Interval) -> bool {
        let ray_origin = r.origin;
        let ray_direction = r.direction;

        for index in 0..3 {
            let axis = index as usize;
            let ax = self.axis_interval(index);
            let adinv = 1.0 / ray_direction[axis];

            let t0 = (ax.min - ray_origin[axis]) * adinv;
            let t1 = (ax.max - ray_origin[axis]) * adinv;

            if t0 < t1 {
                if t0 > ray_t.min {
                    ray_t.min = t0;
                }
                if t1 < ray_t.max {
                    ray_t.max = t1;
                }
            } else {
                if t1 > ray_t.min {
                    ray_t.min = t1;
                }
                if t0 < ray_t.max {
                    ray_t.max = t0;
                }
            }
            if ray_t.max <= ray_t.min {
                return false;
            }
        }

        true
    }
}
