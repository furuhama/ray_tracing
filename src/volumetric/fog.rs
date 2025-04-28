use super::{VolumetricMedium, calculate_beer_lambert_attenuation};
use crate::ray::Ray;
use crate::vec3_glam::ColorGlam;

/// 均一なフォグを表現する構造体
pub struct UniformFog {
    /// フォグの色
    color: ColorGlam,
    /// フォグの密度
    density: f64,
}

impl UniformFog {
    /// 新しいUniformFogインスタンスを作成
    pub fn new(color: ColorGlam, density: f64) -> Self {
        Self { color, density }
    }
}

impl VolumetricMedium for UniformFog {
    fn sample(&self, _ray: &Ray, t_min: f64, t_max: f64) -> (ColorGlam, f64) {
        let distance = t_max - t_min;

        // Beer-Lambertの法則に基づいて透過率を計算
        let transmittance = calculate_beer_lambert_attenuation(self.density, distance);

        // フォグによる散乱光を計算
        let scattered_light = self.color * (1.0 - transmittance);

        (scattered_light, transmittance)
    }
}
