use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::vec3::Vec3;
use crate::utils::random_double;

pub trait Material: Sync {
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
    color: Vec3,
    fuzziness: f64,
}

impl Metal {
    pub fn new(color: Vec3, fuzziness: f64) -> Metal {
        let fuzziness = if fuzziness < 1. { fuzziness } else { 1. };
        Metal {
            color,
            fuzziness
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        let reflected_vector = Vec3::reflect(ray.direction().unit_vector(), hit_record.normal());
        let scattered = Ray::new(hit_record.point(), reflected_vector + Vec3::random_in_unit_sphere() * self.fuzziness); 

        // TODO: Why is there a > 0. if statement?
        if Vec3::dot(&scattered.direction(), &hit_record.normal()) > 0. {
            Some((scattered, self.color))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    // index of refraction
    ir: f64,
}
impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self {
            ir
        }
    }

    // Schlick's approximation
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let r0 = (1. - ref_idx) / (1. + ref_idx);
        let r0_squared = r0*r0;
        r0_squared + (1. - r0_squared) * f64::powi(1. - cosine, 5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        let refraction_ratio = if hit_record.front_face() { 1.0/self.ir } else { self.ir };
        let unit_direction = ray.direction().unit_vector();

        let cos_theta = f64::min(Vec3::dot(&(-unit_direction), &hit_record.normal()), 1.);
        let sin_theta = f64::sqrt(1. - cos_theta * cos_theta);

        let direction: Vec3;

        if refraction_ratio * sin_theta > 1. || Dielectric::reflectance(cos_theta, refraction_ratio) > random_double() { 
            direction = Vec3::reflect(unit_direction, hit_record.normal());
        } else {
            direction = Vec3::refract(unit_direction, hit_record.normal(), refraction_ratio);
        }

        let scattered_ray = Ray::new(hit_record.point(), direction); 

        Some((scattered_ray, Vec3::constant_new(1.0)))
    }
}















