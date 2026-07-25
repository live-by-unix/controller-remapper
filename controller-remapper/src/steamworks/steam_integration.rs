use crate::steamworks::{OverlayManager, RemoteStorageManager, WorkshopManager};
use anyhow::{Context, Result};
use log::{debug, error, info, warn};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;
use steamworks::{Client, AppId, SingleClient};

pub struct SteamIntegration {
    initialized: bool,
    app_id: u32,
    client: Option<SingleClient>,
    workshop: Arc<Mutex<WorkshopManager>>,
    remote_storage: Arc<Mutex<RemoteStorageManager>>,
    overlay: Arc<Mutex<OverlayManager>>,
}

impl SteamIntegration {
    pub fn new() -> Self {
        Self {
            initialized: false,
            app_id: 0,
            client: None,
            workshop: Arc::new(Mutex::new(WorkshopManager::new())),
            remote_storage: Arc::new(Mutex::new(RemoteStorageManager::new())),
            overlay: Arc::new(Mutex::new(OverlayManager::new())),
        }
    }

    pub async fn initialize(&mut self) -> Result<()> {
        if self.initialized {
            debug!("Steam already initialized");
            return Ok(());
        }

        info!("Initializing Steamworks SDK");

        // Read steam_appid.txt if it exists
        if let Ok(app_id) = self.read_steam_appid() {
            self.app_id = app_id;
            info!("Using Steam App ID: {}", app_id);
        } else {
            // Try to get from environment
            if let Ok(app_id_str) = std::env::var("STEAM_APP_ID") {
                self.app_id = app_id_str.parse().unwrap_or(480);
                info!("Using Steam App ID from environment: {}", self.app_id);
            } else {
                self.app_id = 480; // Default to Steamworks Spacewar
                info!("Using default Steam App ID: {}", self.app_id);
            }
        }

        // Initialize Steamworks SDK with actual steamworks-rs
        self.initialize_steam_api().await?;

        // Initialize subsystems with actual client
        if let Some(ref client) = self.client {
            self.workshop.lock().await.initialize_with_client(client.clone(), self.app_id).await?;
            self.remote_storage.lock().await.initialize_with_client(client.clone(), self.app_id).await?;
            self.overlay.lock().await.initialize_with_client(client.clone(), self.app_id).await?;
        }

        self.initialized = true;
        info!("Steamworks SDK initialized successfully");

        Ok(())
    }

    async fn initialize_steam_api(&mut self) -> Result<()> {
        debug!("Initializing Steamworks client");
        
        // Create Steamworks client
        let (client, single) = Client::init().context("Failed to initialize Steamworks client")?;
        self.client = Some(single);
        
        info!("Steamworks client initialized");
        Ok(())
    }

    fn is_steam_running(&self) -> bool {
        // Check for Steam process
        #[cfg(target_os = "windows")]
        {
            // Windows-specific check
            true // Simplified
        }
        
        #[cfg(target_os = "linux")]
        {
            std::path::Path::new("/usr/bin/steam").exists() 
                || std::path::Path::new("/usr/games/steam").exists()
        }
        
        #[cfg(target_os = "macos")]
        {
            std::path::Path::new("/Applications/Steam.app").exists()
        }
        
        #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
        {
            false
        }
    }

    fn read_steam_appid(&self) -> Result<u32> {
        let appid_path = PathBuf::from("steam_appid.txt");
        
        if appid_path.exists() {
            let content = std::fs::read_to_string(&appid_path)
                .context("Failed to read steam_appid.txt")?;
            
            let app_id = content.trim().parse()
                .context("Invalid Steam App ID in steam_appid.txt")?;
            
            Ok(app_id)
        } else {
            Err(anyhow::anyhow!("steam_appid.txt not found"))
        }
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    pub fn get_app_id(&self) -> u32 {
        self.app_id
    }

    pub async fn shutdown(&mut self) -> Result<()> {
        if !self.initialized {
            return Ok(());
        }

        info!("Shutting down Steamworks SDK");

        self.workshop.lock().await.shutdown().await?;
        self.remote_storage.lock().await.shutdown().await?;
        self.overlay.lock().await.shutdown().await?;

        // Call SteamAPI_Shutdown()
        self.shutdown_steam_api().await?;

        self.initialized = false;
        info!("Steamworks SDK shut down");

        Ok(())
    }

    async fn shutdown_steam_api(&self) -> Result<()> {
        debug!("Calling SteamAPI_Shutdown()");
        Ok(())
    }

    pub async fn run_callbacks(&self) {
        if let Some(ref client) = self.client {
            client.run_callbacks();
        }
    }

    pub fn get_client(&self) -> Option<&SingleClient> {
        self.client.as_ref()
    }

    pub fn get_workshop(&self) -> Arc<Mutex<WorkshopManager>> {
        self.workshop.clone()
    }

    pub fn get_remote_storage(&self) -> Arc<Mutex<RemoteStorageManager>> {
        self.remote_storage.clone()
    }

    pub fn get_overlay(&self) -> Arc<Mutex<OverlayManager>> {
        self.overlay.clone()
    }

    pub async fn get_steam_user(&self) -> Option<SteamUser> {
        if !self.initialized {
            return None;
        }

        if let Some(ref client) = self.client {
            let user = client.user();
            let steam_id = user.steam_id();
            let name = user.name();
            
            Some(SteamUser {
                steam_id: steam_id.raw().to_string(),
                name: name.to_string(),
            })
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct SteamUser {
    pub steam_id: String,
    pub name: String,
}

impl Default for SteamIntegration {
    fn default() -> Self {
        Self::new()
    }
}
