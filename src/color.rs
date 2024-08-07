use std::fs::File;
use std::io::Write;
use crate::utils::Interval;
use crate::vec3::Vec3;

pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 { return f64::sqrt(linear_component); }
    return 0.0;
}
pub fn write_color(file: File, color: Vec3){
    let mut r: f64 = color.x();
    let mut g: f64 = color.y();
    let mut b: f64 = color.z();

    // Apply a linear to gamma transform for gamma 2
    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    // Translate the [0,1] component values to the byte range [0,255].
    let intensity = Interval::new(0.0,0.999);
    let ir: i32 = (256.0 * intensity.clamp(r)) as i32;
    let ig: i32 = (256.0 * intensity.clamp(g)) as i32;
    let ib: i32 = (256.0 * intensity.clamp(b)) as i32;

    file.try_clone().expect("REASON").write_all(format!("{} {} {}\n", ir, ig, ib).as_bytes()).unwrap();
}