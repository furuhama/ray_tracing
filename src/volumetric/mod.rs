use crate::ray::Ray;
use crate::vec3_glam::ColorGlam;

pub mod fog;

/// ボリューメトリック効果の基本特性を定義するトレイト
pub trait VolumetricMedium: Send + Sync {
    /// 与えられたレイに対する透過率を計算
    ///
    /// # Arguments
    /// * `ray` - 光線
    /// * `t_min` - レイの開始距離
    /// * `t_max` - レイの終了距離
    ///
    /// # Returns
    /// * `(ColorGlam, f64)` - (散乱光の色, 透過率)
    fn sample(&self, ray: &Ray, t_min: f64, t_max: f64) -> (ColorGlam, f64);
}

/// Beer-Lambertの法則に基づく光の減衰を計算
pub fn calculate_beer_lambert_attenuation(density: f64, distance: f64) -> f64 {
    (-density * distance).exp()
}
