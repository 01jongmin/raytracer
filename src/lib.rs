pub mod vec3;
pub mod sphere;
pub mod world;
pub mod camera;
pub mod material;

mod ray;
mod hittable;
mod utils;

use ray::Ray;
use vec3::Vec3;
use world::World;
use camera::Camera;
use utils::{ random_double };

use image;
use rayon::prelude::*;

fn ray_color(ray: &Ray, world: &World, depth_limit: usize) -> Vec3 {
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

pub fn raytrace(name: &str, image_width: usize, image_height: usize, samples_per_pixel: usize, 
                max_depth: usize, world: &World, camera: &Camera) {
    let buffer = (0..image_width*image_height)
                    .into_par_iter()
                    .flat_map(|i| {
                        let col = i % image_width;
                        let row = i / image_width;

                        let pixel_color: Vec3 = 
                            (0..samples_per_pixel)
                                .into_par_iter()
                                .map(|_| {
                                    let u = (col as f64 + random_double()) / (image_width - 1) as f64;
                                    let v = 1. - (row as f64 + random_double()) / (image_height - 1) as f64;
                                    //let mut pixel_color = Vec3::constant_new(0.);

                                    let ray = camera.get_ray(u, v);
                                    ray_color(&ray, &world, max_depth)
                                })
                                .sum();

                        (pixel_color / samples_per_pixel as f64).rgb()
                    })
                    .collect::<Vec<u8>>();
    
    image::save_buffer(name, &buffer[..], image_width as u32, image_height as u32, image::ColorType::Rgb8).unwrap();
}
