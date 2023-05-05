use crate::utils;
use bvh::ray::Ray;
use bvh::Vector3;


#[derive(Copy, Clone)]
pub struct Camera {
    origin: Vector3,
    horizontal: Vector3,
    vertical: Vector3,
    lower_left_corner: Vector3,
    u: Vector3,
    v: Vector3,
    _w: Vector3,
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        look_from: Vector3,
        look_at: Vector3,
        vup: Vector3,
        aspect_ratio: f32,
        v_fov: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Camera {
        let theta = v_fov * std::f32::consts::PI / 180.0;
        let h = f32::tan(theta / 2.0);

        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).normalize();
        let u = (vup.cross(w)).normalize();
        let v = w.cross(u);

        let origin = look_from;

        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

        let lens_radius = aperture / 2.0;
        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            _w: w,
            u,
            v,
            lens_radius,
        }
    }

    pub fn get_ray(self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * utils::rand_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}
