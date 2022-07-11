use raytracer::camera::Camera;
use raytracer::material::{ Lambertian, Metal, Dielectric };
use raytracer::vec3::Vec3;
use raytracer::{ raytrace, random_scene };

fn main() -> std::io::Result<()> {
    // width over height
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 900;
    let image_height = (image_width as f64 / aspect_ratio) as usize;
    let samples_per_pixel = 10;
    let max_depth = 10;

    let world = random_scene(10);

    let lookfrom = Vec3::new(13., 2., 3.);
    let lookat = Vec3::new(0., 0., 0.);
    let vup = Vec3::new(0., 1., 0.);

    let dist_to_focus = 10.;
    let aperture = 0.1;
    let camera = Camera::new(lookfrom, lookat, vup, 20., aspect_ratio, aperture, dist_to_focus);

    raytrace("closer.png", image_width, image_height, samples_per_pixel, max_depth, &world, &camera);

    Ok(())
}
