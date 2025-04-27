mod aabb;
mod bvh;
mod camera;
mod image;
mod ray;
mod types;
mod vec3;

use camera::Camera;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rand::prelude::*;
use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};

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
    let image_width = 800;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 200; // より多くのサンプル数で画質向上

    // レンダリングの設定
    let max_depth = 50; // 反射の最大回数

    // マテリアルの設定
    let material_ground = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5))); // グレーの地面
    let material_center = Arc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3))); // 赤みがかった拡散面
    let glass = Arc::new(Dielectric::new(1.5)); // ガラス（屈折率1.5）

    // 様々な金属マテリアルの作成
    let mirror = Arc::new(Metal::with_params(
        Color::new(0.95, 0.95, 0.95), // 銀色
        0.0,                          // 完全な鏡面
        0.98,                         // 非常に高い反射率
        1.0,                          // 完全な金属
    ));

    let brushed_aluminum = Arc::new(Metal::with_params(
        Color::new(0.7, 0.7, 0.7), // アルミニウム色
        0.3,                       // 中程度の粗さ
        0.85,                      // 高めの反射率
        0.9,                       // 高い金属性
    ));

    let gold = Arc::new(Metal::with_params(
        Color::new(0.8, 0.6, 0.2), // 金色
        0.1,                       // 低めの粗さ
        0.95,                      // 高い反射率
        0.8,                       // 高い金属性
    ));

    let metallic_plastic = Arc::new(Metal::with_params(
        Color::new(0.6, 0.2, 0.2), // 赤みがかった色
        0.2,                       // 低めの粗さ
        0.7,                       // 中程度の反射率
        0.5,                       // 中程度の金属性
    ));

    // シーンの作成
    let mut world = HittableList::new();

    // 地面の追加
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));

    // 中央の拡散球
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));

    // ガラス球（二重球で中空ガラス球を表現）
    world.add(Arc::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        glass.clone(),
    ))); // 外側のガラス球
    world.add(Arc::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        -0.45, // 負の半径で内側の球を作成
        glass,
    ))); // 内側のガラス球

    // 金属球体の配置（後方に配置）
    world.add(Arc::new(Sphere::new(
        Vec3::new(-2.0, 0.0, -2.0),
        0.5,
        mirror,
    ))); // 鏡面の球

    world.add(Arc::new(Sphere::new(
        Vec3::new(-0.7, 0.0, -2.0),
        0.5,
        brushed_aluminum,
    ))); // ブラシドアルミの球

    world.add(Arc::new(Sphere::new(Vec3::new(0.7, 0.0, -2.0), 0.5, gold))); // 金の球

    world.add(Arc::new(Sphere::new(
        Vec3::new(2.0, 0.0, -2.0),
        0.5,
        metallic_plastic,
    ))); // メタリックプラスチックの球

    // BVHを構築してシーンを最適化
    let world = world.optimize();

    // カメラの設定
    let lookfrom = Vec3::new(0.0, 2.5, 5.0);
    let lookat = Vec3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let focus_dist = Some((lookfrom - lookat).length());
    let aperture = Camera::aperture_from_f_number(16.0);

    let camera = Arc::new(camera::Camera::new(
        lookfrom,
        lookat,
        vup,
        40.0,
        aspect_ratio,
        aperture,
        focus_dist,
    ));

    // プログレス表示の設定
    let multi_progress = MultiProgress::new();
    let total_progress =
        Arc::new(multi_progress.add(ProgressBar::new((image_height * image_width) as u64)));
    total_progress.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} pixels ({eta})")
            .unwrap()
            .progress_chars("#>-"),
    );

    let completed_pixels = Arc::new(AtomicUsize::new(0));

    // 画像の生成
    let pixels: Vec<Color> = (0..image_height)
        .into_par_iter()
        .rev()
        .flat_map(|j| {
            let camera = Arc::clone(&camera);
            let world = Arc::clone(&world);
            let completed_pixels = Arc::clone(&completed_pixels);
            let total_progress = Arc::clone(&total_progress);

            (0..image_width).into_par_iter().map(move |i| {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                let mut rng = rand::thread_rng();

                // 各ピクセルに対して複数回サンプリング
                for _ in 0..samples_per_pixel {
                    let u = (i as f64 + rng.gen_range(0.0..1.0)) / (image_width - 1) as f64;
                    let v = (j as f64 + rng.gen_range(0.0..1.0)) / (image_height - 1) as f64;

                    let ray = camera.get_ray(u, v);
                    pixel_color = pixel_color + ray_color(&ray, &world, max_depth);
                }

                // プログレスバーの更新
                completed_pixels.fetch_add(1, Ordering::Relaxed);
                total_progress.set_position(completed_pixels.load(Ordering::Relaxed) as u64);

                // サンプリング結果の平均を取る
                pixel_color * (1.0 / samples_per_pixel as f64)
            })
        })
        .collect();

    total_progress.finish_with_message("レンダリング完了");

    println!("\nファイルに書き込んでいます...");
    if let Err(e) = image::write_ppm("output.ppm", image_width, image_height, &pixels) {
        eprintln!("Error writing image: {}", e);
    }
    println!("完了！");
}
