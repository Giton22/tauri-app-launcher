#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use std::path::{PathBuf};
use std::str;
use std::{os::windows::process::CommandExt};

#[tauri::command]
fn run_exe(file_path: &str) {
    const DETACHED_PROCESS: u32 = 0x00000008;

    std::process::Command::new("cmd")
        .raw_arg(format!(r#"/C start "" "{}""#, file_path))
        .creation_flags(DETACHED_PROCESS)
        .spawn()
        .expect("Cannot start application");
}

use winsafe::{ ResourceInfo};

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

use pelite::{FileMap, PeFile};
#[tauri::command]
fn get_icon(file_path: &str, dest_path: &str) -> String {
    let map = FileMap::open(&file_path).expect("Error opening the binary");
    let file = PeFile::from_bytes(&map).expect("Error parsing the binary");
    let dest = PathBuf::from(dest_path);
    let resources = file
        .resources()
        .expect("Error binary does not have resources");
        let mut output_path=String::new();
    for (_,group) in resources.icons().filter_map(Result::ok) {
        // Write the ICO file
        let mut contents = Vec::new();
        group.write(&mut contents).unwrap();
        let path = dest.join(&format!("{}.png", get_file_name(file_path)));
        output_path = path.display().to_string();
        println!("{}", path.display());
        let _ = std::fs::write(&path, &contents);
        break;
    }
    println!("{}",output_path);
    return String::from(output_path);
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            run_exe,
            get_file_name,
            get_icon
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
