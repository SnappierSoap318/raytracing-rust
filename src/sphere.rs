use crate::hittable::{Hittable, HitRecord};
use crate::material::Material;
use bvh::ray::Ray;
use bvh::bounding_hierarchy::BHShape;
use bvh::aabb::{Bounded, AABB};
use bvh::{Vector3, Point3};


#[derive(Clone, Copy)]
pub struct Sphere {
    pub center: Vector3,
    pub radius: f32,
    pub mat_ptr: Material,
    pub node_index: usize,
}

impl Sphere{
    pub fn new(cen: Vector3, r:f32, m: Material) -> Sphere{
        Sphere {
            center: cen,
            radius: r,
            mat_ptr: m,
            node_index: 0,
        }
    }
}
impl Bounded for Sphere {
    fn aabb(&self) -> AABB {
        let half_size = Vector3::new(self.radius, self.radius, self.radius);
        let center = Vector3::new(self.center.x, self.center.y, self.center.z);
        let min = center - half_size;
        let max = center + half_size;
        AABB::with_bounds(min, max)
    }
}
impl BHShape for Sphere {
    fn set_bh_node_index(&mut self, index: usize) {
        self.node_index = index;
    }

    fn bh_node_index(&self) -> usize {
        self.node_index
    }
}

impl Hittable for Sphere{
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = oc.dot(r.direction);
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
        let p = at(&r, t);
        let outward_normal = (p - self.center) / self.radius;
        let (normal, front_face) = HitRecord::set_face_normal(r, outward_normal);

        let rec = HitRecord::new(p, normal, t, front_face, self.mat_ptr);

        return Some(rec);
    }
}

fn at(ray: &Ray, t: f32) -> Point3 {
    ray.origin + t * ray.direction
}
