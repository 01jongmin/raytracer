mod ray;
mod vec3;
mod sphere;
mod hittable;
mod world;
mod utils;
mod camera;
mod material;

use vec3::Vec3;
use ray::Ray;
use world::World;
use sphere::Sphere;
use camera::Camera;
use material::{ Lambertian, Metal };
use utils::{ random_double };
use image;
use rayon::prelude::*;
use std::sync::Arc;

fn ray_color(ray: &Ray, world: &World, depth_limit: u64) -> Vec3 {
    if depth_limit <= 0 {
        return Vec3::constant_new(0.);
    }

    match world.did_hit(ray, 0.001, f64::INFINITY) {
        Some(hit_record) => {
            if let Some((scattered, attenuation)) = hit_record.material.scatter(ray, &hit_record) {
                ray_color(&scattered, world, depth_limit - 1) * attenuation 
            } else {
                Vec3::constant_new(0.)
            }
        }
        None => {
            // Linear interpolation from 0 - 1
            let t = (ray.direction().unit_vector().y() + 1.) * 0.5;

            // The more ray point upwards, the bluer the image
            Vec3::new(1.0, 1.0, 1.0) * (-t + 1.0) + Vec3::new(0.5, 0.7, 1.0) * t
        }
    }
}

fn main() -> std::io::Result<()> {
    // width over height
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    let material_ground = Lambertian{color: Vec3::new(0.8, 0.8, 0.0)};
    let material_center = Lambertian{color: Vec3::new(0.7, 0.3, 0.3)};
    let material_left   = Metal::new(Vec3::new(0.8, 0.8, 0.8), 0.3);
    let material_right  = Metal::new(Vec3::new(0.8, 0.6, 0.2), 1.0);

    let mut world = World::new();

    let center_sphere = Sphere::new(
        Vec3::new(0., 0., -1.),
        0.5,
        &material_center
    );

    let green_sphere = Sphere::new(
        Vec3::new(0., -100.5, -1.),
        100.,
        &material_ground
    );

    let left_sphere = Sphere::new(
        Vec3::new(-1., 0., -1.),
        0.5,
        &material_left,
    );

    let right_sphere = Sphere::new(
        Vec3::new(1., 0., -1.),
        0.5,
        &material_right,
    );

    world.add(Box::new(center_sphere));
    world.add(Box::new(green_sphere));
    world.add(Box::new(left_sphere));
    world.add(Box::new(right_sphere));

    let samples_per_pixel = 100;
    let max_depth: u64 = 100;
    let camera = Camera::new();
    let arc_world = Arc::new(world);

    let buffer = (0..image_width*image_height)
                    .into_par_iter()
                    .flat_map(|i| {
                        let col = i % image_width;
                        let row = i / image_width;

                        let pixel_color: Vec3 = (0..samples_per_pixel)
                                                .into_par_iter()
                                                .map(|_| {
                                                    let u = (col as f64 + random_double()) / (image_width - 1) as f64;
                                                    let v = (row as f64 + random_double()) / (image_height - 1) as f64;
                                                    //let mut pixel_color = Vec3::constant_new(0.);

                                                    let ray = camera.get_ray(u, v);
                                                    ray_color(&ray, &arc_world, max_depth)
                                                })
                                                .sum();

                        (pixel_color / samples_per_pixel as f64).rgb()
                    })
                    .collect::<Vec<u8>>();

    image::save_buffer("image.png", &buffer[..], image_width, image_height, image::ColorType::Rgb8).unwrap();
                    
    Ok(())
}
