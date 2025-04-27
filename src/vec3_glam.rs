use glam::Vec3A;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Vec3Glam(Vec3A);

impl Vec3Glam {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3Glam(Vec3A::new(x as f32, y as f32, z as f32))
    }

    pub fn x(&self) -> f64 {
        self.0.x as f64
    }

    pub fn y(&self) -> f64 {
        self.0.y as f64
    }

    pub fn z(&self) -> f64 {
        self.0.z as f64
    }

    pub fn get(&self, index: usize) -> f64 {
        match index {
            0 => self.x(),
            1 => self.y(),
            2 => self.z(),
            _ => panic!("Vec3のインデックスは0-2の範囲である必要があります"),
        }
    }

    pub fn length(&self) -> f64 {
        self.0.length() as f64
    }

    pub fn length_squared(&self) -> f64 {
        self.0.length_squared() as f64
    }

    pub fn dot(&self, other: &Vec3Glam) -> f64 {
        self.0.dot(other.0) as f64
    }

    pub fn cross(&self, other: &Vec3Glam) -> Vec3Glam {
        Vec3Glam(self.0.cross(other.0))
    }

    pub fn unit_vector(&self) -> Vec3Glam {
        Vec3Glam(self.0.normalize())
    }

    pub fn reflect(&self, normal: &Vec3Glam) -> Vec3Glam {
        let dot_product = self.0.dot(normal.0);
        Vec3Glam(self.0 - 2.0 * dot_product * normal.0)
    }

    pub fn refract(&self, normal: &Vec3Glam, etai_over_etat: f64) -> Vec3Glam {
        let cos_theta = (-*self).dot(normal).min(1.0);
        let r_out_perp = etai_over_etat as f32 * (self.0 + cos_theta as f32 * normal.0);
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * normal.0;
        Vec3Glam(r_out_perp + r_out_parallel)
    }
}

impl Add for Vec3Glam {
    type Output = Vec3Glam;

    fn add(self, other: Vec3Glam) -> Vec3Glam {
        Vec3Glam(self.0 + other.0)
    }
}

impl Sub for Vec3Glam {
    type Output = Vec3Glam;

    fn sub(self, other: Vec3Glam) -> Vec3Glam {
        Vec3Glam(self.0 - other.0)
    }
}

impl Mul<f64> for Vec3Glam {
    type Output = Vec3Glam;

    fn mul(self, t: f64) -> Vec3Glam {
        Vec3Glam(self.0 * t as f32)
    }
}

impl Mul<Vec3Glam> for f64 {
    type Output = Vec3Glam;

    fn mul(self, v: Vec3Glam) -> Vec3Glam {
        v * self
    }
}

impl Mul<Vec3Glam> for Vec3Glam {
    type Output = Vec3Glam;

    fn mul(self, other: Vec3Glam) -> Vec3Glam {
        Vec3Glam(self.0 * other.0)
    }
}

impl Div<f64> for Vec3Glam {
    type Output = Vec3Glam;

    fn div(self, t: f64) -> Vec3Glam {
        self * (1.0 / t)
    }
}

impl Neg for Vec3Glam {
    type Output = Vec3Glam;

    fn neg(self) -> Vec3Glam {
        Vec3Glam(-self.0)
    }
}

// Vec3Glam型をColorGlam型としても使用
pub type ColorGlam = Vec3Glam;

impl ColorGlam {
    pub fn to_rgb(&self) -> (u8, u8, u8) {
        (
            (256.0 * self.x().clamp(0.0, 0.999)) as u8,
            (256.0 * self.y().clamp(0.0, 0.999)) as u8,
            (256.0 * self.z().clamp(0.0, 0.999)) as u8,
        )
    }
}
