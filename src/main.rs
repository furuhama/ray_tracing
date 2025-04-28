mod aabb;
mod bvh;
mod camera;
mod image;
mod material;
mod object;
mod ray;
mod scene;
mod types;
mod vec3_glam;
mod volumetric;

use std::env;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::volumetric::{VolumetricMedium, fog::UniformFog};
use camera::Camera;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use material::{Dielectric, Lambertian, Metal};
use object::{HittableList, Sphere};
use rand::prelude::*;
use ray::Ray;
use rayon::prelude::*;
use scene::{MaterialConfig, Scene, ShapeConfig, VolumetricConfig};
use types::{Hittable, Material};
use vec3_glam::ColorGlam;

fn ray_color(
    ray: &Ray,
    world: &impl Hittable,
    volumetric: Option<&dyn VolumetricMedium>,
    depth: i32,
) -> ColorGlam {
    // 反射回数が制限を超えた場合は黒を返す
    if depth <= 0 {
        return ColorGlam::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(ray, 0.001, f64::INFINITY) {
        let mut color = ColorGlam::new(0.0, 0.0, 0.0);

        if let Some(scatter) = rec.material.scatter(ray, &rec) {
            color =
                scatter.attenuation * ray_color(&scatter.scattered, world, volumetric, depth - 1);
        }

        // ボリューメトリック効果の適用
        if let Some(medium) = volumetric {
            let (scattered_light, transmittance) = medium.sample(ray, 0.0, rec.t);
            color = color * transmittance + scattered_light;
        }

        return color;
    }

    let unit_direction = ray.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    let background = ColorGlam::new(1.0, 1.0, 1.0) * (1.0 - t) + ColorGlam::new(0.5, 0.7, 1.0) * t;

    // 背景色にもボリューメトリック効果を適用
    if let Some(medium) = volumetric {
        let (scattered_light, transmittance) = medium.sample(ray, 0.0, 1000.0); // 十分な距離
        background * transmittance + scattered_light
    } else {
        background
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let scene_path = env::args()
        .nth(1)
        .unwrap_or_else(|| "scenes/default.yaml".to_string());

    let scene = Scene::from_yaml_file(&scene_path)?;

    // Convert scene config to actual scene objects
    let camera = Arc::new(Camera::new(
        scene.camera.look_from.into(),
        scene.camera.look_at.into(),
        scene.camera.vup.into(),
        scene.camera.vfov,
        scene.camera.aspect_ratio,
        scene.camera.aperture,
        scene.camera.focus_dist,
    ));

    let mut world = HittableList::new();

    for obj in scene.objects {
        let material: Arc<dyn Material> = match obj.material {
            MaterialConfig::Lambertian { albedo } => Arc::new(Lambertian::new(albedo.into())),
            MaterialConfig::Metal { albedo, fuzz } => Arc::new(Metal::new(albedo.into(), fuzz)),
            MaterialConfig::Dielectric { ir } => Arc::new(Dielectric::new(ir)),
        };

        match obj.shape {
            ShapeConfig::Sphere { center, radius } => {
                world.add(Arc::new(Sphere::new(center.into(), radius, material)));
            }
        }
    }

    // ボリューメトリック効果の設定
    let volumetric: Option<Arc<Box<dyn VolumetricMedium>>> =
        scene.volumetric.map(|config| match config {
            VolumetricConfig::UniformFog { color, density } => {
                Arc::new(
                    Box::new(UniformFog::new(color.into(), density)) as Box<dyn VolumetricMedium>
                )
            }
        });

    // 画像の基本設定
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 800;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 200; // より多くのサンプル数で画質向上

    // レンダリングの設定
    let max_depth = 50; // 反射の最大回数

    // BVHを構築してシーンを最適化
    let world = world.optimize();

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
    let pixels: Vec<ColorGlam> = (0..image_height)
        .into_par_iter()
        .rev()
        .flat_map(|j| {
            let camera = Arc::clone(&camera);
            let world = Arc::clone(&world);
            let volumetric = volumetric.clone();
            let completed_pixels = Arc::clone(&completed_pixels);
            let total_progress = Arc::clone(&total_progress);

            (0..image_width).into_par_iter().map(move |i| {
                let mut pixel_color = ColorGlam::new(0.0, 0.0, 0.0);
                let mut rng = rand::thread_rng();

                // 各ピクセルに対して複数回サンプリング
                for _ in 0..samples_per_pixel {
                    let u = (i as f64 + rng.gen_range(0.0..1.0)) / (image_width - 1) as f64;
                    let v = (j as f64 + rng.gen_range(0.0..1.0)) / (image_height - 1) as f64;

                    let ray = camera.get_ray(u, v);
                    pixel_color = pixel_color
                        + ray_color(
                            &ray,
                            &world,
                            volumetric.as_deref().map(|v| v.as_ref()),
                            max_depth,
                        );
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

    Ok(())
}
