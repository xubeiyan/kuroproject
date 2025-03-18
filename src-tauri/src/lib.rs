use serde_json::json;
use std::ffi::OsString;
use std::fs::{self, File};
use std::path::{Component, Path, PathBuf};
use sysinfo::Disks;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_free_space])
        .invoke_handler(tauri::generate_handler![get_all_save_and_config])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// 获取可用空间和磁盘类型
#[tauri::command(rename_all = "snake_case")]
fn get_free_space(file_path: String) -> Option<String> {
    let path = Path::new(&file_path);
    let driver: OsString = match path.components().next().unwrap() {
        Component::Prefix(prefix_component) => prefix_component.as_os_str().into(),
        _ => unreachable!(),
    };

    // println!("{:?}", driver.to_str().unwrap());
    let disks = Disks::new_with_refreshed_list();

    for disk in disks.list() {
        if disk.mount_point().to_str().unwrap() == format!("{}\\", driver.to_str().unwrap()) {
            let disk_info = json!({
                "size": disk.available_space(),
                "removable": disk.is_removable(),
            });

            return Some(disk_info.to_string());
        }
    }

    None
}

// 读取指定目录的所有文件夹和kuro.conf文件
#[tauri::command(rename_all = "snake_case")]
fn get_all_save_and_config(path: String) -> String {
    let mut start_path = PathBuf::from(&path);
    start_path.push("kuro.conf");

    if !Path::new(start_path.to_str().expect("start path to str failed")).exists() {
        return "not found kuro.conf".to_string();
    }

    let file = File::open(start_path).unwrap();
    let json: serde_json::Value = serde_json::from_reader(file).expect("not proper JSON");

    let entries = fs::read_dir(path).unwrap();

    let mut file_list = Vec::new();

    for entry in entries {
        let entry = entry.unwrap();
        let file_type = if entry.file_type().unwrap().is_dir() {
            "dir"
        } else {
            "file"
        };
        let name = entry.file_name().into_string().unwrap();

        file_list.push(json!({
            "type": file_type,
            "name": name,
        }));
    }

    let result = json!({
        "conf_file_content": json!(json),
        "save_folder_list": file_list
    });

    return serde_json::to_string(&result).unwrap();
}
