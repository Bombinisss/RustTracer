use crate::hittables::HitRecord;
use crate::ray::Ray;
use crate::textures::{SolidColor, Texture};
use crate::utils::random_double;
use crate::vec3::Vec3;
use std::sync::Arc;

pub trait Scatterable {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Option<Ray>, Vec3)>;
    fn emitted(&self, _u: f64, _v: f64, _p: Vec3) -> Vec3;
}
#[derive(Clone)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
    DiffuseLight(DiffuseLight),
    Isotropic(Isotropic),
}
impl Scatterable for Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Option<Ray>, Vec3)> {
        match self {
            Material::Lambertian(l) => l.scatter(r_in, rec),
            Material::Metal(m) => m.scatter(r_in, rec),
            Material::Dielectric(d) => d.scatter(r_in, rec),
            Material::DiffuseLight(d) => d.scatter(r_in, rec),
            Material::Isotropic(i) => i.scatter(r_in, rec),
        }
    }

    fn emitted(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        match self {
            Material::Lambertian(l) => l.emitted(u, v, p),
            Material::Metal(m) => m.emitted(u, v, p),
            Material::Dielectric(d) => d.emitted(u, v, p),
            Material::DiffuseLight(d) => d.emitted(u, v, p),
            Material::Isotropic(i) => i.emitted(u, v, p),
        }
    }
}
#[derive(Clone)]
pub struct Lambertian {
    texture: Arc<dyn Texture>,
}
impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        let texture = Arc::new(SolidColor::new(albedo));
        Lambertian { texture }
    }

    pub fn new_from_texture(texture: Arc<dyn Texture>) -> Lambertian {
        Lambertian { texture }
    }
}
impl Scatterable for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Option<Ray>, Vec3)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new(rec.p, scatter_direction);
        let attenuation = self.texture.value(rec.u, rec.v, rec.p);
        Some((Some(scattered), attenuation))
    }

    fn emitted(&self, _u: f64, _v: f64, _p: Vec3) -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
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

    fn emitted(&self, _u: f64, _v: f64, _p: Vec3) -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
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

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
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
        if cannot_refract || Dielectric::reflectance(cos_theta, ri) > random_double() {
            let direction = Vec3::reflect(unit_direction, rec.normal);
            let scattered = Ray::new(rec.p, direction);
            Some((Some(scattered), attenuation))
        } else {
            let direction = Vec3::refract(&unit_direction, &rec.normal, ri);
            let scattered = Ray::new(rec.p, direction);
            Some((Some(scattered), attenuation))
        }
    }

    fn emitted(&self, _u: f64, _v: f64, _p: Vec3) -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }
}

#[derive(Clone)]
pub struct DiffuseLight {
    texture: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(albedo: Vec3) -> DiffuseLight {
        let texture = Arc::new(SolidColor::new(albedo));
        DiffuseLight { texture }
    }

    pub fn new_from_texture(texture: Arc<dyn Texture>) -> DiffuseLight {
        DiffuseLight { texture }
    }
}

impl Scatterable for DiffuseLight {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<(Option<Ray>, Vec3)> {
        None
    }

    fn emitted(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        self.texture.value(u, v, p)
    }
}

#[derive(Clone)]
pub struct Isotropic {
    texture: Arc<dyn Texture>,
}

impl Isotropic {
    pub fn new_with_color(albedo: Vec3) -> Self {
        Self {
            texture: Arc::new(SolidColor::new(albedo)),
        }
    }

    pub fn new_with_texture(texture: Arc<dyn Texture>) -> Self {
        Self { texture }
    }
}

impl Scatterable for Isotropic {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Option<Ray>, Vec3)> {
        let scattered = Ray::new(rec.p, Vec3::random_unit_vector());
        let attenuation = self.texture.value(rec.u, rec.v, rec.p);
        Some((Some(scattered), attenuation))
    }

    fn emitted(&self, _u: f64, _v: f64, _p: Vec3) -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }
}
