use crate::material::Material;
use crate::ray::{Ray};
use crate::sphere::Sphere;
use crate::vec3::Vec3;

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Material
}

impl HitRecord {
    pub fn new(p: Vec3, normal: Vec3, t: f64, front_face: bool, mat: Material) -> HitRecord {
        HitRecord {
            p: p,
            normal: normal,
            t: t,
            front_face: front_face,
            material: mat
        }
    }

    pub fn set_face_normal(r: Ray, outward_normal: Vec3) -> (Vec3, bool) {
        let front_face = Vec3::dot(r.dir(), outward_normal) < 0.0;
        let normal = match front_face {
            true => outward_normal,
            false => -outward_normal,
        };
        return (normal, front_face);
    }
}

pub fn hit_world(world: &Vec<Sphere>, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>{
    let mut closest_so_far = t_max;
    let mut hit_record = None;
    for sphere in world {
        if let Some(hit) = sphere.hit(r, t_min, closest_so_far) {
            closest_so_far = hit.t;
            hit_record = Some(hit);
        }
    }
    hit_record
}

pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
