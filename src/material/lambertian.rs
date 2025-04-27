use crate::ray::Ray;
use crate::types::{HitRecord, Material, ScatterInfo, random_unit_vector};
use crate::vec3_glam::Vec3Glam;

type ColorGlam = Vec3Glam;

#[derive(Clone)]
pub struct Lambertian {
    albedo: ColorGlam,
}

impl Lambertian {
    pub fn new(albedo: ColorGlam) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, rec: &HitRecord) -> Option<ScatterInfo> {
        let mut scatter_direction = rec.normal + random_unit_vector();

        // 散乱方向がゼロベクトルに近い場合は法線方向を使用
        if scatter_direction.length_squared() < 1e-8 {
            scatter_direction = rec.normal;
        }

        Some(ScatterInfo {
            scattered: Ray::new(rec.point, scatter_direction),
            attenuation: self.albedo,
        })
    }
}
