use crate::vec3::Vec3;

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
    pub dir_inv: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, dir: Vec3) -> Ray {
        Ray {
            origin,
            dir,
            dir_inv: Vec3::new(1.0 / dir.x(), 1.0 / dir.y(), 1.0 / dir.z()),
        }
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
