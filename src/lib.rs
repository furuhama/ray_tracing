use rand::Rng;
use std::ops::{Add, Div, Mul, Sub};
use std::sync::Arc;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn unit_vector(&self) -> Vec3 {
        let len = self.length();
        *self / len
    }

    // 反射ベクトルの計算
    pub fn reflect(&self, normal: &Vec3) -> Vec3 {
        *self - *normal * 2.0 * self.dot(normal)
    }

    // 屈折ベクトルの計算（スネルの法則）
    pub fn refract(&self, normal: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = (-*self).dot(normal).min(1.0);
        let r_out_perp = etai_over_etat * (*self + cos_theta * *normal);
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * *normal;
        r_out_perp + r_out_parallel
    }
}

// Vec3型をColor型としても使用
pub type Color = Vec3;

impl Color {
    // RGB値を0-255の範囲で出力
    pub fn to_rgb(&self) -> (u8, u8, u8) {
        (
            (256.0 * self.x.clamp(0.0, 0.999)) as u8,
            (256.0 * self.y.clamp(0.0, 0.999)) as u8,
            (256.0 * self.z.clamp(0.0, 0.999)) as u8,
        )
    }
}

// 基本的な演算子のオーバーロード
impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, t: f64) -> Vec3 {
        Vec3 {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        v * self
    }
}

// Vec3同士の乗算（アダマール積）
impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, t: f64) -> Vec3 {
        self * (1.0 / t)
    }
}

#[derive(Debug)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}

pub struct ScatterInfo {
    pub attenuation: Color,
    pub scattered: Ray,
}

// 金属マテリアル
#[derive(Clone)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Metal {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

// 誘電体マテリアル（ガラスなど）
#[derive(Clone)]
pub struct Dielectric {
    // 屈折率（Index of Refraction）
    ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Dielectric { ir }
    }

    // Schlickの近似を用いた反射率の計算
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<ScatterInfo> {
        let mut rng = rand::thread_rng();

        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = ray_in.direction().unit_vector();
        let cos_theta = (-unit_direction).dot(&rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract
            || Self::reflectance(cos_theta, refraction_ratio) > rng.gen_range(0.0..1.0)
        {
            // 全反射または確率的な反射
            unit_direction.reflect(&rec.normal)
        } else {
            // 屈折
            unit_direction.refract(&rec.normal, refraction_ratio)
        };

        Some(ScatterInfo {
            scattered: Ray::new(rec.point, direction),
            attenuation: Color::new(1.0, 1.0, 1.0),
        })
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<ScatterInfo> {
        let reflected = ray_in.direction().unit_vector().reflect(&rec.normal);
        let scattered = Ray::new(rec.point, reflected + random_unit_vector() * self.fuzz);

        if scattered.direction().dot(&rec.normal) > 0.0 {
            Some(ScatterInfo {
                scattered,
                attenuation: self.albedo,
            })
        } else {
            None
        }
    }
}

pub trait Material: Send + Sync + 'static {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<ScatterInfo>;
}

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Arc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.direction().dot(&outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -1.0 * outward_normal
        };
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

// ランダムな単位ベクトルを生成
pub fn random_unit_vector() -> Vec3 {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    let a = rng.gen_range(0.0..2.0 * std::f64::consts::PI);
    let z = rng.gen_range(-1.0..1.0);
    let r = ((1.0_f64 - z * z) as f64).sqrt();

    Vec3::new(r * a.cos(), r * a.sin(), z)
}

#[derive(Clone)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, rec: &HitRecord) -> Option<ScatterInfo> {
        let mut scatter_direction = rec.normal + random_unit_vector();

        // 散乱方向がゼロベクトルに近い場合は法線方向を使用
        if scatter_direction.length_squared() < 1e-8 {
            scatter_direction = rec.normal;
        }

        Some(ScatterInfo {
            scattered: Ray::new(rec.point, scatter_direction),
            attenuation: self.albedo,
        })
    }
}

pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Arc<dyn Material>) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = oc.dot(&ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        // 衝突点のうち、範囲内で最も近いものを見つける
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let t = root;
        let point = ray.at(t);
        let outward_normal = (point - self.center) / self.radius;
        let mut rec = HitRecord {
            point,
            normal: outward_normal,
            material: Arc::clone(&self.material),
            t,
            front_face: false,
        };
        rec.set_face_normal(ray, outward_normal);

        Some(rec)
    }
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut hit_anything = None;

        for object in &self.objects {
            if let Some(hit_record) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit_record.t;
                hit_anything = Some(HitRecord {
                    point: hit_record.point,
                    normal: hit_record.normal,
                    material: Arc::clone(&hit_record.material),
                    t: hit_record.t,
                    front_face: hit_record.front_face,
                });
            }
        }

        hit_anything
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

pub mod image {
    use std::fs::File;
    use std::io::{self, Write};

    pub fn write_ppm(
        path: &str,
        width: u32,
        height: u32,
        pixels: &[super::Color],
    ) -> io::Result<()> {
        let mut file = File::create(path)?;

        // PPMヘッダーの書き込み
        writeln!(file, "P3")?;
        writeln!(file, "{} {}", width, height)?;
        writeln!(file, "255")?;

        // ピクセルデータの書き込み
        for pixel in pixels {
            let (r, g, b) = pixel.to_rgb();
            writeln!(file, "{} {} {}", r, g, b)?;
        }

        Ok(())
    }
}
