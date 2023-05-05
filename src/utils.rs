use bvh::Vector3;
use rand::Rng;

pub fn deg_2_rad(deg: f64) -> f64 {
    deg * std::f64::consts::PI / 180.0
}
pub fn rand_vec() -> Vector3 {
    let mut rng = rand::thread_rng();
    Vector3 {
        x: rng.gen_range(0.0..1.0),
        y: rng.gen_range(0.0..1.0),
        z: rng.gen_range(0.0..1.0),
    }
}
pub fn rand_vec_range(min: f32, max: f32) -> Vector3 {
    let mut rng = rand::thread_rng();
    Vector3 {
        x: rng.gen_range(min..max),
        y: rng.gen_range(min..max),
        z: rng.gen_range(min..max),
    }
}

pub fn rand_in_unit_sphere() -> Vector3 {
    loop {
        let p = rand_vec_range(-1.0, 1.0);
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}
pub fn rand_unit_vec() -> Vector3 {
    return rand_in_unit_sphere().normalize();
}

pub fn near_zero(x: Vector3) -> bool {
    let s = 1e-8;
    ((x.x.abs() < s) && (x.y.abs() < s)) && (x.z.abs() < s)
}

pub fn rand_in_unit_disk() -> Vector3 {
    let mut rng = rand::thread_rng();
    loop {
        let p = Vector3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}
