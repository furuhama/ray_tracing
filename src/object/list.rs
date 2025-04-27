use std::sync::Arc;

use crate::ray::Ray;
use crate::types::{HitRecord, Hittable};

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut hit_anything = None;

        for object in &self.objects {
            if let Some(hit_record) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit_record.t;
                hit_anything = Some(HitRecord {
                    point: hit_record.point,
                    normal: hit_record.normal,
                    material: Arc::clone(&hit_record.material),
                    t: hit_record.t,
                    front_face: hit_record.front_face,
                });
            }
        }

        hit_anything
    }
}
