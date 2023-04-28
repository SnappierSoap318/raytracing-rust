use crate::vec3::Vec3;

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, dir: Vec3) -> Ray{
        Ray { origin, dir }
    }

    pub fn origin(self) -> Vec3 {
        self.origin
    }

    pub fn dir(self) -> Vec3 {
        self.dir
    }

    pub fn at(self, t: f64) -> Vec3 {
        self.origin + t * self.dir
    }
}
