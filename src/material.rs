use crate::hittables::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub trait Scatterable {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Option<Ray>, Vec3)>;
}
#[derive(Clone)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}
impl Scatterable for Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Option<Ray>, Vec3)> {
        match self {
            Material::Lambertian(l) => l.scatter(r_in, rec),
            Material::Metal(m) => m.scatter(r_in, rec),
            Material::Dielectric(d) => d.scatter(r_in, rec),
        }
    }
}
#[derive(Clone)]
pub struct Lambertian {
    albedo: Vec3,
}
impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Lambertian { albedo }
    }
}
impl Scatterable for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Option<Ray>, Vec3)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new(rec.p, scatter_direction);
        let attenuation = self.albedo;
        Some((Some(scattered), attenuation))
    }
}

#[derive(Clone)]
pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Self {
        let mut fuzz = fuzz;
        if !(fuzz < 1.0) {
            fuzz = 1.0;
        }
        Metal { albedo, fuzz }
    }
}

impl Scatterable for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Option<Ray>, Vec3)> {
        let mut reflected = Vec3::reflect(r_in.direction, rec.normal);
        reflected = Vec3::unit_vector(reflected) + (self.fuzz * Vec3::random_unit_vector());
        let scattered = Ray::new(rec.p, reflected);
        let attenuation = self.albedo;

        Some((Some(scattered), attenuation))
    }
}

#[derive(Clone)]
pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Dielectric { refraction_index }
    }
}

impl Scatterable for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Option<Ray>, Vec3)> {
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = Vec3::unit_vector(r_in.direction);
        let cos_theta = (-unit_direction).dot(&rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = ri * sin_theta > 1.0;
        if cannot_refract {
            let direction = Vec3::reflect(unit_direction, rec.normal);
            let scattered = Ray::new(rec.p, direction);
            Some((Some(scattered), attenuation))
        } else {
            let direction = Vec3::refract(&unit_direction, &rec.normal, ri);
            let scattered = Ray::new(rec.p, direction);
            Some((Some(scattered), attenuation))
        }
    }
}
