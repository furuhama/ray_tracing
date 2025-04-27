use crate::ray::Ray;
use crate::types::{HitRecord, Material, ScatterInfo};
use crate::vec3::Color;
use rand::Rng;

#[derive(Clone)]
pub struct Dielectric {
    // 屈折率（Index of Refraction）
    ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Dielectric { ir }
    }

    // Schlickの近似を用いた反射率の計算
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<ScatterInfo> {
        let mut rng = rand::thread_rng();

        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = ray_in.direction().unit_vector();
        let cos_theta = (-unit_direction).dot(&rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract
            || Self::reflectance(cos_theta, refraction_ratio) > rng.gen_range(0.0..1.0)
        {
            // 全反射または確率的な反射
            unit_direction.reflect(&rec.normal)
        } else {
            // 屈折
            unit_direction.refract(&rec.normal, refraction_ratio)
        };

        Some(ScatterInfo {
            scattered: Ray::new(rec.point, direction),
            attenuation: Color::new(1.0, 1.0, 1.0),
        })
    }
}
