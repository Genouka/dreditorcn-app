// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod cmd;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        //.invoke_system(include_str!("./inject.js").to_string())
        .invoke_handler(tauri::generate_handler![
            cmd::read_file_11,
            cmd::write_sync_t,
            cmd::get_platform,
            cmd::get_save_directory
        ])
        .run(tauri::generate_context!())
        .expect("加载失败，联系开发者秋冥解决");
}
