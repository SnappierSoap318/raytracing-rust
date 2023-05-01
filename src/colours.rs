use num;

use crate::vec3::Vec3;

pub fn write_colours(colour: Vec3, samples_per_pixel: i32) -> Vec3 {
    let mut r = colour.x();
    let mut g = colour.y();
    let mut b = colour.z();
    
    let scale = 1.0 / samples_per_pixel as f64;
    r =(r * scale).sqrt();
    g =(g * scale).sqrt();
    b =(b * scale).sqrt();
    Vec3 {
        x: 256.0 * num::clamp(r, 0.000, 1.0),
        y: 256.0 * num::clamp(g, 0.000, 1.0),
        z: 256.0 * num::clamp(b, 0.000, 1.0) 
    }
}
