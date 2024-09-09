mod aabb;
mod bvh;
mod camera;
mod color;
mod hittables;
mod image;
mod material;
mod ray;
mod shapes;
mod textures;
mod utils;
mod vec3;

use crate::bvh::BvhNode;
use crate::camera::Camera;
use crate::hittables::{HittableList, RotateY, Translate};
use crate::material::{Dielectric, DiffuseLight, Lambertian, Material, Metal};
use crate::shapes::{Cube, Cuboid, Quad, Sphere};
use crate::textures::{CheckerTexture, ImageTexture};
use crate::utils::{random_double, random_double_range, rotate_y_translation};
use crate::vec3::Vec3;
use std::sync::Arc;

fn spheres_and_cubes() {
    let mut world = HittableList::new();

    let checker = Material::Lambertian(Lambertian::new_from_texture(Arc::new(
        CheckerTexture::new_from_rgb(0.32, Vec3::new(0.2, 0.3, 0.1), Vec3::new(0.9, 0.9, 0.9)),
    )));

    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        checker,
    )));

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
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.55 {
                    // metal sphere
                    let albedo = Vec3::random_range(0.5, 1.0);
                    let fuzz = random_double_range(0.0, 0.5);
                    sphere_material = Material::Metal(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.65 {
                    // diffuse cube
                    let albedo = Vec3::random() * Vec3::random();
                    cube_material = Material::Lambertian(Lambertian::new(albedo));
                    world.add(Arc::new(Cube::new(center, 0.3, cube_material)));
                } else if choose_mat < 0.75 {
                    // metal cube
                    let albedo = Vec3::random_range(0.5, 1.0);
                    let fuzz = random_double_range(0.0, 0.5);
                    cube_material = Material::Metal(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Cube::new(center, 0.3, cube_material)));
                } else if choose_mat < 0.85 {
                    // glass sphere
                    let refraction_index = random_double_range(0.0, 2.0);
                    let sphere_material = Material::Dielectric(Dielectric::new(refraction_index));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Material::Dielectric(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let earth_texture = Arc::new(ImageTexture::new("earthmap.jpg"));
    let earth_surface = Material::Lambertian(Lambertian::new_from_texture(earth_texture));
    world.add(Arc::new(Sphere::new(
        Vec3::new(-5.0, 1.0, 0.0),
        2.0,
        earth_surface,
    )));

    //let material3 = Material::Metal(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.01));
    let moon_texture = Arc::new(ImageTexture::new("moon.png"));
    let moon_surface = Material::Lambertian(Lambertian::new_from_texture(moon_texture));
    world.add(Arc::new(Cuboid::new(
        Vec3::new(4.5, 1.0, -0.3),
        Vec3::new(4.0, 2.0, 1.0),
        moon_surface,
    )));

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
        "out1.ppm",
        Vec3::new(0.70, 0.80, 1.00),
    );

    cam.render(&bvh_node);
}

fn checkered_spheres() {
    let mut world = HittableList::new();

    let checker = Material::Lambertian(Lambertian::new_from_texture(Arc::new(
        CheckerTexture::new_from_rgb(0.32, Vec3::new(0.2, 0.3, 0.1), Vec3::new(0.9, 0.9, 0.9)),
    )));

    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -10.0, 0.0),
        10.0,
        checker.clone(),
    )));

    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 10.0, 0.0),
        10.0,
        checker,
    )));

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
        "out2.ppm",
        Vec3::new(0.70, 0.80, 1.00),
    );

    cam.render(&bvh_node);
}

fn earth() {
    let mut world = HittableList::new();

    let earth_texture = Arc::new(ImageTexture::new("earthmap.jpg"));
    let earth_surface = Material::Lambertian(Lambertian::new_from_texture(earth_texture));

    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 0.0, 0.0),
        2.0,
        earth_surface,
    )));

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
        "out3.ppm",
        Vec3::new(0.70, 0.80, 1.00),
    );

    cam.render(&bvh_node);
}

