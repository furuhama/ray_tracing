use crate::aabb::AABB;
use crate::ray::Ray;
use crate::vec3::{Color, Vec3};
use std::sync::Arc;

pub struct ScatterInfo {
    pub attenuation: Color,
    pub scattered: Ray,
}

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Arc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.direction().dot(&outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -1.0 * outward_normal
        };
    }
}

pub trait Material: Send + Sync + 'static {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<ScatterInfo>;
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;

    /// オブジェクトのバウンディングボックスを計算
    ///
    /// # Arguments
    ///
    /// * `time0` - 時間範囲の開始（モーションブラー用）
    /// * `time1` - 時間範囲の終了（モーションブラー用）
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB>;
}

// Arc<dyn Hittable>に対するHittableトレイトの実装
impl Hittable for Arc<dyn Hittable> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        (**self).hit(ray, t_min, t_max)
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        (**self).bounding_box(time0, time1)
    }
}

pub fn random_unit_vector() -> Vec3 {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    let a = rng.gen_range(0.0..2.0 * std::f64::consts::PI);
    let z = rng.gen_range(-1.0..1.0);
    let r = ((1.0_f64 - z * z) as f64).sqrt();

    Vec3::new(r * a.cos(), r * a.sin(), z)
}
