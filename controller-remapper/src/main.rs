// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod core;
mod profiles;
mod steamworks;
mod ui;

use core::controller_manager::ControllerManager;
use profiles::profile_manager::ProfileManager;
use steamworks::steam_integration::SteamIntegration;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    env_logger::init();
    
    let controller_manager = Arc::new(Mutex::new(ControllerManager::new().await));
    let profile_manager = Arc::new(ProfileManager::new());
    let steam_integration = Arc::new(Mutex::new(SteamIntegration::new()));
    
    // Initialize Steam
    if let Err(e) = steam_integration.lock().await.initialize() {
        log::error!("Failed to initialize Steam: {}", e);
    }
    
    // Start controller detection
    let cm_clone = controller_manager.clone();
    tokio::spawn(async move {
        if let Err(e) = cm_clone.lock().await.start_detection().await {
            log::error!("Controller detection error: {}", e);
        }
    });
    
    // Build Tauri app
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(move |app| {
            let app_handle = app.handle().clone();
            
            // Register Tauri commands
            ui::commands::register_commands(
                app,
                controller_manager.clone(),
                profile_manager.clone(),
                steam_integration.clone(),
            );
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
