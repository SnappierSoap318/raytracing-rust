use crate::ray::Ray;
use crate::vec3::Vec3;
#[derive(Copy, Clone)]
pub struct Camera {
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
}

impl Camera {
    pub fn new(look_from: Vec3, look_at: Vec3, vup: Vec3, aspect_ratio: f64, v_fov: f64) -> Camera {
        let focal_length = 1.0;

        let theta = v_fov * std::f64::consts::PI / 180.0;
        let h = f64::tan(theta / 2.0);

        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).unit_vector();
        let u = (vup.cross(w)).unit_vector();
        let v = w.cross(u);

        let origin = look_from;

        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;
        
        Camera {
            origin: origin,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: lower_left_corner,
        }
    }

    pub fn get_ray(self, s: f64, t: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin,
        )
    }
}
