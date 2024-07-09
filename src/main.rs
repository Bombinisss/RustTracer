use std::fs::File;
use std::io::Write;

fn main() {

    let file = File::create("test.ppm").unwrap();

    let image_width = 256;
    let image_height = 256;

    file.try_clone().expect("REASON").write_all(format!("P3\n{} {}\n255\n", image_width, image_height).as_bytes()).expect("File header write failed!");

    for j in 0..image_height {
        for i in 0..image_width {
            let r: f64 = (i) as f64 / (image_width - 1) as f64;
            let g: f64 = (j) as f64 / (image_height - 1) as f64;
            let b: f64 = 0.0;

            let ir: i32 = (255.999 * r) as i32;
            let ig: i32 = (255.999 * g) as i32;
            let ib: i32 = (255.999 * b) as i32;

            file.try_clone().expect("REASON").write_all(format!("{} {} {}\n", ir, ig, ib).as_bytes()).expect(format!("Fail on j{} i{}", j, i).as_str());
        }
    }
}
