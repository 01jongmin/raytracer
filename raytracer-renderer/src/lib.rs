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
use utils::{ random_double, random_double_range };
use material::{ Material, Lambertian, Metal, Dielectric };
use sphere::Sphere;
use hittable::Hittable;

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

pub fn raytrace_buffer(image_width: usize, image_height: usize, samples_per_pixel: usize, 
                       max_depth: usize, world: &World, camera: &Camera) -> Vec<u8> {
    (0..image_width*image_height)
        .into_par_iter()
        .flat_map(|i| {
            let (col, row)  = (i % image_width, i / image_width);

            let pixel_color: Vec3 = 
                (0..samples_per_pixel)
                    .into_par_iter()
                    .map(|_| {
                        let u = (col as f64 + random_double()) / (image_width - 1) as f64;
                        let v = 1. - (row as f64 + random_double()) / (image_height - 1) as f64;

                        let ray = camera.get_ray(u, v);
                        ray_color(&ray, &world, max_depth)
                    })
                    .sum();

            println!("{}, {}", col, row);

            (pixel_color / samples_per_pixel as f64).rgb()
        })
        .collect::<Vec<u8>>()
}

pub fn raytrace(name: &str, image_width: usize, image_height: usize, samples_per_pixel: usize, 
                max_depth: usize, world: &World, camera: &Camera) {
    let buffer = raytrace_buffer(image_width, image_height, samples_per_pixel, max_depth, world, camera);
    
    image::save_buffer(name, &buffer[..], image_width as u32, image_height as u32, image::ColorType::Rgb8).unwrap();
}

pub fn random_scene(number: isize) -> World<'static> {
    let mut world = World::new();

    let material_ground = Lambertian{color: Vec3::new(0.5, 0.5, 0.5)};

    let ground_sphere = Sphere::new(
        Vec3::new(0., -1000., 0.),
        1000.,
        Box::new(material_ground),
    );

    world.add(Box::new(ground_sphere));

    for a in -number..number {
        for b in -number..number {
            let material_selector = random_double();
            let center_point = Vec3::new(a as f64 + 0.9*random_double(), 0.2, b as f64 + 0.9*random_double());

            if (center_point - Vec3::new(4., 0.2, 0.)).length() > 0.9 {
                let material: Box<dyn Material>;

                if material_selector < 0.8 {
                    let color = Vec3::random() * Vec3::random();
                    material = Box::new(Lambertian { color });
                } else if material_selector < 0.95 {
                    let color = Vec3::random_range(0.5, 1.);
                    let fuzz = random_double_range(0., 0.5);
                    material = Box::new(Metal::new(color, fuzz));
                } else {
                    let sphere_material = Dielectric::new(1.5);
                    material = Box::new(sphere_material);
                }

                let generated_sphere = Box::new(Sphere::new(center_point, 0.2, material));
                world.add(generated_sphere);
            }
        }
    }

    let material1 = Box::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(Vec3::new(0., 1., 0.), 1., material1)));

    let material2 = Box::new(Lambertian { color: Vec3::new(0.4, 0.2, 0.1) });
    world.add(Box::new(Sphere::new(Vec3::new(-4., 1., 0.), 1., material2)));

    let material3 = Box::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(Vec3::new(4., 1., 0.), 1., material3)));

    world
}
