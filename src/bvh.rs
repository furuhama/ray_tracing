use rand::Rng;
use std::sync::Arc;

use crate::aabb::AABB;
use crate::ray::Ray;
use crate::types::{HitRecord, Hittable};

/// BVH（Boundary Volume Hierarchy）のノード
pub struct BVHNode {
    left: Arc<dyn Hittable>,  // 左の子ノード
    right: Arc<dyn Hittable>, // 右の子ノード
    bounding_box: AABB,       // このノードのバウンディングボックス
}

impl BVHNode {
    /// オブジェクトのリストからBVHを構築
    pub fn new(mut objects: Vec<Arc<dyn Hittable>>, time0: f64, time1: f64) -> Self {
        // 軸をランダムに選択（x, y, z）
        let axis = rand::thread_rng().gen_range(0..3);

        let (left, right) = match objects.len() {
            1 => (objects[0].clone(), objects[0].clone()),
            2 => {
                // 2つのオブジェクトを軸に沿って比較して並べ替え
                let box_left = objects[0].bounding_box(time0, time1).unwrap();
                let box_right = objects[1].bounding_box(time0, time1).unwrap();

                if box_left.min().get(axis) < box_right.min().get(axis) {
                    (objects[0].clone(), objects[1].clone())
                } else {
                    (objects[1].clone(), objects[0].clone())
                }
            }
            _ => {
                // 選択した軸に沿ってオブジェクトをソート
                objects.sort_by(|a, b| {
                    let box_a = a.bounding_box(time0, time1).unwrap();
                    let box_b = b.bounding_box(time0, time1).unwrap();
                    box_a
                        .min()
                        .get(axis)
                        .partial_cmp(&box_b.min().get(axis))
                        .unwrap()
                });

                // オブジェクトを半分に分割
                let mid = objects.len() / 2;
                let right_objects = objects.split_off(mid);

                // 再帰的にBVHを構築
                (
                    Arc::new(BVHNode::new(objects, time0, time1)) as Arc<dyn Hittable>,
                    Arc::new(BVHNode::new(right_objects, time0, time1)) as Arc<dyn Hittable>,
                )
            }
        };

        // 子ノードのバウンディングボックスから、このノードのバウンディングボックスを計算
        let box_left = left.bounding_box(time0, time1).unwrap();
        let box_right = right.bounding_box(time0, time1).unwrap();
        let bounding_box = AABB::surrounding_box(&box_left, &box_right);

        BVHNode {
            left,
            right,
            bounding_box,
        }
    }
}

impl Hittable for BVHNode {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // まずバウンディングボックスとの交差判定
        if !self.bounding_box.hit(ray, t_min, t_max) {
            return None;
        }

        // 左の子ノードとの交差判定
        let hit_left = self.left.hit(ray, t_min, t_max);

        // 右の子ノードとの交差判定（左の子ノードとの交差点がある場合は、その手前までを探索）
        let t_max = if let Some(ref hit_rec) = hit_left {
            hit_rec.t
        } else {
            t_max
        };
        let hit_right = self.right.hit(ray, t_min, t_max);

        // より近い方の交差点を返す
        hit_right.or(hit_left)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        Some(self.bounding_box)
    }
}
