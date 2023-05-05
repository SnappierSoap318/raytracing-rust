use crate::hittable::HitRecord;
use bvh::ray::Ray;
use crate::utils;
use bvh::Vector3;
use rand::Rng;

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}
impl Properties for Material {
    fn scatter(self, ray: &Ray, rec: HitRecord) -> (Option<Ray>, Vector3) {
        match self {
            Material::Lambertian(l) => l.scatter(ray, rec),
            Material::Metal(m) => m.scatter(ray, rec),
            Material::Dielectric(d) => d.scatter(ray, rec),
        }
    }
}

pub trait Properties {
    fn scatter(self, ray: &Ray, rec: HitRecord) -> (Option<Ray>, Vector3);
}

#[derive(Copy, Clone)]
pub struct Lambertian {
    albedo: Vector3,
}

impl Lambertian {
    pub fn new(_albedo: Vector3) -> Lambertian {
        Lambertian { albedo: _albedo }
    }
}

impl Properties for Lambertian {
    fn scatter(self, _ray: &Ray, rec: HitRecord) -> (Option<Ray>, Vector3) {
        let mut scatter_dir = rec.normal + utils::rand_unit_vec();

        if utils::near_zero(scatter_dir) {
            scatter_dir = rec.normal;
        }

        let scattered = Ray::new(rec.p, scatter_dir);
        let attenuation = self.albedo;
        (Some(scattered), attenuation)
    }
}

#[derive(Copy, Clone)]
pub struct Metal {
    albedo: Vector3,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vector3, fuzz: f32) -> Metal {
        Metal {
            albedo,
            fuzz: match fuzz < 1.0 {
                true => fuzz,
                false => 1.0,
            },
        }
    }

    pub fn reflect(v: Vector3, n: Vector3) -> Vector3 {
        v - 2.0 * Vector3::dot(v, n) * n
    }
}

impl Properties for Metal {
    fn scatter(self, _ray: &Ray, rec: HitRecord) -> (Option<Ray>, Vector3) {
        let reflect = Metal::reflect(_ray.direction.normalize(), rec.normal);
        let scattered = Ray::new(rec.p, reflect + self.fuzz * utils::rand_in_unit_sphere());
        let attenuation = self.albedo;
        (Some(scattered), attenuation)
    }
}

#[derive(Copy, Clone)]
pub struct Dielectric {
    ir: f32,
}

impl Dielectric {
    pub fn new(ir: f32) -> Dielectric {
        Dielectric { ir }
    }

    pub fn reflect(v: &Vector3, n: &Vector3) -> Vector3 {
        *v - 2.0 * v.dot(*n) * *n
    }
    pub fn refract(uv: &Vector3, n: &Vector3, etai_over_etat: f32) -> Vector3 {
        let cos_theta = ((-*uv).dot(*n)).min(1.0);

        let r_out_perp = etai_over_etat * (*uv + *n * cos_theta);
        let r_out_parallel = -1.0 * f32::sqrt(f32::abs(1.0 - r_out_perp.length_squared())) * *n;
        r_out_perp + r_out_parallel
    }
    pub fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Properties for Dielectric {
    fn scatter(self, r: &Ray, rec: HitRecord) -> (Option<Ray>, Vector3) {
        let mut rng = rand::thread_rng();

        let attenuation = Vector3::new(1.0, 1.0, 1.0);

        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_dir = r.direction.normalize();

        let cos_theta = (-unit_dir).dot(rec.normal).min(1.0); 
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let dir: Vector3;
        let cannot_refract = (refraction_ratio * sin_theta) > 1.0;
        if cannot_refract || rng.gen_bool(Dielectric::reflectance(cos_theta, refraction_ratio) as f64) {
            dir = Dielectric::reflect(&unit_dir, &rec.normal);
        } else {
            dir = Dielectric::refract(&unit_dir, &rec.normal, refraction_ratio);
        }

        let scattered = Ray::new(rec.p, dir);
        (Some(scattered), attenuation)
    }
}
