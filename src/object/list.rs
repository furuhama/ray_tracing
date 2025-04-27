use std::sync::Arc;

use crate::aabb::AABB;
use crate::bvh::BVHNode;
use crate::ray::Ray;
use crate::types::{HitRecord, Hittable};

pub struct HittableList {
    objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }

    /// BVHを構築してシーンを最適化
    pub fn optimize(&self) -> Arc<dyn Hittable> {
        if self.objects.is_empty() {
            panic!("空のシーンは最適化できません");
        }

        // オブジェクトのクローンを作成
        let objects: Vec<Arc<dyn Hittable>> = self.objects.iter().map(Arc::clone).collect();

        Arc::new(BVHNode::new(objects, 0.0, 1.0))
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

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        if self.objects.is_empty() {
            return None;
        }

        let mut result = None;

        for object in &self.objects {
            if let Some(bbox) = object.bounding_box(time0, time1) {
                result = Some(if let Some(result_box) = result {
                    AABB::surrounding_box(&result_box, &bbox)
                } else {
                    bbox
                });
            } else {
                return None;
            }
        }

        result
    }
}
