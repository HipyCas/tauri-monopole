use serde_json;
use std::str::FromStr;
use tauri::api::dialog::{message, FileDialogBuilder};

use magnetic_monopole::exec_return;

use crate::structs::{Config, IterationResults, JsVec3};

#[tauri::command]
pub fn error(title: &str, msg: &str) {
  message(title, msg)
}

#[tauri::command]
pub fn exec(
  count: u32,
  velocity: JsVec3,
  position: JsVec3,
  mass: f32,
  charge: f32,
  intensity: f32,
  time_ms: f32,
) -> Vec<IterationResults> {
  let res = exec_return(
    count,
    &mut (velocity.to_vec3()),
    &mut (position.to_vec3()),
    mass,
    charge,
    intensity,
    time_ms,
    false,
  )
  .2;

  let mut fin = vec![];
  for item in res {
    fin.push(IterationResults::new(
      JsVec3::from_vec3(&item.0),
      JsVec3::from_vec3(&item.1),
      JsVec3::from_vec3(&item.2),
      JsVec3::from_vec3(&item.3),
    ))
  }

  fin
}

#[tauri::command]
pub fn load_config() -> Result<Config, String> {
  let path = match FileDialogBuilder::new().pick_file() {
    Some(path) => path,
    None => return Err(String::from_str("Unable to get file path of configuration file").unwrap()),
  };
  let contents = match std::fs::read_to_string(path) {
    Ok(string) => string,
    Err(err) => {
      return Err(format!(
        "Error while reading the contents of the file: {}",
        err
      ))
    }
  };
  let config: Config = match serde_json::from_str(&contents) {
    Ok(parsed) => parsed,
    Err(err) => return Err(format!("Error while parsing configuration JSON: {}", err)),
  };
  Ok(config)
}

#[tauri::command]
pub fn save_config(config: Config) -> Result<String, String> {
  let json = match serde_json::to_string(&config) {
    Ok(string) => string,
    Err(_) => return Err(String::from_str("Error while parsing config to JSON string").unwrap()), // TODO Better error handling?
  };
  let path = match FileDialogBuilder::new().save_file() {
    Some(value) => value,
    None => {
      return Err(
        String::from_str("Unable to get file path to save configuration JSON file").unwrap(),
      )
    }
  };
  let str_path = path.to_str().unwrap_or("unknown").to_owned();
  match std::fs::write(path, json) {
    Ok(_) => {}
    Err(_) => {
      return Err(format!(
        "Unable to write configuration to file in path {}",
        str_path
      ))
    }
  };
  Ok(str_path)
}
