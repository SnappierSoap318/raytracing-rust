use crate::material::Material;
use bvh::ray::Ray;
use crate::sphere::Sphere;
use bvh::Vector3;


#[derive(Clone, Copy)]
pub struct HitRecord {
    pub p: Vector3,
    pub normal: Vector3,
    pub t: f32,
    pub front_face: bool,
    pub material: Material,
}

impl HitRecord {
    pub fn new(p: Vector3, normal: Vector3, t: f32, front_face: bool, material: Material) -> HitRecord {
        HitRecord {
            p,
            normal,
            t,
            front_face,
            material,
        }
    }

    pub fn set_face_normal(r: &Ray, outward_normal: Vector3) -> (Vector3, bool) {
        let front_face = Vector3::dot(r.direction, outward_normal) < 0.0;
        let normal = match front_face {
            true => outward_normal,
            false => -outward_normal,
        };
        return (normal, front_face);
    }
}

pub fn hit_world(world: Vec<&Sphere>, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
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
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}