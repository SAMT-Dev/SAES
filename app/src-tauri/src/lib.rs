use std::env;

use hash::{check_hash, clear_check_hash, get_image, get_image_hash, get_images};
use saes_shared::structs::user::Driver;
use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    AppHandle, Emitter, Listener, Manager, PhysicalSize, Size,
};
use tokio::sync::RwLock;
use util::login::{
    begin_login, check_auth, check_envs, check_faction, done_setup, get_api_url,
    get_faction_options, save_auth_token, save_game_dir, set_faction, set_game_dir,
};

mod hash;
mod util;

pub static DISCORD_TOKEN: RwLock<Option<String>> = RwLock::const_new(None);
pub static API_URL: RwLock<Option<String>> = RwLock::const_new(None);
pub static AUTH: RwLock<Option<Driver>> = RwLock::const_new(None);

#[tauri::command]
async fn update_done(app: AppHandle) {
    app.emit("setloadertext", "Konfiguráció betöltése").unwrap();
    let main = tauri::WebviewWindowBuilder::from_config(
        &app,
        &app.config().app.windows.get(2).unwrap().clone(),
    )
    .unwrap();
    let config = util::config::load_config();
    if config.is_none() {
        let loader = app.get_webview_window("loader").unwrap();
        app.emit("setloadertext", "Konfiguráció nem létezik")
            .unwrap();
        let main = main.build().unwrap();
        let main_clone = main.clone();
        main.once("panel", move |_| {
            main_clone.emit("changepanel", "main/noconfig").unwrap();
        });
        loader.close().unwrap();
        main.show().unwrap();
        main.set_focus().unwrap();
        return;
    }
    app.emit("setloadertext", "Felület előkészítése").unwrap();
    let main = main.build().unwrap();
    let main_clone = main.clone();
    main.once("panel", move |_| {
        main_clone.emit("changepanel", "main").unwrap();
    });
    let loader = app.get_webview_window("loader").unwrap();
    loader.close().unwrap();
    main.show().unwrap();
    main.set_focus().unwrap();
    main.set_size(Size::Physical(PhysicalSize {
        height: 720,
        width: 1280,
    }))
    .unwrap();
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
            util::config::setup_folders();
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
            set_game_dir,
            save_game_dir,
            done_setup,
            get_image,
            get_images,
            get_image_hash,
            check_hash,
            clear_check_hash,
            stop_app,
            check_auth,
            save_auth_token,
            check_faction,
            get_faction_options,
            set_faction
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn stop_app(app: AppHandle) {
    app.exit(0);
}
