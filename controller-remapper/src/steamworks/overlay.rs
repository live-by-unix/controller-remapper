use anyhow::{Context, Result};
use log::{debug, info, warn};
use steamworks::{SingleClient, SteamId};

pub struct OverlayManager {
    app_id: u32,
    initialized: bool,
    client: Option<SingleClient>,
    overlay_enabled: bool,
}

impl OverlayManager {
    pub fn new() -> Self {
        Self {
            app_id: 0,
            initialized: false,
            client: None,
            overlay_enabled: false,
        }
    }

    pub async fn initialize(&mut self, app_id: u32) -> Result<()> {
        self.app_id = app_id;
        self.initialized = true;
        
        // Check if overlay is enabled
        self.overlay_enabled = self.is_overlay_enabled();
        
        info!("Overlay manager initialized for app ID: {} (overlay: {})", 
              app_id, self.overlay_enabled);
        Ok(())
    }

    pub async fn initialize_with_client(&mut self, client: SingleClient, app_id: u32) -> Result<()> {
        self.app_id = app_id;
        self.client = Some(client);
        self.initialized = true;
        
        // Check if overlay is enabled
        self.overlay_enabled = self.is_overlay_enabled();
        
        info!("Overlay manager initialized with Steam client for app ID: {} (overlay: {})", 
              app_id, self.overlay_enabled);
        Ok(())
    }

    pub async fn shutdown(&mut self) -> Result<()> {
        self.initialized = false;
        info!("Overlay manager shut down");
        Ok(())
    }

    pub fn is_overlay_enabled(&self) -> bool {
        if !self.initialized {
            return false;
        }

        if let Some(ref client) = self.client {
            let utils = client.utils();
            utils.is_overlay_enabled()
        } else {
            false
        }
    }

    pub async fn activate_overlay(&self) -> Result<()> {
        if !self.initialized {
            return Err(anyhow::anyhow!("Overlay manager not initialized"));
        }

        if !self.overlay_enabled {
            warn!("Overlay is not enabled");
            return Ok(());
        }

        info!("Activating Steam Overlay");

        if let Some(ref client) = self.client {
            let friends = client.friends();
            friends.activate_game_overlay("friends");
            info!("Steam Overlay activated");
            Ok(())
        } else {
            Err(anyhow::anyhow!("Steam client not available"))
        }
    }

    pub async fn activate_overlay_to_user(&self, steam_id: &str) -> Result<()> {
        if !self.initialized {
            return Err(anyhow::anyhow!("Overlay manager not initialized"));
        }

        if !self.overlay_enabled {
            warn!("Overlay is not enabled");
            return Ok(());
        }

        info!("Activating Steam Overlay to user: {}", steam_id);

        if let Some(ref client) = self.client {
            let friends = client.friends();
            let steam_id_parsed = steam_id.parse::<u64>()
                .context("Invalid Steam ID")?;
            let steam_id = SteamId::from(steam_id_parsed);
            
            friends.activate_game_overlay_to_user(steam_id, "chat");
            info!("Steam Overlay activated to user: {}", steam_id);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Steam client not available"))
        }
    }

    pub async fn activate_overlay_to_web_page(&self, url: &str) -> Result<()> {
        if !self.initialized {
            return Err(anyhow::anyhow!("Overlay manager not initialized"));
        }

        if !self.overlay_enabled {
            warn!("Overlay is not enabled");
            return Ok(());
        }

        info!("Activating Steam Overlay to web page: {}", url);

        if let Some(ref client) = self.client {
            let friends = client.friends();
            friends.activate_game_overlay_to_web_page(url);
            info!("Steam Overlay activated to web page: {}", url);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Steam client not available"))
        }
    }

    pub async fn activate_overlay_to_store(&self, app_id: u32) -> Result<()> {
        if !self.initialized {
            return Err(anyhow::anyhow!("Overlay manager not initialized"));
        }

        if !self.overlay_enabled {
            warn!("Overlay is not enabled");
            return Ok(());
        }

        info!("Activating Steam Overlay to store for app: {}", app_id);

        if let Some(ref client) = self.client {
            let friends = client.friends();
            friends.activate_game_overlay_to_store(steamworks::AppId(app_id));
            info!("Steam Overlay activated to store for app: {}", app_id);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Steam client not available"))
        }
    }

