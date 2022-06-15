use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;

pub struct World<'a> {
    objects: Vec<Box<dyn Hittable + Send + Sync + 'a>>,
}

impl<'a> World<'a>
{
    pub fn new() -> World<'a> {
        World {
            objects: vec![]
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Box<dyn Hittable + Send + Sync + 'a>) {
        self.objects.push(object);
    }

    pub fn did_hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut track_hit_record = None;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if let Some(hit_record) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit_record.t();
                track_hit_record = Some(hit_record);
            }
        }

        track_hit_record
    }

}
