use dotenv::dotenv;
use keyring;
use std::{env, error::Error};

#[cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

fn main() {
    dotenv().ok();
    let context = tauri::generate_context!();
    tauri::Builder::default()
        // .menu(tauri::Menu::os_default(&context.package_info().name))
        .invoke_handler(tauri::generate_handler![store_token, load_token])
        .run(context)
        .expect("error while running tauri application");
}

#[tauri::command]
fn store_token(username: &str, service: &str, value: &str) -> bool {
    let entry = keyring::Entry::new(service, username);
    entry.set_password(value).is_ok()
}

#[tauri::command]
fn load_token(username: &str, service: &str) -> String {
    // let entry = keyring::Entry::new(service, username);
    // entry.get_password().unwrap_or_default()
    env::var(service.clone()).expect(format!("{} is not found", &service).as_ref())
}
