use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::vec3::Vec3;

pub trait Material {
    // Returns scattered ray and color
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)>;
}

pub struct Lambertian {
    pub color: Vec3
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        let scatter_direction = hit_record.normal() + Vec3::random_unit_vector();
        Some((Ray::new(hit_record.point(), scatter_direction), self.color))
    }
}

pub struct Metal {
    pub color: Vec3
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        let reflected_vector = Vec3::reflect(ray.direction().unit_vector(), hit_record.normal());
        let scattered = Ray::new(hit_record.point(), reflected_vector);

        // TODO: Why is there a > 0. if statement?
        if Vec3::dot(&scattered.direction(), &hit_record.normal()) > 0. {
            Some((scattered, self.color))
        } else {
            None
        }
    }
}
