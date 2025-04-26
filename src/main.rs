use ray_tracing::{Color, Hittable, HittableList, Ray, Sphere, Vec3, image};

fn ray_color(ray: &Ray, world: &impl Hittable) -> Color {
    if let Some(rec) = world.hit(ray, 0.0, f64::INFINITY) {
        // 法線を色として使用（法線の方向を可視化）
        return 0.5
            * Color::new(
                rec.normal.x() + 1.0,
                rec.normal.y() + 1.0,
                rec.normal.z() + 1.0,
            );
    }

    // 背景のグラデーション
    let unit_direction = ray.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

fn main() {
    // 画像の基本設定
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    // カメラの基本設定
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal * 0.5 - vertical * 0.5 - Vec3::new(0.0, 0.0, focal_length);

    // シーンの作成
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5))); // 中央の球体
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0))); // 地面

    // 画像の生成
    let mut pixels = Vec::with_capacity((image_width * image_height) as usize);

    for j in (0..image_height).rev() {
        println!("Remaining scanlines: {}", j);
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;

            let direction = lower_left_corner + horizontal * u + vertical * v - origin;
            let ray = Ray::new(origin, direction);
            let pixel_color = ray_color(&ray, &world);
            pixels.push(pixel_color);
        }
    }

    println!("Writing image to file...");
    if let Err(e) = image::write_ppm("output.ppm", image_width, image_height, &pixels) {
        eprintln!("Error writing image: {}", e);
    }
    println!("Done!");
}
