use crate::ray::Ray;
use crate::vec3::Vec3;
use std::f64::consts::PI;

use rand::prelude::*;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,          // カメラ座標系のx軸
    v: Vec3,          // カメラ座標系のy軸
    w: Vec3,          // カメラ座標系のz軸
    lens_radius: f64, // レンズの半径
}

impl Camera {
    /// 新しいカメラを作成
    ///
    /// # Arguments
    ///
    /// * `lookfrom` - カメラの位置
    /// * `lookat` - 注視点
    /// * `vup` - 上方向ベクトル
    /// * `vfov` - 垂直方向の視野角（度）
    /// * `aspect_ratio` - アスペクト比
    /// * `aperture` - 絞り値（F値の逆数）
    /// * `focus_dist` - 焦点距離（オプション。Noneの場合は注視点までの距離を使用）
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: Option<f64>,
    ) -> Self {
        let theta = vfov * PI / 180.0;
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        // カメラの座標系を構築
        let w = (lookfrom - lookat).unit_vector(); // カメラのz軸（前方）
        let u = vup.cross(&w).unit_vector(); // カメラのx軸（右方向）
        let v = w.cross(&u); // カメラのy軸（上方向）

        let focus_distance = focus_dist.unwrap_or_else(|| (lookfrom - lookat).length());
        let origin = lookfrom;
        let horizontal = u * viewport_width * focus_distance;
        let vertical = v * viewport_height * focus_distance;
        let lower_left_corner = origin - horizontal * 0.5 - vertical * 0.5 - w * focus_distance;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius: aperture / 2.0,
        }
    }

    /// レンズ上のランダムな点を生成
    pub fn random_in_unit_disk() -> Vec3 {
        let mut rng = rand::thread_rng();
        loop {
            let p = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    /// 指定された位置からレイを生成
    ///
    /// # Arguments
    ///
    /// * `s` - 水平方向の位置（0.0 ~ 1.0）
    /// * `t` - 垂直方向の位置（0.0 ~ 1.0）
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = Self::random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x() + self.v * rd.y();

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset,
        )
    }

    /// F値から絞り値を計算
    ///
    /// # Arguments
    ///
    /// * `f_number` - F値（例：f/1.8なら1.8）
    pub fn aperture_from_f_number(f_number: f64) -> f64 {
        1.0 / f_number
    }
}
