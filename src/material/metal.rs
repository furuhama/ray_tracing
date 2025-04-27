use crate::ray::Ray;
use crate::types::{HitRecord, Material, ScatterInfo, random_unit_vector};
use crate::vec3_glam::Vec3Glam;

type ColorGlam = Vec3Glam;

#[derive(Clone)]
pub struct Metal {
    base_color: ColorGlam, // 金属の基本色
    reflectivity: f64,     // 反射率（0.0 ~ 1.0）
    metallicness: f64,     // 金属性（0.0 ~ 1.0）
    roughness: f64,        // 表面の粗さ（0.0 ~ 1.0）
}

impl Metal {
    /// 新しいメタルマテリアルを作成
    ///
    /// # Arguments
    ///
    /// * `base_color` - 金属の基本色
    /// * `roughness` - 表面の粗さ（0.0 ~ 1.0）
    /// * `reflectivity` - 反射率（0.0 ~ 1.0、デフォルト0.9）
    /// * `metallicness` - 金属性（0.0 ~ 1.0、デフォルト1.0）
    pub fn new(base_color: ColorGlam, roughness: f64) -> Self {
        Self::with_params(base_color, roughness, 0.9, 1.0)
    }

    /// すべてのパラメータを指定してメタルマテリアルを作成
    pub fn with_params(
        base_color: ColorGlam,
        roughness: f64,
        reflectivity: f64,
        metallicness: f64,
    ) -> Self {
        Metal {
            base_color,
            reflectivity: reflectivity.clamp(0.0, 1.0),
            metallicness: metallicness.clamp(0.0, 1.0),
            roughness: roughness.clamp(0.0, 1.0),
        }
    }

    /// フレネル反射率の計算（Schlickの近似を使用）
    fn fresnel(&self, cos_theta: f64) -> f64 {
        let r0 = self.reflectivity;
        r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5)
    }

    /// 金属/非金属の混合色を計算
    fn mix_color(&self, pure_metal: ColorGlam) -> ColorGlam {
        let dielectric = ColorGlam::new(0.04, 0.04, 0.04); // 非金属の反射色
        dielectric * (1.0 - self.metallicness) + pure_metal * self.metallicness
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<ScatterInfo> {
        let unit_direction = ray_in.direction().unit_vector();

        // フレネル反射率の計算
        let cos_theta = (-unit_direction).dot(&rec.normal).min(1.0);
        let reflection_probability = self.fresnel(cos_theta);

        // 完全鏡面反射方向の計算
        let reflected = unit_direction.reflect(&rec.normal);

        // ラフネスに基づくランダムな方向のずれを追加
        let scattered_direction = if self.roughness > 0.0 {
            reflected + random_unit_vector() * self.roughness
        } else {
            reflected
        };

        // 反射方向が表面の裏側を向いている場合は光が吸収される
        if scattered_direction.dot(&rec.normal) > 0.0 {
            // 金属性に基づく色の計算
            let attenuation = self.mix_color(self.base_color);

            Some(ScatterInfo {
                scattered: Ray::new(rec.point, scattered_direction),
                attenuation: attenuation * reflection_probability,
            })
        } else {
            None
        }
    }
}
