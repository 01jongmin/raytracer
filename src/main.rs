use raytracer::world::World;
use raytracer::sphere::Sphere;
use raytracer::camera::Camera;
use raytracer::material::{ Lambertian, Metal };
use raytracer::vec3::Vec3;
use raytracer::raytrace;

fn main() -> std::io::Result<()> {
    // width over height
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as usize;

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

    let samples_per_pixel = 200;
    let max_depth = 100;
    let camera = Camera::new();

    raytrace("abc.png", image_width, image_height, samples_per_pixel, max_depth, &world, &camera);

    Ok(())
}
