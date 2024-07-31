mod vec3;
mod color;
mod ray;
mod sphere;
mod hittables;
mod utils;
mod cube;
mod camera;

use crate::camera::Camera;
use crate::cube::Cube;
use crate::hittables::{HittableList};
use crate::sphere::Sphere;
use crate::vec3::Vec3;

fn main() {
    /* World */
    let mut world = HittableList::new();

    world.add(Box::new(Sphere::new(Vec3::new(0.0,0.0,-1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0,-100.5,-1.0), 100.0)));

    world.add(Box::new(Cube::new(Vec3::new(-0.9,0.4,-1.0), 0.3)));
    world.add(Box::new(Cube::new(Vec3::new(0.9,-0.2,-1.0), 0.3)));

    /* Camera */
    let cam: Camera = Camera::new(16.0 / 9.0, 400.0);

    cam.render(&world);
}
