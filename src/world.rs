use crate::material::{Dielectric, Lambertian, Material, Metal};
use crate::sphere::Sphere;
use crate::utils::{rand_vec, rand_vec_range};
use bvh::Vector3;
use rand::Rng;

pub fn make_world() -> Vec<Sphere> {
    let mut world = Vec::new();

    let mat_ground = Lambertian::new(Vector3::new(0.5, 0.5, 0.5));

    world.push(Sphere::new(
        Vector3::new(0.0, -1000.0, 0.0),
        1000.0,
        Material::Lambertian(mat_ground),
    ));

    let mut rng = rand::thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen_range(0.0..1.0);
            let center = Vector3::new(
                a as f32 + 0.9 * rng.gen_range(0.0..1.0),
                0.2,
                b as f32 + 0.9 * rng.gen_range(0.0..1.0),
            );

            if (center - Vector3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_mat: Material;

                match choose_mat {
                    x if x >= 0.0 && x <= 0.8 => {
                        let albedo = rand_vec() * rand_vec();
                        sphere_mat = Material::Lambertian(Lambertian::new(albedo));
                        world.push(Sphere::new(center, 0.2, sphere_mat));
                    }
                    x if x >= 0.81 && x <= 0.95 => {
                        let albedo = rand_vec_range(0.5, 1.0);
                        let fuzz = rng.gen_range(0.0..=0.5);
                        sphere_mat = Material::Metal(Metal::new(albedo, fuzz));
                        world.push(Sphere::new(center, 0.2, sphere_mat));
                    }
                    _ => {
                        sphere_mat = Material::Dielectric(Dielectric::new(1.5));
                        world.push(Sphere::new(center, 0.2, sphere_mat));
                    }
                }
            }
        }
    }

    let mat1: Material = Material::Dielectric(Dielectric::new(1.5));
    world.push(Sphere::new(Vector3::new(0.0, 1.0, 0.0), 1.0, mat1));

    let mat2: Material = Material::Lambertian(Lambertian::new(Vector3::new(0.4, 0.2, 0.1)));
    world.push(Sphere::new(Vector3::new(-4.0, 1.0, 0.0), 1.0, mat2));

    let mat3: Material = Material::Metal(Metal::new(Vector3::new(0.7, 0.6, 0.5), 0.0));
    world.push(Sphere::new(Vector3::new(4.0, 1.0, 0.0), 1.0, mat3));
    
    world
}