    pub async fn activate_overlay_invite_dialog(&self, lobby_id: &str) -> Result<()> {
        if !self.initialized {
            return Err(anyhow::anyhow!("Overlay manager not initialized"));
        }

        if !self.overlay_enabled {
            warn!("Overlay is not enabled");
            return Ok(());
        }

        info!("Activating Steam Overlay invite dialog for lobby: {}", lobby_id);

        if let Some(ref client) = self.client {
            let friends = client.friends();
            let lobby_id_parsed = lobby_id.parse::<u64>()
                .context("Invalid lobby ID")?;
            let lobby_id = steamworks::LobbyId::from(lobby_id_parsed);
            
            friends.activate_game_overlay_invite_dialog(lobby_id);
            info!("Steam Overlay invite dialog activated for lobby: {}", lobby_id);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Steam client not available"))
        }
    }

    pub async fn set_overlay_notification_position(&self, position: OverlayPosition) -> Result<()> {
        if !self.initialized {
            return Err(anyhow::anyhow!("Overlay manager not initialized"));
        }

        debug!("Setting overlay notification position: {:?}", position);

        if let Some(ref client) = self.client {
            let utils = client.utils();
            let steam_position = match position {
                OverlayPosition::TopLeft => steamworks::OverlayNotificationPosition::TopLeft,
                OverlayPosition::TopRight => steamworks::OverlayNotificationPosition::TopRight,
                OverlayPosition::BottomLeft => steamworks::OverlayNotificationPosition::BottomLeft,
                OverlayPosition::BottomRight => steamworks::OverlayNotificationPosition::BottomRight,
            };
            
            utils.set_overlay_notification_position(steam_position);
            debug!("Overlay notification position set to {:?}", position);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Steam client not available"))
        }
    }

    pub async fn is_overlay_active(&self) -> bool {
        if !self.initialized {
            return false;
        }

        if let Some(ref client) = self.client {
            let utils = client.utils();
            utils.is_overlay_active()
        } else {
            false
        }
    }

    // Profile-specific overlay methods
    pub async fn show_profile_selector(&self) -> Result<()> {
        if !self.initialized {
            return Err(anyhow::anyhow!("Overlay manager not initialized"));
        }

        info!("Showing profile selector in overlay");

        if let Some(ref client) = self.client {
            let friends = client.friends();
            // Activate overlay with custom URL for profile selector
            friends.activate_game_overlay_to_web_page("controller-remapper://profile-selector");
            info!("Profile selector overlay activated");
            Ok(())
        } else {
            Err(anyhow::anyhow!("Steam client not available"))
        }
    }

    pub async fn show_profile_editor(&self, profile_id: &str) -> Result<()> {
        if !self.initialized {
            return Err(anyhow::anyhow!("Overlay manager not initialized"));
        }

        info!("Showing profile editor in overlay for: {}", profile_id);

        if let Some(ref client) = self.client {
            let friends = client.friends();
            // Activate overlay with custom URL for profile editor
            let url = format!("controller-remapper://profile-editor/{}", profile_id);
            friends.activate_game_overlay_to_web_page(&url);
            info!("Profile editor overlay activated for: {}", profile_id);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Steam client not available"))
        }
    }

    pub async fn notify_profile_loaded(&self, profile_name: &str) -> Result<()> {
        if !self.initialized {
            return Err(anyhow::anyhow!("Overlay manager not initialized"));
        }

        info!("Sending notification: Profile '{}' loaded", profile_name);

        if let Some(ref client) = self.client {
            let friends = client.friends();
            // Use Steam overlay notification system
            let message = format!("Profile '{}' loaded successfully", profile_name);
            // Note: steamworks-rs doesn't have a direct notification API,
            // this would typically be handled through the overlay UI
            debug!("Profile load notification: {}", message);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Steam client not available"))
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum OverlayPosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl Default for OverlayManager {
    fn default() -> Self {
        Self::new()
    }
}
