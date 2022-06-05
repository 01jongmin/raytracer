use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::material::Material;
use std::rc::Rc;

pub struct HitRecord {
    point: Vec3,
    normal: Vec3,
    pub material: Rc<dyn Material>,
    t: f64,
    front_face: bool,
}

impl HitRecord {
    pub fn new(point: Vec3, normal: Vec3, material: Rc<dyn Material>, t: f64, front_face: bool) -> HitRecord {
        HitRecord {
            point,
            normal,
            material,
            t,
            front_face
        }
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    pub fn point(&self) -> Vec3 {
        self.point
    }
}
    

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
