use serde::{Deserialize, Serialize};

use magnetic_monopole::vector::Vec3;

#[derive(Serialize, Deserialize)]
pub struct JsVec3 {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

impl JsVec3 {
  pub fn new(x: f32, y: f32, z: f32) -> Self {
    Self { x, y, z }
  }

  pub fn from_vec3(vec: &Vec3) -> Self {
    Self {
      x: vec.x,
      y: vec.y,
      z: vec.z,
    }
  }

  pub fn to_vec3(&self) -> Vec3 {
    Vec3 {
      x: self.x,
      y: self.y,
      z: self.z,
    }
  }

  pub fn from_str(string: &str) -> Result<JsVec3, String> {
    let err_str = format!(
      "{} is not of the format <x>,<y>,<z>, where the values are of type float",
      string
    );
    let vec = string.split(',').collect::<Vec<&str>>();
    let mut iter = vec.iter();

    let x = match iter.next() {
      Some(value) => value,
      None => return Err(err_str),
    };
    let x: f32 = match x.trim().parse() {
      Ok(value) => value,
      Err(_) => return Err(format!("Could not parse {} for the x value", x)),
    };

    let y = match iter.next() {
      Some(value) => value,
      None => return Err(err_str),
    };
    let y: f32 = match y.trim().parse() {
      Ok(value) => value,
      Err(_) => return Err(format!("Could not parse {} for the x value", x)),
    };

    let z = match iter.next() {
      Some(value) => value,
      None => return Err(err_str),
    };
    let z: f32 = match z.trim().parse() {
      Ok(value) => value,
      Err(_) => return Err(format!("Could not parse {} for the x value", x)),
    };

    Ok(JsVec3 { x, y, z })
  }
}

#[derive(Serialize, Deserialize)]
pub struct IterationResults {
  pub velocity: JsVec3,
  pub position: JsVec3,
  pub acceleration: JsVec3,
  pub force: JsVec3,
}

impl IterationResults {
  pub fn new(velocity: JsVec3, position: JsVec3, acceleration: JsVec3, force: JsVec3) -> Self {
    Self {
      velocity,
      position,
      acceleration,
      force,
    }
  }
}

#[derive(Serialize, Deserialize)]
pub struct Config {
  count: u32,
  velocity: JsVec3,
  position: JsVec3,
  mass: f32,
  charge: f32,
  intensity: f32,
  //? time_ms: f32 <- Should I allow this to be changed
}
