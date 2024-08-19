use crate::color::linear_to_gamma;
use crate::hittables::{Hittable};
use crate::material::Scatterable;
use crate::ray::Ray;
use crate::utils::{degrees_to_radians, random_double, Interval};
use crate::vec3::Vec3;
use rayon::prelude::*;
use std::fs::File;
use std::io;
use std::io::Write;
use std::sync::Arc;

pub struct Camera {
    image_width: f64,
    image_height: f64,
    camera_center: Vec3,
    pixel00_loc: Vec3,   // Location of pixel 0, 0
    pixel_delta_u: Vec3, // Offset to pixel to the right
    pixel_delta_v: Vec3, // Offset to pixel below
    file: File,
    samples_per_pixel: i32,   // Count of random samples for each pixel
    pixel_samples_scale: f64, // Color scale factor for a sum of pixel samples
    max_depth: i32,           // Maximum number of ray bounces into scene
    defocus_angle: f64,       // Variation angle of rays through each pixel
    defocus_disk_u: Vec3,     // Defocus disk horizontal radius
    defocus_disk_v: Vec3,     // Defocus disk vertical radius
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
        (1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0)
    }

    pub fn render(&self, world: &dyn Hittable) {
        let image_width = self.image_width as usize;
        let image_height = self.image_height as usize;

        // Initialize a pixel buffer and box it to allocate on the heap.
        let mut pixels = vec![0; image_width * image_height * 3].into_boxed_slice();

        // Divide the pixel buffer into mutable chunks (bands), each corresponding to a row.
        let bands: Vec<(usize, &mut [u8])> =
            pixels.chunks_mut(image_width * 3).enumerate().collect();

        let progress = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let total_rows = image_height;

        // Parallel rendering of each row (band).
        bands.into_par_iter().for_each(|(j, band)| {
            for (i, pixel) in band.chunks_exact_mut(3).enumerate() {
                let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
                for _sample in 0..self.samples_per_pixel {
                    let r = self.get_ray(i as i32, j as i32);
                    pixel_color = pixel_color + Camera::ray_color(r, self.max_depth, world);
                }

                // Scale and gamma correct the color, then convert to bytes.
                let color = self.pixel_samples_scale * pixel_color;

                let mut r: f64 = color.x();
                let mut g: f64 = color.y();
                let mut b: f64 = color.z();

                // Apply a linear to gamma transform for gamma 2
                r = linear_to_gamma(r);
                g = linear_to_gamma(g);
                b = linear_to_gamma(b);

                // Translate the [0,1] component values to the byte range [0,255].
                let intensity = Interval::new(0.0, 0.999);
                let ir: u8 = (256.0 * intensity.clamp(r)) as u8;
                let ig: u8 = (256.0 * intensity.clamp(g)) as u8;
                let ib: u8 = (256.0 * intensity.clamp(b)) as u8;

                // Store the RGB values in the pixel buffer.
                pixel[0] = ir;
                pixel[1] = ig;
                pixel[2] = ib;
            }

            // Update progress
            let progress_count = progress.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            let percentage = (progress_count + 1) as f64 / total_rows as f64;
            let bar_width = 50; // Width of the progress bar
            let filled_length = (percentage * bar_width as f64).round() as usize;
            let bar = "=".repeat(filled_length) + &" ".repeat(bar_width - filled_length);
            print!("\rRendering: [{}] {:.2}%", bar, percentage * 100.0);
            io::stdout().flush().unwrap();
        });

        print!("\rRendering: [{}] 100.00%", "=".repeat(50));
        io::stdout().flush().unwrap();

        // Write the header to a file after rendering is complete.
        writeln!(
            &self.file.try_clone().expect("REASON"),
            "P3\n{} {}\n255",
            image_width,
            image_height
        )
        .expect("File header write failed!");

        // Write all pixel data to the file.
        for chunk in pixels.chunks(3) {
            writeln!(
                &self.file.try_clone().expect("REASON"),
                "{} {} {}",
                chunk[0],
                chunk[1],
                chunk[2]
            )
            .unwrap();
        }

        println!("\nDone");
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        // Construct a camera ray originating from the origin and directed at randomly sampled
        // point around the pixel location i, j.
        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.camera_center
        } else {
            self.defocus_disk_sample()
        };

        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn defocus_disk_sample(&self) -> Vec3 {
        // Returns a random point in the camera defocus disk.
        let p = Vec3::random_in_unit_disk();
        self.camera_center + (p.x() * self.defocus_disk_u) + (p.y() * self.defocus_disk_v)
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
        defocus_angle: f64,
        focus_dist: f64,
    ) -> Self {
        let file = File::create("test.ppm").unwrap();

        let mut image_height = image_width / aspect_ratio;
        if image_height < 1.0 {
            image_height = 1.0
        }

        // Determine viewport dimensions.
        let theta = degrees_to_radians(vertical_fov);
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * image_width / image_height;
        let camera_center = look_from;

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w = Vec3::unit_vector(look_from - look_at);
        let u = Vec3::unit_vector(Vec3::cross(&vup, &w));
        let v = Vec3::cross(&w, &u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * u; // Vector across viewport horizontal edge
        let viewport_v = viewport_height * -v; // Vector down viewport vertical edge

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / image_width;
        let pixel_delta_v = viewport_v / image_height;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            camera_center - (focus_dist * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + ((pixel_delta_u + pixel_delta_v) * 0.5);

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = focus_dist * f64::tan(degrees_to_radians(defocus_angle / 2.0));
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        let pixel_samples_scale = 1.0 / samples_per_pixel as f64;

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
            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
        }
    }
}
