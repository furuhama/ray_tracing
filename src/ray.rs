use crate::vec3_glam::Vec3Glam;

#[derive(Debug)]
pub struct Ray {
    origin: Vec3Glam,
    direction: Vec3Glam,
}

impl Ray {
    pub fn new(origin: Vec3Glam, direction: Vec3Glam) -> Self {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> Vec3Glam {
        self.origin
    }

    pub fn direction(&self) -> Vec3Glam {
        self.direction
    }

    pub fn at(&self, t: f64) -> Vec3Glam {
        self.origin + self.direction * t
    }
}
