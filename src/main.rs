mod camera;
mod color;
mod cube;
mod hittables;
mod material;
mod ray;
mod sphere;
mod utils;
mod vec3;

use crate::camera::Camera;
use crate::cube::Cube;
use crate::hittables::HittableList;
use crate::material::{Lambertian, Metal, Dielectric};
use crate::sphere::Sphere;
use crate::vec3::Vec3;

fn main() {
    /* World */
    let mut world = HittableList::new();

    let material_ground = material::Material::Lambertian(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));
    let material_center = material::Material::Lambertian(Lambertian::new(Vec3::new(0.1, 0.2, 0.5)));
    let material_metal = material::Material::Metal(Metal::new(Vec3::new(0.8, 0.8, 0.8), 0.3));
    let material_dark_metal = material::Material::Metal(Metal::new(Vec3::new(0.8, 0.6, 0.2), 1.0));
    let glass = material::Material::Dielectric(Dielectric::new(1.50));

    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, material_ground)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.2), 0.5, material_center)));
    world.add(Box::new(Sphere::new(Vec3::new(-1.0,    0.0, -1.0), 0.5, glass)));
    world.add(Box::new(Sphere::new(Vec3::new( 1.0,    0.8, -1.0), 0.3, material_metal.clone())));

    world.add(Box::new(Cube::new(Vec3::new(-0.2, 0.8, -1.0), 0.3, material_metal)));
    world.add(Box::new(Cube::new(Vec3::new(0.9, -0.2, -1.0), 0.3, material_dark_metal)));

    /* Camera */
    let cam: Camera = Camera::new(16.0 / 9.0, 400.0, 100, 50);

    cam.render(&world);
}
