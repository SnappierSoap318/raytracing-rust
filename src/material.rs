use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
}
impl Properties for Material {
    fn scatter(self, ray: Ray, rec: HitRecord) -> (Option<Ray>, Vec3) {
        match self {
            Material::Lambertian(l) => l.scatter(ray, rec),
            Material::Metal(l) => l.scatter(ray, rec),
        }
    }
}

pub trait Properties {
    fn scatter(self, ray: Ray, rec: HitRecord) -> (Option<Ray>, Vec3);
}

#[derive(Copy, Clone)]
pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(_albedo: Vec3) -> Lambertian {
        Lambertian { albedo: _albedo }
    }
}

impl Properties for Lambertian {
    fn scatter(self, _ray: Ray, rec: HitRecord) -> (Option<Ray>, Vec3) {
        let mut scatter_dir = rec.normal + Vec3::rand_unit_vec();

        if scatter_dir.near_zero() {
            scatter_dir = rec.normal;
        }

        let scattered = Ray::new(rec.p, scatter_dir);
        let attenuation = self.albedo;
        (Some(scattered), attenuation)
    }
}

#[derive(Copy, Clone)]
pub struct Metal {
    albedo: Vec3,
}

impl Metal {
    pub fn new(_albedo: Vec3) -> Metal {
        Metal { albedo: _albedo }
    }

    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v - 2.0 * Vec3::dot(v, n) * n
    }
}

impl Properties for Metal {
    fn scatter(self, _ray: Ray, rec: HitRecord) -> (Option<Ray>, Vec3) {
        let reflect = Metal::reflect(Vec3::unit_vector(_ray.dir()), rec.normal);
        let scattered = Ray::new(rec.p, reflect);
        let attenuation = self.albedo;
        (Some(scattered), attenuation)
    }
}
