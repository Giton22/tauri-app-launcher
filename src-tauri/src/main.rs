#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use std::str;
use std::{os::windows::process::CommandExt, process::Command};

#[tauri::command]
fn run_exe(file_path: &str) {
    const DETACHED_PROCESS: u32 = 0x00000008;

    std::process::Command::new("cmd")
        .raw_arg(format!(r#"/C start "" "{}""#, file_path))
        .creation_flags(DETACHED_PROCESS)
        .spawn()
        .expect("Cannot start application");
}

use winsafe::ResourceInfo;

#[tauri::command]
fn get_file_name(file_path: &str) -> String {
    let res_info = ResourceInfo::read_from(format!(r#"{}"#, file_path).as_str());
    let value = res_info.unwrap();

    for block in value.blocks() {
        if let Some(file_description) = block.product_name() {
            println!("File Description: {}", file_description);
            return format!("{}", file_description);
        }
    }
    return String::from("Unknown");
}

use std::io::{self, Write};
#[tauri::command]
fn get_icon(app_dir_path:&str,file_path: &str) -> String {

    let file_name = get_file_name(file_path);
    let output = Command::new("extracticon")
        .raw_arg(format!(
            r#"{} {}assets/{}.png"#,
            file_path, app_dir_path, file_name
        ))
        .output()
        .expect("program not found");
    println!("{}assets/{}.png", app_dir_path, file_name);
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
    return String::from(format!(r#"assets\{}.png"#, file_name));
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![run_exe, get_file_name, get_icon])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
