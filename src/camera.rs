use std::fs::File;
use std::io::Write;
use crate::color::write_color;
use crate::hittables::{HitRecord, Hittable, HittableList};
use crate::ray::Ray;
use crate::utils::Interval;
use crate::vec3::Vec3;

pub struct Camera {
    image_width: f64,
    image_height: f64,
    camera_center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    file: File
}

impl Camera {
    fn ray_color(r: Ray, world: &dyn Hittable) -> Vec3 {
        let mut rec = HitRecord::default();
        if world.hit(r, Interval::new(0.0, f64::INFINITY), &mut rec){
            return 0.5 * (rec.normal + Vec3::new(1.0,1.0,1.0))
        }

        let unit_direction = Vec3::unit_vector(r.direction);
        let a = 0.5 * (unit_direction.y() + 1.0);
        return (1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0);
    }

    pub fn render(&self, world: &HittableList) -> () {
        self.file.try_clone()
            .expect("REASON")
            .write_all(format!("P3\n{} {}\n255\n", (self.image_width) as i32, (self.image_height) as i32)
                .as_bytes()).expect("File header write failed!");

        for j in 0..(self.image_height) as i32 {
            println!("Scan lines remaining: {} ", (self.image_height) as i32 - j);
            for i in 0..(self.image_width) as i32 {
                let pixel_center = self.pixel00_loc + (self.pixel_delta_u * (i) as f64) + (self.pixel_delta_v * (j) as f64);
                let ray_direction = pixel_center - self.camera_center;
                let r = Ray::new(self.camera_center, ray_direction);

                let pixel_color = Camera::ray_color(r, world);
                write_color(self.file.try_clone().unwrap(), pixel_color);
            }
        }
        print!("Done\n")
    }

    pub fn new(aspect_ratio: f64, image_width: f64) -> Self {
        let file = File::create("test.ppm").unwrap();

        let mut image_height = image_width / aspect_ratio;
        if image_height < 1.0 { image_height = 1.0 }

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * image_width / image_height;
        let camera_center = Vec3::new(0.0, 0.0, 0.0);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / image_width;
        let pixel_delta_v = viewport_v / image_height;

        let viewport_upper_left = camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;

        let pixel00_loc = viewport_upper_left + ((pixel_delta_u + pixel_delta_v) * 0.5);

        Self { image_width, image_height, camera_center, pixel00_loc, pixel_delta_u, pixel_delta_v, file }
    }
}