fn quads() {
    let mut world = HittableList::new();

    let left_red = Material::Lambertian(Lambertian::new(Vec3::new(1.0, 0.2, 0.2)));
    let back_green = Material::Lambertian(Lambertian::new(Vec3::new(0.2, 1.0, 0.2)));
    let right_blue = Material::Lambertian(Lambertian::new(Vec3::new(0.2, 0.2, 1.0)));
    let upper_orange = Material::Lambertian(Lambertian::new(Vec3::new(1.0, 0.5, 0.0)));
    let lower_teal = Material::Lambertian(Lambertian::new(Vec3::new(0.2, 0.8, 0.8)));

    world.add(Arc::new(Quad::new(
        Vec3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        left_red,
    )));
    world.add(Arc::new(Quad::new(
        Vec3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        back_green,
    )));
    world.add(Arc::new(Quad::new(
        Vec3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        right_blue,
    )));
    world.add(Arc::new(Quad::new(
        Vec3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        upper_orange,
    )));
    world.add(Arc::new(Quad::new(
        Vec3::new(-2.0, -3.0, 5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        lower_teal,
    )));

    let bvh_node = BvhNode::new_from_list(&world);

    /* Camera */
    let cam: Camera = Camera::new(
        16.0 / 9.0,
        1200.0,
        500,
        50,
        80.0,
        Vec3::new(0.0, 0.0, 9.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.0,
        10.0,
        "out4.ppm",
        Vec3::new(0.70, 0.80, 1.00),
    );

    cam.render(&bvh_node);
}

fn light() {
    let mut world = HittableList::new();

    let checker = Material::Lambertian(Lambertian::new_from_texture(Arc::new(
        CheckerTexture::new_from_rgb(0.32, Vec3::new(0.2, 0.3, 0.1), Vec3::new(0.9, 0.9, 0.9)),
    )));

    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        checker,
    )));

    let red = Material::Lambertian(Lambertian::new(Vec3::new(1.0, 0.2, 0.2)));
    let light = Material::DiffuseLight(DiffuseLight::new(Vec3::new(5.0, 5.0, 5.0)));

    world.add(Arc::new(Sphere::new(Vec3::new(0.0, 2.0, 0.0), 2.0, red)));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 7.0, 0.0),
        2.0,
        light.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Vec3::new(3.0, 1.0, -2.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        light,
    )));
    let bvh_node = BvhNode::new_from_list(&world);

    /* Camera */
    let cam: Camera = Camera::new(
        16.0 / 9.0,
        1200.0,
        500,
        50,
        20.0,
        Vec3::new(26.0, 3.0, 6.0),
        Vec3::new(0.0, 2.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.0,
        10.0,
        "out5.ppm",
        Vec3::new(0.0, 0.0, 0.0),
    );

    cam.render(&bvh_node);
}

fn cornell_box() {
    let mut world = HittableList::new();

    let red = Material::Lambertian(Lambertian::new(Vec3::new(0.65, 0.05, 0.05)));
    let white = Material::Lambertian(Lambertian::new(Vec3::new(0.73, 0.73, 0.73)));
    let green = Material::Lambertian(Lambertian::new(Vec3::new(0.12, 0.45, 0.15)));
    let light = Material::DiffuseLight(DiffuseLight::new(Vec3::new(15.0, 15.0, 15.0)));

    world.add(Arc::new(Quad::new(
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green,
    )));
    world.add(Arc::new(Quad::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red,
    )));
    world.add(Arc::new(Quad::new(
        Vec3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 4.0),
        Vec3::new(0.0, 0.0, -105.0),
        light,
    )));
    world.add(Arc::new(Quad::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Vec3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Vec3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));
    world.add(Arc::new(Translate::new(
        Arc::new(RotateY::new(
            Arc::new(Cuboid::new(
                Vec3::new(212.5, 82.5, 147.5),
                Vec3::new(165.0, 165.0, 165.0),
                white.clone(),
            )),
            -18.0,
        )),
        Vec3::new(23.0, 0.0, -29.0),
    )));

    world.add(Arc::new(Translate::new(
        Arc::new(RotateY::new(
            Arc::new(Cuboid::new(
                Vec3::new(347.5, 165.0, 377.5),
                Vec3::new(165.0, 330.0, 165.0),
                white,
            )),
            15.0,
        )),
        Vec3::new(-70.0, 0.0, 40.0),
    )));

    let bvh_node = BvhNode::new_from_list(&world);

    /* Camera */
    let cam: Camera = Camera::new(
        1.0,
        1000.0,
        1000,
        50,
        40.0,
        Vec3::new(278.0, 278.0, -800.0),
        Vec3::new(278.0, 278.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.0,
        10.0,
        "out6.ppm",
        Vec3::new(0.0, 0.0, 0.0),
    );

    cam.render(&bvh_node);
}

