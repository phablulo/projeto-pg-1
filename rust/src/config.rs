use serde::{Deserialize};
use serde_json::Result;

#[derive(Deserialize)]
pub struct Camera {
  pub position: [f32; 3],
  pub target: [f32; 3],
  pub orientation: [f32; 3],
  pub fov: f32,
  pub image_plane_distance: f32,
  pub width: i32,
  pub height: i32,
  pub ambient_light: [f32; 3],
}
#[derive(Deserialize)]
pub struct Material {
  pub rgb: [f32; 3],
  pub kd: f32,
  pub ks: f32,
  pub kt: f32,
  pub kr: f32,
  pub alpha: f32
}
#[derive(Deserialize)]
pub enum ObjectType {
  Plane,
  Sphere
}
#[derive(Deserialize)]
pub struct Object {
  pub kind: ObjectType,
  pub material: Material,
  // sphere
  pub r: f32,
  pub center: [f32; 3],
  // plane
  pub normal: [f32; 3],
  pub point: [f32; 3]
}
#[derive(Deserialize)]
pub struct Light {
  pub position: [f32; 3],
  pub difuse: [f32; 3],
  pub specular: [f32; 3]
}
#[derive(Deserialize)]
pub enum Background {
  Sky,
  Black
}
#[derive(Deserialize)]
pub struct Dof {
  pub active: bool,
  pub focal_length: f32,
  pub r: f32,
  pub samples: u32
}
#[derive(Deserialize)]
pub struct Scene {
  pub name: String,
  pub camera: Camera,
  pub objects: Vec<Object>,
  pub lights: Vec<Light>,
  pub background: Background,
  pub dof: Dof
}

pub fn parse(contents: String) -> Result<Vec<Scene>> {
  let scenes:Vec<Scene> = serde_json::from_str(&contents)?;
  Ok(scenes)
}