// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    let v1 = collisioner::math::Vector3::new(1.0, 2.0, 3.0);
    let v2 = collisioner::math::Vector3::new(4.0, 5.0, 6.0);
    let v3 = v1 + v2;

    format!("Hello, {}! {:?} + {:?} = {:?}", name, v1, v2, v3)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
