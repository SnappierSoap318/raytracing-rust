use crate::vec3::Vec3;

pub fn write_colours(colour: Vec3) -> Vec3 {
    Vec3 {
        x: 255.999 * colour.x(),
        y: 255.999 * colour.y(),
        z: 255.999 * colour.z(),
    }
}
