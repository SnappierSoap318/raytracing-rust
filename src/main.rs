pub mod camera;
pub mod colours;
pub mod hittable;
pub mod material;
pub mod ray;
pub mod sphere;
pub mod vec3;
pub mod world;

use image;
use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;
use rayon::prelude::*;

use crate::camera::Camera;
use crate::hittable::{hit_world, HitRecord};

use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::Vec3;

use material::Properties;

use Vec3 as color;

fn ray_colour(ray: Ray, world: Vec<Sphere>, depth: i32) -> Vec3 {
    let rec: Option<HitRecord> = hit_world(&world, ray, 0.001, std::f64::MAX);

    if depth <= 0 {
        return color::new(0.0, 0.0, 0.0);
    }
    match rec {
        Some(rec) => {
            let (scattered, attenuation) = rec.material.scatter(ray, rec);

            match scattered {
                Some(scattered) => attenuation * ray_colour(scattered, world, depth - 1),
                None => color::new(0.0, 0.0, 0.0),
            }
        }
        _ => {
            let unit_direction = ray.dir().unit_vector();
            let t = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * color::new(1.0, 1.0, 1.0) + t * color::new(0.5, 0.7, 1.0)
        }
    }
}

fn main() {
    println!("Raytracer Init!");
    // Gif specs
    // let frames: i32 = 60;

    // Image Specs
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 3840;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 128;
    let depth = 64;

    // Camera Specs
    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let focus_dist = 10.0;
    let aperture = 0.1;
    let v_fov = 20.0;

    print!("Aspect Ratio: {} \n", aspect_ratio);
    print!("Image Height: {} \n", image_height);
    print!("Image Width: {} \n", image_width);

    println!("Creating World");
    let world = world::make_world();
    println!("Creating World Complete!");

    let prog = ProgressBar::new(image_height as u64);
    prog.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] {msg} {bar:40.cyan/blue} {percent}% {pos:>7}/{len:7}",
        )
        .unwrap()
        .progress_chars("##-"),
    );
    prog.set_message("Scanlines Complete: ");

    // Camera
    let cam = Camera::new(
        look_from,
        look_at,
        vup,
        aspect_ratio,
        v_fov,
        aperture,
        focus_dist,
    );

    //Image buffer
    let mut imagebuf = image::ImageBuffer::new(image_width, image_height);

    let iter = imagebuf.enumerate_pixels_mut().into_iter().par_bridge();

    iter.into_par_iter().for_each(|f| {
        let i = f.0;
        let j = f.1;

        let mut pixel_color = color::new(0.0, 0.0, 0.0);
        let mut rng = rand::thread_rng();
        for _ in 0..samples_per_pixel {
            let u = (i as f64 + rng.gen_range(0.0..1.0)) / (image_width - 1) as f64;
            let v = (j as f64 + rng.gen_range(0.0..1.0)) / (image_height - 1) as f64;

            let r = cam.get_ray(u, v);
            pixel_color += ray_colour(r, world.clone(), depth);
        }
        let pixel = colours::write_colours(pixel_color, samples_per_pixel);
        *f.2 = image::Rgb(pixel);
        if i == j {
            prog.inc(1);
        }
    });
    println!("Render complete");
    // write image to file
    println!("Rotating 180");
    imagebuf = image::imageops::rotate180(&imagebuf);

    println!("Flipping Horizontally");
    imagebuf = image::imageops::flip_horizontal(&imagebuf);

    println!("Saving to Output.png");
    imagebuf.save(format!("output.png",)).unwrap();
}
