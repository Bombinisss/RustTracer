mod aabb;
mod bvh;
mod camera;
mod color;
mod cube;
mod hittables;
mod material;
mod ray;
mod shapes;
mod sphere;
mod textures;
mod utils;
mod vec3;
mod image;

use crate::bvh::BvhNode;
use crate::camera::Camera;
use crate::cube::Cube;
use crate::hittables::HittableList;
use crate::material::{Dielectric, Lambertian, Material, Metal};
use crate::shapes::Cuboid;
use crate::sphere::Sphere;
use crate::textures::{CheckerTexture, ImageTexture};
use crate::utils::{random_double, random_double_range};
use crate::vec3::Vec3;
use shapes::Shapes;
use std::sync::Arc;

fn spheres_and_cubes() {
    let mut world = HittableList::new();

    let checker = Material::Lambertian(Lambertian::new_from_texture(Arc::new(
        CheckerTexture::new_from_rgb(0.32, Vec3::new(0.2, 0.3, 0.1), Vec3::new(0.9, 0.9, 0.9)),
    )));

    world.add(Arc::new(Shapes::Sphere(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        checker,
    ))));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Vec3::new(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Material;
                let cube_material: Material;

                if choose_mat < 0.4 {
                    // diffuse sphere
                    let albedo = Vec3::random() * Vec3::random();
                    sphere_material = Material::Lambertian(Lambertian::new(albedo));
                    world.add(Arc::new(Shapes::Sphere(Sphere::new(
                        center,
                        0.2,
                        sphere_material,
                    ))));
                } else if choose_mat < 0.55 {
                    // metal sphere
                    let albedo = Vec3::random_range(0.5, 1.0);
                    let fuzz = random_double_range(0.0, 0.5);
                    sphere_material = Material::Metal(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Shapes::Sphere(Sphere::new(
                        center,
                        0.2,
                        sphere_material,
                    ))));
                } else if choose_mat < 0.65 {
                    // diffuse cube
                    let albedo = Vec3::random() * Vec3::random();
                    cube_material = Material::Lambertian(Lambertian::new(albedo));
                    world.add(Arc::new(Shapes::Cube(Cube::new(
                        center,
                        0.3,
                        cube_material,
                    ))));
                } else if choose_mat < 0.75 {
                    // metal cube
                    let albedo = Vec3::random_range(0.5, 1.0);
                    let fuzz = random_double_range(0.0, 0.5);
                    cube_material = Material::Metal(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Shapes::Cube(Cube::new(
                        center,
                        0.3,
                        cube_material,
                    ))));
                } else if choose_mat < 0.85 {
                    // glass sphere
                    let refraction_index = random_double_range(0.0, 2.0);
                    let sphere_material = Material::Dielectric(Dielectric::new(refraction_index));
                    world.add(Arc::new(Shapes::Sphere(Sphere::new(
                        center,
                        0.2,
                        sphere_material,
                    ))));
                }
            }
        }
    }

    let material1 = Material::Dielectric(Dielectric::new(1.5));
    world.add(Arc::new(Shapes::Sphere(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    ))));

    let earth_texture = Arc::new(ImageTexture::new("earthmap.jpg"));
    let earth_surface = Material::Lambertian(Lambertian::new_from_texture(earth_texture));
    world.add(Arc::new(Shapes::Sphere(Sphere::new(
        Vec3::new(-5.0, 1.0, 0.0),
        2.0,
        earth_surface,
    ))));

    let material3 = Material::Metal(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.01));
    world.add(Arc::new(Shapes::Cuboid(Cuboid::new(
        Vec3::new(4.0, 1.0, -0.3),
        Vec3::new(4.0, 2.0, 1.0),
        material3,
    ))));

    let bvh_node = BvhNode::new_from_list(&world);

    /* Camera */
    let cam: Camera = Camera::new(
        16.0 / 9.0,
        1200.0,
        500,
        20,
        20.0,
        Vec3::new(13.0, 2.0, 3.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.6,
        10.0,
    );

    cam.render(&bvh_node);
}

fn checkered_spheres() {
    let mut world = HittableList::new();

    let checker = Material::Lambertian(Lambertian::new_from_texture(Arc::new(
        CheckerTexture::new_from_rgb(0.32, Vec3::new(0.2, 0.3, 0.1), Vec3::new(0.9, 0.9, 0.9)),
    )));

    world.add(Arc::new(Shapes::Sphere(Sphere::new(
        Vec3::new(0.0, -10.0, 0.0),
        10.0,
        checker.clone(),
    ))));

    world.add(Arc::new(Shapes::Sphere(Sphere::new(
        Vec3::new(0.0, 10.0, 0.0),
        10.0,
        checker,
    ))));

    let bvh_node = BvhNode::new_from_list(&world);

    /* Camera */
    let cam: Camera = Camera::new(
        16.0 / 9.0,
        1200.0,
        500,
        50,
        20.0,
        Vec3::new(13.0, 2.0, 3.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.0,
        10.0,
    );

    cam.render(&bvh_node);
}

fn earth() {
    let mut world = HittableList::new();

    let earth_texture = Arc::new(ImageTexture::new("earthmap.jpg"));
    let earth_surface = Material::Lambertian(Lambertian::new_from_texture(earth_texture));

    world.add(Arc::new(Shapes::Sphere(Sphere::new(
        Vec3::new(0.0, 0.0, 0.0),
        2.0,
        earth_surface,
    ))));

    let bvh_node = BvhNode::new_from_list(&world);

    /* Camera */
    let cam: Camera = Camera::new(
        16.0 / 9.0,
        1200.0,
        500,
        50,
        20.0,
        Vec3::new(0.0, 0.0, 12.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.0,
        10.0,
    );

    cam.render(&bvh_node);
}

fn main() {
    let num = 1;
    match num {
        1 => spheres_and_cubes(),
        2 => checkered_spheres(),
        3 => earth(),
        _ => {}
    }
}
