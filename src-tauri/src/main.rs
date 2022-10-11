#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use std::{process::Command};
use execute::Execute;
use systemicons;

use native_dialog::{FileDialog};
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn exe_picker() -> String {

    let path = FileDialog::new()
    .set_location("~/Desktop")
    .add_filter("Executable", &["exe"])
    .show_open_single_file()
    .unwrap();
    
    let path = match path {
        Some(x)=> return x.to_string_lossy().to_string(),
        None => return format!("None")
    };

    

}
#[tauri::command]
fn run_exe(filePath:&str){

    let mut exec_command = Command::new(filePath);
    exec_command.execute();

}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![exe_picker, run_exe])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
