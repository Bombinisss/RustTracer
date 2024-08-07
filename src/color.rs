use std::fs::File;
use std::io::Write;
use crate::utils::Interval;
use crate::vec3::Vec3;

pub fn write_color(file: File, color: Vec3){
    let r: f64 = color.x();
    let g: f64 = color.y();
    let b: f64 = color.z();

    let intensity = Interval::new(0.0,0.999);
    let ir: i32 = (256.0 * intensity.clamp(r)) as i32;
    let ig: i32 = (256.0 * intensity.clamp(g)) as i32;
    let ib: i32 = (256.0 * intensity.clamp(b)) as i32;

    file.try_clone().expect("REASON").write_all(format!("{} {} {}\n", ir, ig, ib).as_bytes()).unwrap();
}