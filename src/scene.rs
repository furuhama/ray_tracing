use crate::vec3_glam::Vec3Glam;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Scene {
    pub camera: CameraConfig,
    pub objects: Vec<ObjectConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CameraConfig {
    pub look_from: Vec3Config,
    pub look_at: Vec3Config,
    pub vup: Vec3Config,
    pub vfov: f64,
    pub aspect_ratio: f64,
    pub aperture: f64,
    pub focus_dist: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Vec3Config {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl From<Vec3Config> for Vec3Glam {
    fn from(v: Vec3Config) -> Self {
        Vec3Glam::new(v.x, v.y, v.z)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectConfig {
    pub shape: ShapeConfig,
    pub material: MaterialConfig,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ShapeConfig {
    Sphere { center: Vec3Config, radius: f64 },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MaterialConfig {
    Lambertian { albedo: Vec3Config },
    Metal { albedo: Vec3Config, fuzz: f64 },
    Dielectric { ir: f64 },
}

impl Scene {
    pub fn from_yaml(yaml_str: &str) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_str(yaml_str)
    }

    pub fn from_yaml_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = std::fs::read_to_string(path)?;
        Ok(Self::from_yaml(&contents)?)
    }
}
