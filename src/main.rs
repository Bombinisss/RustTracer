mod vec3;
mod color;

use std::fs::File;
use std::io::Write;
use crate::color::write_color;
use crate::vec3::Vec3;

fn main() {

    let file = File::create("test.ppm").unwrap();

    let image_width = 256;
    let image_height = 256;

    file.try_clone().expect("REASON").write_all(format!("P3\n{} {}\n255\n", image_width, image_height).as_bytes()).expect("File header write failed!");

    for j in 0..image_height {
        println!("Scan lines remaining: {} ",image_height-j);
        for i in 0..image_width {
            let pixel_color: Vec3 = Vec3::new((i) as f64 / (image_width - 1) as f64,(j) as f64 / (image_height - 1) as f64,0.0);
            write_color(file.try_clone().unwrap(),pixel_color);
        }
    }
    print!("Done\n")
}
