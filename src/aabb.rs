use crate::ray::Ray;
use crate::vec3::Vec3;

/// 軸並行境界ボックス（Axis-Aligned Bounding Box）
#[derive(Clone, Copy)]
pub struct AABB {
    minimum: Vec3, // ボックスの最小点
    maximum: Vec3, // ボックスの最大点
}

impl AABB {
    pub fn new(minimum: Vec3, maximum: Vec3) -> Self {
        AABB { minimum, maximum }
    }

    pub fn min(&self) -> Vec3 {
        self.minimum
    }

    pub fn max(&self) -> Vec3 {
        self.maximum
    }

    /// AABBとレイの交差判定
    ///
    /// スラブ法を使用して高速な交差判定を行う
    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
        // 各軸について交差判定
        for a in 0..3 {
            let inv_d = 1.0 / ray.direction().get(a);
            let mut t0 = (self.minimum.get(a) - ray.origin().get(a)) * inv_d;
            let mut t1 = (self.maximum.get(a) - ray.origin().get(a)) * inv_d;

            // レイの方向が負の場合、t0とt1を入れ替え
            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            // 交差区間の更新
            let t_min = t0.max(t_min);
            let t_max = t1.min(t_max);

            if t_max <= t_min {
                return false;
            }
        }
        true
    }

    /// 2つのAABBを含む最小のAABBを生成
    pub fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
        let small = Vec3::new(
            box0.min().x().min(box1.min().x()),
            box0.min().y().min(box1.min().y()),
            box0.min().z().min(box1.min().z()),
        );

        let big = Vec3::new(
            box0.max().x().max(box1.max().x()),
            box0.max().y().max(box1.max().y()),
            box0.max().z().max(box1.max().z()),
        );

        AABB::new(small, big)
    }
}
