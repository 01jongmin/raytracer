use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera {
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(look_from: Vec3, look_at: Vec3, vup: Vec3, vfov: f64, 
               aspect_ratio: f64, aperture: f64, focus_dist: f64) -> Camera {

        let theta = vfov * std::f64::consts::PI/180.;
        let h = f64::tan(theta/2.);
        let viewport_height = 2. * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).unit_vector();
        let u = Vec3::cross(&vup, &w).unit_vector();
        let v = Vec3::cross(&w, &u);

        let origin = look_from;

        // Viewport dimension coordinate
        let horizontal = u * viewport_width * focus_dist;
        let vertical = v * viewport_height * focus_dist;
        let lower_left_corner = origin - horizontal/2. - vertical/2. - w * focus_dist;

        let lens_radius = aperture / 2.;

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            w,
            lens_radius,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = Vec3::random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x() + self.v * rd.y();

        Ray::new(
            self.origin + offset, 
            self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset,
        )
    }
}

