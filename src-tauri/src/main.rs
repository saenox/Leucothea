// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use harsh::Harsh;
use qrcode_generator::QrCodeEcc;
use base64::{Engine as _, engine::general_purpose};
use clipboard_anywhere::set_clipboard;

#[tauri::command]
fn hash(url: &str, id: &str) -> Vec<String> {
    let hasher = Harsh::default();
    let tar = format!("{}/openh5/training-sharing-platform/pages/home-page/index?locationId={}", url, hasher.encode_hex(id).unwrap());

    let result: Vec<u8> = qrcode_generator::to_png_to_vec(tar.as_bytes(), QrCodeEcc::Low, 1024).unwrap();

    let image = general_purpose::STANDARD.encode(&result);

    vec![tar, format!("data:image/png;base64,{}", image)]
}

#[tauri::command]
fn cpy(url: &str) {
    let _ = set_clipboard(url);
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![hash, cpy])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
