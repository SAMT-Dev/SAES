use std::env;

use lazy_static::lazy_static;
use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    AppHandle, Emitter, Manager,
};
use tokio::sync::RwLock;
use util::login::{begin_login, check_envs, get_api_url};

mod util;

lazy_static! {
    pub static ref DISCORD_TOKEN: RwLock<Option<String>> = RwLock::new(None);
    pub static ref API_URL: RwLock<Option<String>> = RwLock::new(None);
}

#[tauri::command]
async fn update_done(app: AppHandle) {
    app.emit("setloadertext", "Konfiguráció betöltése").unwrap();
    util::config::setup_folders();
    let config = util::config::load_config();
    if config.is_none() {
        let loader = app.get_webview_window("loader").unwrap();
        let main = app.get_webview_window("main").unwrap();
        app.emit("setloadertext", "Konfiguráció nem létezik")
            .unwrap();
        loader.close().unwrap();
        main.show().unwrap();
        main.set_focus().unwrap();
        app.emit("setmainpage", "noconfig").unwrap();
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_single_instance::init(|_app, _argv, _cwd| {
            println!("Single instance callback");
        }))
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_opener::Builder::new().build())
        .setup(|app| {
            let quit_i = MenuItem::with_id(app, "quit", "Kilépés", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit_i])?;
            TrayIconBuilder::new()
                .menu(&menu)
                .title("SAMT App")
                .tooltip("SAMT App")
                .icon(app.default_window_icon().unwrap().clone())
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {
                        println!("menu item {:?} not handled", event.id);
                    }
                })
                .build(app)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            update_done,
            begin_login,
            get_api_url,
            check_envs,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
