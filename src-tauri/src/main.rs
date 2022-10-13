#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use std::os::windows::process::CommandExt;
use std::str;

#[tauri::command]
fn run_exe(file_path: &str) {
    const DETACHED_PROCESS: u32 = 0x00000008;

    std::process::Command::new("cmd")
        .raw_arg(format!(r#"/C start "" "{}""#, file_path))
        .creation_flags(DETACHED_PROCESS)
        .spawn()
        .expect("Cannot start application");
}

use winsafe::{ResourceInfo};

#[tauri::command]
fn get_file_name(file_path:&str)->String {

    let res_info = ResourceInfo::read_from(format!(r#"{}"#,file_path).as_str());
    let value = res_info.unwrap();

    for block in value.blocks() {
        if let Some(file_description) = block.product_name() {
            println!("File Description: {}", file_description);
            return format!("{}",file_description);
        }
    }
    return String::from("Unknown");

}
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            run_exe,
            get_file_name
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
