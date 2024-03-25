use std::fmt::format;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
use chrono::prelude::*;
use polars::prelude::*;
use tauri_plugin_dialog::{DialogExt, FileResponse};
use tauri_plugin_fs::FsExt;

// return a file_path `Option`, or `None` if the user closes the dialog
#[tauri::command]
fn greet(name: &str) -> String {
    // format!("Hello, {}! You've been greeted from me!", name)
    df!(
        "integer" => &[1, 2, 3],
        "date" => &[
                NaiveDate::from_ymd_opt(2025, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
                NaiveDate::from_ymd_opt(2025, 1, 2).unwrap().and_hms_opt(0, 0, 0).unwrap(),
                NaiveDate::from_ymd_opt(2025, 1, 3).unwrap().and_hms_opt(0, 0, 0).unwrap(),
        ],
        "float" => &[4.0, 5.0, 6.0],
        "string" => &["a", "b", "c"],
    )
    .unwrap()
    .to_string()
}
#[tauri::command]
fn pick(app_handle: tauri::AppHandle) -> () {
    print!("pick() called from Rust, showing dialog to pick a file...");

    let file = app_handle
        .dialog()
        .file()
        .add_filter("csv files", &["csv"])
        .pick_file(|file_path: Option<FileResponse>| {
            match file_path {
                Some(file_response) => {
                    println!("picked file: {:?}", file_response);
                    // Read the file
                    print!("{}", file_response.path.to_str().unwrap().to_string());

                    let df = CsvReader::from_path(file_response.path.to_str().unwrap().to_string())
                        .unwrap()
                        .finish()
                        .unwrap();
                    println!("{}", df);
                }
                None => {
                    println!("no file picked");
                    "".to_string(); // Return an empty string if no file is picked
                }
            }
        });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            // allowed the given directory
            let scope = app.fs_scope();
            scope.allow_directory("/", false);
            dbg!(scope.allowed());

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet, pick])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
