use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;
use rand::Rng;

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}
impl Properties for Material {
    fn scatter(self, ray: Ray, rec: HitRecord) -> (Option<Ray>, Vec3) {
        match self {
            Material::Lambertian(l) => l.scatter(ray, rec),
            Material::Metal(m) => m.scatter(ray, rec),
            Material::Dielectric(d) => d.scatter(ray, rec),
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
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Metal {
        Metal {
            albedo,
            fuzz: match fuzz < 1.0 {
                true => fuzz,
                false => 1.0,
            },
        }
    }

    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v - 2.0 * Vec3::dot(v, n) * n
    }
}

impl Properties for Metal {
    fn scatter(self, _ray: Ray, rec: HitRecord) -> (Option<Ray>, Vec3) {
        let reflect = Metal::reflect(_ray.dir().unit_vector(), rec.normal);
        let scattered = Ray::new(rec.p, reflect + self.fuzz * Vec3::rand_in_unit_sphere());
        let attenuation = self.albedo;
        (Some(scattered), attenuation)
    }
}

#[derive(Copy, Clone)]
pub struct Dielectric {
    ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Dielectric {
        Dielectric { ir }
    }

    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v - 2.0 * v.dot(n) * n
    }
    pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = ((-*uv).dot(*n)).min(1.0);

        let r_out_perp = etai_over_etat * (*uv + *n * cos_theta);
        let r_out_parallel = -1.0 * f64::sqrt(f64::abs(1.0 - r_out_perp.length_squared())) * *n;
        r_out_perp + r_out_parallel
    }
    pub fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Properties for Dielectric {
    fn scatter(self, r: Ray, rec: HitRecord) -> (Option<Ray>, Vec3) {
        let mut rng = rand::thread_rng();

        let attenuation = Vec3::new(1.0, 1.0, 1.0);

        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_dir = r.dir().unit_vector();

        let cos_theta = (-unit_dir).dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let dir: Vec3;
        let cannot_refract = (refraction_ratio * sin_theta) > 1.0;
        if cannot_refract || rng.gen_bool(Dielectric::reflectance(cos_theta, refraction_ratio)) {
            dir = Dielectric::reflect(unit_dir, rec.normal);
        } else {
            dir = Dielectric::refract(&unit_dir, &rec.normal, refraction_ratio);
        }

        let scattered = Ray::new(rec.p, dir);
        (Some(scattered), attenuation)
    }
}
