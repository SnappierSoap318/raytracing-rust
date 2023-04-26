mod colours;
mod hittable;
mod ray;
mod sphere;
pub mod utils;
mod vec3;

use image;
use indicatif::{ProgressBar, ProgressStyle};

use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::Vec3;

use Vec3 as color;

fn ray_colour(ray: Ray, world: &Sphere) -> Vec3 {
    let rec: Option<HitRecord> = world.hit(ray, 0.0, f64::INFINITY);

    match rec {
        Some(rec) => 0.5 * (rec.normal + color::new(1.0, 1.0, 1.0)),
        _ => {
            let unit_direction = ray.dir().unit_vector();
            let t = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * color::new(1.0, 1.0, 1.0) + t * color::new(0.5, 0.7, 1.0)
        }
    }
}

fn main() {
    println!("Raytracer Init!");

    // Image Specs
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1920 * 1;
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    print!("Aspect Ratio: {} \n", aspect_ratio);
    print!("Image Height: {} \n", image_height);
    print!("Image Width: {} \n", image_width);

    let prog = ProgressBar::new(image_height as u64);
    prog.set_style(
        ProgressStyle::with_template("[{elapsed_precise}] {msg} {bar:40.cyan/blue} {percent}%")
            .unwrap()
            .progress_chars("##-"),
    );
    prog.set_message("Scanlines Complete: ");

    // World
    let mut world = Vec::new();
    world.push(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
    world.push(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    //Image buffer
    let mut imagebuf = image::ImageBuffer::new(image_width, image_height);

    // Generate image data
    for j in 0..image_height {
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;

            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
            for object in &world {
                pixel_color = colours::write_colours(ray_colour(r, object.to_owned()));
            }

            imagebuf.put_pixel(
                i,
                j,
                image::Rgb([
                    pixel_color.x() as u8,
                    pixel_color.y() as u8,
                    pixel_color.z() as u8,
                ]),
            );
        }
        prog.inc(j as u64);
    }
    println!("Render complete");
    // write image to file
    println!("Rotating 180");
    imagebuf = image::imageops::rotate180(&imagebuf);

    println!("Flipping Horizontally");
    imagebuf = image::imageops::flip_horizontal(&imagebuf);

    println!("Saving to Output.png");
    imagebuf.save("output.png").unwrap();
}
