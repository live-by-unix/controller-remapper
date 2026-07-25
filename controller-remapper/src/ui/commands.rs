use crate::core::controller_manager::ControllerManager;
use crate::core::controller_types::{ControllerInfo, InputMapper};
use crate::profiles::profile_manager::ProfileManager;
use crate::profiles::profile::Profile;
use crate::steamworks::steam_integration::SteamIntegration;
use std::sync::Arc;
use tauri::{AppHandle, Manager};
use tokio::sync::Mutex;

pub fn register_commands(
    app: &mut tauri::App,
    controller_manager: Arc<Mutex<ControllerManager>>,
    profile_manager: Arc<ProfileManager>,
    steam_integration: Arc<Mutex<SteamIntegration>>,
) {
    app.manage(controller_manager);
    app.manage(profile_manager);
    app.manage(steam_integration);
}

#[tauri::command]
async fn scan_controllers(
    controller_manager: tauri::State<'_, Arc<Mutex<ControllerManager>>>,
) -> Result<Vec<ControllerInfo>, String> {
    controller_manager
        .lock()
        .await
        .get_controllers()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_controller(
    id: String,
    controller_manager: tauri::State<'_, Arc<Mutex<ControllerManager>>>,
) -> Result<Option<ControllerInfo>, String> {
    controller_manager
        .lock()
        .await
        .get_controller(&id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn map_button(
    button: String,
    key: String,
    controller_manager: tauri::State<'_, Arc<Mutex<ControllerManager>>>,
) -> Result<(), String> {
    controller_manager
        .lock()
        .await
        .map_button(&button, &key)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn map_axis(
    axis: String,
    positive_key: Option<String>,
    negative_key: Option<String>,
    controller_manager: tauri::State<'_, Arc<Mutex<ControllerManager>>>,
) -> Result<(), String> {
    controller_manager
        .lock()
        .await
        .map_axis(&axis, positive_key.as_deref(), negative_key.as_deref())
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn clear_mappings(
    controller_manager: tauri::State<'_, Arc<Mutex<ControllerManager>>>,
) -> Result<(), String> {
    controller_manager
        .lock()
        .await
        .clear_mappings()
        .await;
    Ok(())
}

#[tauri::command]
async fn get_input_mapper(
    controller_manager: tauri::State<'_, Arc<Mutex<ControllerManager>>>,
) -> Result<InputMapper, String> {
    controller_manager
        .lock()
        .await
        .get_input_mapper()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn set_input_mapper(
    mapper: InputMapper,
    controller_manager: tauri::State<'_, Arc<Mutex<ControllerManager>>>,
) -> Result<(), String> {
    controller_manager
        .lock()
        .await
        .set_input_mapper(mapper)
        .await;
    Ok(())
}

#[tauri::command]
async fn load_all_profiles(
    profile_manager: tauri::State<'_, Arc<ProfileManager>>,
) -> Result<Vec<crate::profiles::profile::ProfileMetadata>, String> {
    profile_manager
        .load_all_profiles()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_profile(
    profile_id: String,
    profile_manager: tauri::State<'_, Arc<ProfileManager>>,
) -> Result<Option<Profile>, String> {
    Ok(profile_manager.get_profile(&profile_id).await)
}

#[tauri::command]
async fn get_profile_by_name(
    name: String,
    profile_manager: tauri::State<'_, Arc<ProfileManager>>,
) -> Result<Option<Profile>, String> {
    Ok(profile_manager.get_profile_by_name(&name).await)
}

#[tauri::command]
async fn create_profile(
    name: String,
    game_name: String,
    author: String,
    profile_manager: tauri::State<'_, Arc<ProfileManager>>,
) -> Result<Profile, String> {
    profile_manager
        .create_profile(&name, &game_name, &author)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn save_profile(
    profile: Profile,
    profile_manager: tauri::State<'_, Arc<ProfileManager>>,
) -> Result<String, String> {
    profile_manager
        .save_profile(&profile)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn delete_profile(
    profile_id: String,
    profile_manager: tauri::State<'_, Arc<ProfileManager>>,
) -> Result<(), String> {
    profile_manager
        .delete_profile(&profile_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn duplicate_profile(
    profile_id: String,
    new_name: String,
    profile_manager: tauri::State<'_, Arc<ProfileManager>>,
) -> Result<Profile, String> {
    profile_manager
        .duplicate_profile(&profile_id, &new_name)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn import_profile(
    json_content: String,
    profile_manager: tauri::State<'_, Arc<ProfileManager>>,
) -> Result<Profile, String> {
    profile_manager
        .import_profile(&json_content)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn export_profile(
    profile_id: String,
    profile_manager: tauri::State<'_, Arc<ProfileManager>>,
) -> Result<String, String> {
    profile_manager
        .export_profile(&profile_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn search_profiles(
    query: String,
    profile_manager: tauri::State<'_, Arc<ProfileManager>>,
) -> Result<Vec<crate::profiles::profile::ProfileMetadata>, String> {
    Ok(profile_manager.search_profiles(&query).await)
}

#[tauri::command]
async fn get_profiles_for_game(
    game_name: String,
    profile_manager: tauri::State<'_, Arc<ProfileManager>>,
) -> Result<Vec<Profile>, String> {
    Ok(profile_manager.get_profiles_for_game(&game_name).await)
}

#[tauri::command]
async fn initialize_steam(
    steam_integration: tauri::State<'_, Arc<Mutex<SteamIntegration>>>,
) -> Result<bool, String> {
    steam_integration
        .lock()
        .await
        .initialize()
        .await
        .map(|_| true)
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn is_steam_initialized(
    steam_integration: tauri::State<'_, Arc<Mutex<SteamIntegration>>>,
) -> Result<bool, String> {
    Ok(steam_integration.lock().await.is_initialized())
}

#[tauri::command]
async fn get_steam_user(
    steam_integration: tauri::State<'_, Arc<Mutex<SteamIntegration>>>,
) -> Result<Option<crate::steamworks::steam_integration::SteamUser>, String> {
    steam_integration
        .lock()
        .await
        .get_steam_user()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn upload_to_workshop(
    profile: Profile,
    steam_integration: tauri::State<'_, Arc<Mutex<SteamIntegration>>>,
) -> Result<crate::steamworks::workshop::WorkshopUploadResult, String> {
    let workshop = steam_integration.lock().await.get_workshop();
    workshop
        .lock()
        .await
        .upload_profile(&profile)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn download_from_workshop(
    workshop_id: u64,
    steam_integration: tauri::State<'_, Arc<Mutex<SteamIntegration>>>,
) -> Result<Profile, String> {
    let workshop = steam_integration.lock().await.get_workshop();
    workshop
        .lock()
        .await
        .download_profile(workshop_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn subscribe_workshop_item(
    workshop_id: u64,
    steam_integration: tauri::State<'_, Arc<Mutex<SteamIntegration>>>,
) -> Result<(), String> {
    let workshop = steam_integration.lock().await.get_workshop();
    workshop
        .lock()
        .await
        .subscribe(workshop_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn unsubscribe_workshop_item(
    workshop_id: u64,
    steam_integration: tauri::State<'_, Arc<Mutex<SteamIntegration>>>,
) -> Result<(), String> {
    let workshop = steam_integration.lock().await.get_workshop();
    workshop
        .lock()
        .await
        .unsubscribe(workshop_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn search_workshop(
    query: String,
    tags: Vec<String>,
    steam_integration: tauri::State<'_, Arc<Mutex<SteamIntegration>>>,
) -> Result<Vec<crate::steamworks::workshop::WorkshopItem>, String> {
    let workshop = steam_integration.lock().await.get_workshop();
    workshop
        .lock()
        .await
        .search_items(&query, &tags)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn sync_cloud(
    steam_integration: tauri::State<'_, Arc<Mutex<SteamIntegration>>>,
) -> Result<(), String> {
    let remote_storage = steam_integration.lock().await.get_remote_storage();
    remote_storage
        .lock()
        .await
        .sync_cloud()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn save_profile_to_cloud(
    profile: Profile,
    steam_integration: tauri::State<'_, Arc<Mutex<SteamIntegration>>>,
) -> Result<(), String> {
    let remote_storage = steam_integration.lock().await.get_remote_storage();
    remote_storage
        .lock()
        .await
        .save_profile_to_cloud(&profile)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn load_profile_from_cloud(
    profile_id: String,
    steam_integration: tauri::State<'_, Arc<Mutex<SteamIntegration>>>,
) -> Result<Profile, String> {
    let remote_storage = steam_integration.lock().await.get_remote_storage();
    remote_storage
        .lock()
        .await
        .load_profile_from_cloud(&profile_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn show_overlay(
    steam_integration: tauri::State<'_, Arc<Mutex<SteamIntegration>>>,
) -> Result<(), String> {
    let overlay = steam_integration.lock().await.get_overlay();
    overlay
        .lock()
        .await
        .activate_overlay()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn show_profile_selector_overlay(
    steam_integration: tauri::State<'_, Arc<Mutex<SteamIntegration>>>,
) -> Result<(), String> {
    let overlay = steam_integration.lock().await.get_overlay();
    overlay
        .lock()
        .await
        .show_profile_selector()
        .await
        .map_err(|e| e.to_string())
}
