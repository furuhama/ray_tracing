mod camera;
mod image;
mod ray;
mod types;
mod vec3;

use rand::prelude::*;

mod material {
    pub mod dielectric;
    pub mod lambertian;
    pub mod metal;
}

mod object {
    pub mod list;
    pub mod sphere;
}

use material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal};
use object::{list::HittableList, sphere::Sphere};
use ray::Ray;
use std::sync::Arc;
use types::Hittable;
use vec3::{Color, Vec3};

fn ray_color(ray: &Ray, world: &impl Hittable, depth: i32) -> Color {
    // 反射回数が制限を超えた場合は黒を返す
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(ray, 0.001, f64::INFINITY) {
        if let Some(scatter) = rec.material.scatter(ray, &rec) {
            return scatter.attenuation * ray_color(&scatter.scattered, world, depth - 1);
        }
        return Color::new(0.0, 0.0, 0.0);
    }

    let unit_direction = ray.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

fn main() {
    // 画像の基本設定
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 100; // アンチエイリアシング用のサンプル数

    // レンダリングの設定
    let max_depth = 50; // 反射の最大回数

    // カメラの設定
    let camera = camera::Camera::new(aspect_ratio);

    // シーンの作成
    let mut world = HittableList::new();

    // マテリアルの設定
    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let material_left = Arc::new(Dielectric::new(1.5)); // ガラス（屈折率1.5）
    let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    // オブジェクトの追加
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    ))); // 地面
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    ))); // 中央の球
    world.add(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left.clone(),
    ))); // 外側のガラス球
    world.add(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        -0.45, // 負の半径で内側の球を作成
        material_left,
    ))); // 内側のガラス球
    world.add(Box::new(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    ))); // 右の球

    // 画像の生成
    let mut pixels = Vec::with_capacity((image_width * image_height) as usize);

    for j in (0..image_height).rev() {
        println!("Remaining scanlines: {}", j);
        for i in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            let mut rng = rand::thread_rng();

            // 各ピクセルに対して複数回サンプリング
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rng.gen_range(0.0..1.0)) / (image_width - 1) as f64;
                let v = (j as f64 + rng.gen_range(0.0..1.0)) / (image_height - 1) as f64;

                let ray = camera.get_ray(u, v);
                pixel_color = pixel_color + ray_color(&ray, &world, max_depth);
            }

            // サンプリング結果の平均を取る
            pixel_color = pixel_color * (1.0 / samples_per_pixel as f64);
            pixels.push(pixel_color);
        }
    }

    println!("Writing image to file...");
    if let Err(e) = image::write_ppm("output.ppm", image_width, image_height, &pixels) {
        eprintln!("Error writing image: {}", e);
    }
    println!("Done!");
}
