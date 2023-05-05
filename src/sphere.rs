use crate::hittable::{Hittable, HitRecord};
use crate::material::Material;
use crate::vec3::Vec3;
use crate::ray::Ray;

#[derive(Clone, Copy)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub mat_ptr: Material,
}

impl Sphere{
    pub fn new(cen: Vec3, r:f64, m: Material) -> Sphere{
        Sphere {
            center: cen,
            radius: r,
            mat_ptr: m
        }
    }
}

impl Hittable for Sphere{
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.dir().length_squared();
        let half_b = oc.dot(r.dir());
        let c = oc.length_squared() - self.radius * self.radius;
    
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            
            root = (-half_b + sqrtd) / a;
            
            if root < t_min || t_max < root {
                return None;
            }
        }
        let t = root;
        let p = r.at(t);
        let outward_normal = (p - self.center) / self.radius;
        let (normal, front_face) = HitRecord::set_face_normal(r, outward_normal);

        let rec = HitRecord::new(p, normal, t, front_face, self.mat_ptr);

        return Some(rec);
    }
}
