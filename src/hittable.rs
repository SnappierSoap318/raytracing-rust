use crate::ray::Ray;
use crate::vec3::Vec3;
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: Vec3, normal: Vec3, t: f64, front_face: bool) -> HitRecord {
        HitRecord {
            p: p,
            normal: normal,
            t: t,
            front_face: front_face,
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

pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
