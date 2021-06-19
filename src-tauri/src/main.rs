#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

// use tauri::api::cli::get_matches;
// use tauri::api::config::Config;

pub mod cli;
pub mod cmds;
pub mod structs;

fn main() {
  // match get_matches(Config::from) {
  //   Ok(matches) => cli::main(matches),
  //   None => {}
  // };
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      cmds::exec,
      cmds::error,
      cmds::load_config,
      cmds::save_config
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
