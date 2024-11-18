use serde_json::json;
use std::ffi::OsString;
use std::path::{Component, Path};
use sysinfo::Disks;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_free_space])
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
