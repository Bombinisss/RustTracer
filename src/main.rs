mod vec3;
mod color;
mod ray;

use std::fs::File;
use std::io::Write;
use crate::color::write_color;
use crate::ray::Ray;
use crate::vec3::Vec3;

fn ray_color(r: Ray) -> Vec3 {
    let unit_direction = Vec3::unit_vector(r.direction);
    let a = 0.5*(unit_direction.y() + 1.0);
    return (1.0-a)*Vec3::new(1.0, 1.0, 1.0) + a*Vec3::new(0.5, 0.7, 1.0);
}
fn main() {

    let file = File::create("test.ppm").unwrap();

    let image_width = 400;
    let aspect_ratio = 16.0 / 9.0;
    let mut image_height= ((image_width) as f64 /aspect_ratio) as i32;
    if image_height < 1 {image_height = 1}

    /* Camera */
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width/image_height) as f64;
    let camera_center = Vec3::new(0.0,0.0,0.0);

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / (image_width) as f64;
    let pixel_delta_v = viewport_v / (image_height) as f64;

    let viewport_upper_left = camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u/2.0 - viewport_v/2.0;

    let pixel00_loc = viewport_upper_left + ((pixel_delta_u + pixel_delta_v) * 0.5);

    /* Render */
    file.try_clone().expect("REASON").write_all(format!("P3\n{} {}\n255\n", image_width, image_height).as_bytes()).expect("File header write failed!");

    for j in 0..image_height {
        println!("Scan lines remaining: {} ",image_height-j);
        for i in 0..image_width {

            let pixel_center = pixel00_loc + (pixel_delta_u * (i) as f64) + (pixel_delta_v * (j) as f64);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(r);
            write_color(file.try_clone().unwrap(),pixel_color);
        }
    }
    print!("Done\n")
}
