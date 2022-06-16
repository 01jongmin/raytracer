use crate::hittable::{ Hittable, HitRecord };
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::material::Material;

pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Box<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = Vec3::dot(&oc, &ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant >= 0. {
            let sqrt_discriminant = discriminant.sqrt();
            let root_neg = (-half_b - sqrt_discriminant) / a;
            let root_pos = (-half_b + sqrt_discriminant) / a;

            for root in [root_neg, root_pos].iter() {
                if *root < t_max && *root > t_min {
                    let point = ray.at(*root);
                    let normal = (point - self.center) / self.radius;
                    let front_face = Vec3::dot(&ray.direction(), &normal) < 0.0;

                    let normal = if front_face { normal } else { -normal };

                    return Some(HitRecord::new(
                        point,
                        normal,
                        &(*self.material),
                        *root,
                        front_face,
                    ));
                }
            }
        }

        None
    }
}
