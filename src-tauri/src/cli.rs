use tauri::api::cli::Matches;

pub fn main(cli: &Matches) {
  println!("{:#?}", cli);
}
