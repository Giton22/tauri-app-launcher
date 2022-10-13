#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
 )]
 use std::str;
 use std::{os::windows::process::CommandExt};



#[tauri::command]
 fn run_exe(file_path:&str) {
    const DETACHED_PROCESS: u32 = 0x00000008;

    std::process::Command::new("cmd")
    .raw_arg(format!(r#"/C start "" "{}""#,file_path))
    .creation_flags(DETACHED_PROCESS).spawn().expect("Cannot start application");
}
#[tauri::command]
fn get_file_name(file_path:&str)->String{
let path = file_path;
let mut output_value=String::new();
let output = std::process::Command::new("powershell").raw_arg(format!(r#"(get-command "{}").fileversioninfo.filedescription"#,path)).output().expect("failed to execute process");
output_value.push_str(match str::from_utf8(&output.stdout) {
    Ok(val) => val,
    Err(_) => panic!("got non UTF-8 data from git"),
});
return output_value;
}

// use winsafe::prelude::*;
// use winsafe::{HINSTANCE, ResourceInfo};

// #[tauri::command]
// fn message_box()-> String{

//     let exe_name = HINSTANCE::NULL.GetModuleFileName();
// let res_info = ResourceInfo::read_from(&exe_name);
//     return String::from("test");
// }
 fn main() {
    tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![run_exe,get_file_name,message_box])
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
  }