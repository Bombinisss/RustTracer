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
}
impl Scatterable for Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Option<Ray>, Vec3)> {
        match self {
            Material::Lambertian(l) => l.scatter(r_in, rec),
            Material::Metal(l) => l.scatter(r_in, rec),
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
}

impl Metal {
    pub fn new(albedo: Vec3) -> Self {
        Metal { albedo }
    }
}

impl Scatterable for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Option<Ray>, Vec3)> {
        let reflected = Vec3::reflect(r_in.direction, rec.normal);
        let scattered = Ray::new(rec.p, reflected);
        let attenuation = self.albedo;

        Some((Some(scattered), attenuation))
    }
}
