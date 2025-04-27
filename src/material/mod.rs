pub mod dielectric;
pub mod lambertian;
pub mod metal;

pub use dielectric::Dielectric;
pub use lambertian::Lambertian;
pub use metal::Metal;

use crate::ray::Ray;
use crate::types::HitRecord;
use crate::vec3_glam::ColorGlam;

pub trait Material: Send + Sync {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(ColorGlam, Ray)>;
}
