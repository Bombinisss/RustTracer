use crate::color::write_color;
use crate::hittables::{Hittable, HittableList};
use crate::material::Scatterable;
use crate::ray::Ray;
use crate::utils::{random_double, Interval, degrees_to_radians};
use crate::vec3::Vec3;
use std::fs::File;
use std::io::Write;

pub struct Camera {
    image_width: f64,
    image_height: f64,
    camera_center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    file: File,
    samples_per_pixel: i32,
    pixel_samples_scale: f64,
    max_depth: i32,
}

impl Camera {
    fn ray_color(r: Ray, depth: i32, world: &dyn Hittable) -> Vec3 {
        if depth <= 0 {
            return Vec3::new(0.0, 0.0, 0.0);
        }

        if let Some(hit) = world.hit(r, Interval::new(0.001, f64::INFINITY)) {
            let temp_rec = Some(hit).unwrap();

            if let Some(scat) = temp_rec.material.scatter(&r, &temp_rec) {
                return scat.1 * Self::ray_color(scat.0.unwrap(), depth - 1, world);
            }
            return Vec3::new(0.0, 0.0, 0.0);
        }

        let unit_direction = Vec3::unit_vector(r.direction);
        let a = 0.5 * (unit_direction.y() + 1.0);
        return (1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0);
    }

    pub fn render(&self, world: &HittableList) -> () {
        self.file
            .try_clone()
            .expect("REASON")
            .write_all(
                format!(
                    "P3\n{} {}\n255\n",
                    (self.image_width) as i32,
                    (self.image_height) as i32
                )
                .as_bytes(),
            )
            .expect("File header write failed!");

        for j in 0..(self.image_height) as i32 {
            println!("Scan lines remaining: {} ", (self.image_height) as i32 - j);
            for i in 0..(self.image_width) as i32 {
                let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
                for _sample in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color = pixel_color + Camera::ray_color(r, self.max_depth, world);
                }
                write_color(
                    self.file.try_clone().unwrap(),
                    self.pixel_samples_scale * pixel_color,
                );
            }
        }
        print!("Done\n")
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        // Construct a camera ray originating from the origin and directed at randomly sampled
        // point around the pixel location i, j.
        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);

        let ray_origin = self.camera_center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square(&self) -> Vec3 {
        // Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
        Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
    }

    pub fn new(
        aspect_ratio: f64,
        image_width: f64,
        samples_per_pixel: i32,
        max_depth: i32,
        vertical_fov: f64,
        look_from: Vec3,
        look_at: Vec3,
        vup: Vec3,
    ) -> Self {
        let file = File::create("test.ppm").unwrap();

        let mut image_height = image_width / aspect_ratio;
        if image_height < 1.0 {
            image_height = 1.0
        }

        // Determine viewport dimensions.
        let focal_length = (look_from - look_at).length();
        let theta = degrees_to_radians(vertical_fov);
        let h = f64::tan(theta/2.0);
        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = viewport_height * image_width / image_height;
        let camera_center = look_from;

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w = Vec3::unit_vector(look_from - look_at);
        let u = Vec3::unit_vector(Vec3::cross(&vup, &w));
        let v = Vec3::cross(&w, &u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * u;    // Vector across viewport horizontal edge
        let viewport_v = viewport_height * -v;  // Vector down viewport vertical edge

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / image_width;
        let pixel_delta_v = viewport_v / image_height;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = camera_center - (focal_length * w) - viewport_u/2.0 - viewport_v/2.0;
        let pixel00_loc = viewport_upper_left + ((pixel_delta_u + pixel_delta_v) * 0.5);
        let pixel_samples_scale = 1.0 / (samples_per_pixel) as f64;

        Self {
            image_width,
            image_height,
            camera_center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            file,
            samples_per_pixel,
            pixel_samples_scale,
            max_depth,
        }
    }
}