fn final_scene() {
    let mut world = HittableList::new();

    let ground = Material::Lambertian(Lambertian::new(Vec3::new(0.48, 0.83, 0.53)));

    let boxes_per_side = 20;

    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_double_range(1.0, 101.0);
            let z1 = z0 + w;

            world.add(Arc::new(Cuboid::new_from_corners(
                Vec3::new(x0, y0, z0),
                Vec3::new(x1, y1, z1),
                ground.clone(),
            )));
        }
    }

    let light = Material::DiffuseLight(DiffuseLight::new(Vec3::new(7.0, 7.0, 7.0)));

    world.add(Arc::new(Quad::new(
        Vec3::new(123.0, 554.0, 147.0),
        Vec3::new(300.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 265.0),
        light,
    )));

    let glass = Material::Dielectric(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Vec3::new(260.0, 150.0, 45.0),
        50.0,
        glass.clone(),
    )));

    world.add(Arc::new(Sphere::new(
        Vec3::new(360.0, 150.0, 145.0),
        73.0,
        glass.clone(),
    )));

    world.add(Arc::new(Sphere::new(
        Vec3::new(360.0, 150.0, 145.0),
        70.0,
        Material::Lambertian(Lambertian::new(Vec3::new(0.1, 0.3, 0.8))),
    )));

    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 150.0, 145.0),
        50.0,
        Material::Metal(Metal::new(Vec3::new(0.8, 0.8, 0.9), 0.35)),
    )));

    let glass_sphere_size = 80.0;
    world.add(Arc::new(Sphere::new(
        Vec3::new(220.0, 280.0, 300.0),
        glass_sphere_size,
        glass.clone(),
    )));
    let moon_texture = Arc::new(ImageTexture::new("moon.png"));
    let moon_surface = Material::Lambertian(Lambertian::new_from_texture(moon_texture));
    let moon_cube_center = Vec3::new(220.0, 280.0, 300.0);
    let moon_cube = Arc::new(Cube::new(
        moon_cube_center,
        glass_sphere_size / 2.0,
        moon_surface,
    ));
    let rotation_angle = -75.0;
    world.add(Arc::new(Translate::new(
        Arc::new(RotateY::new(moon_cube, rotation_angle)),
        rotate_y_translation(moon_cube_center, rotation_angle),
    )));

    let earth_texture = Arc::new(ImageTexture::new("earthmap.jpg"));
    let earth_surface = Material::Lambertian(Lambertian::new_from_texture(earth_texture));
    world.add(Arc::new(Sphere::new(
        Vec3::new(400.0, 200.0, 400.0),
        100.0,
        earth_surface,
    )));

    let white = Material::Lambertian(Lambertian::new(Vec3::new(0.73, 0.73, 0.73)));
    let sphere_count = 1000;
    let mut spheres = HittableList::new();
    for _i in 0..sphere_count {
        spheres.add(Arc::new(Sphere::new(
            Vec3::random_range(0.0, 165.0),
            10.0,
            white.clone(),
        )));
    }

    world.add(Arc::new(Translate::new(
        Arc::new(RotateY::new(
            Arc::new(BvhNode::new_from_list(&spheres)),
            15.0,
        )),
        Vec3::new(-100.0, 270.0, 395.0),
    )));

    let bvh_node = BvhNode::new_from_list(&world);

    /* Camera */
    let cam: Camera = Camera::new(
        1.0,
        1000.0,
        1000,
        40,
        40.0,
        Vec3::new(478.0, 278.0, -600.0),
        Vec3::new(278.0, 278.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.0,
        10.0,
        "out7.ppm",
        Vec3::new(0.0, 0.0, 0.0),
    );

    cam.render(&bvh_node);
}

fn main() {
    let num = 7;
    match num {
        1 => spheres_and_cubes(),
        2 => checkered_spheres(),
        3 => earth(),
        4 => quads(),
        5 => light(),
        6 => cornell_box(),
        7 => final_scene(),
        _ => final_scene(),
    }
}
