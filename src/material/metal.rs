use crate::ray::Ray;
use crate::types::{HitRecord, Material, ScatterInfo, random_unit_vector};
use crate::vec3::Color;

#[derive(Clone)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Metal {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<ScatterInfo> {
        let reflected = ray_in.direction().unit_vector().reflect(&rec.normal);
        let scattered = Ray::new(rec.point, reflected + random_unit_vector() * self.fuzz);

        if scattered.direction().dot(&rec.normal) > 0.0 {
            Some(ScatterInfo {
                scattered,
                attenuation: self.albedo,
            })
        } else {
            None
        }
    }
}
