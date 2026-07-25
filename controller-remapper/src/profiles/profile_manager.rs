use crate::profiles::profile::{Profile, ProfileMetadata};
use anyhow::{Context, Result};
use dirs;
use log::{debug, error, info, warn};
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct ProfileManager {
    profiles_dir: PathBuf,
    profiles: Arc<Mutex<Vec<Profile>>>,
}

impl ProfileManager {
    pub fn new() -> Self {
        let profiles_dir = Self::get_profiles_dir();
        
        // Create profiles directory if it doesn't exist
        if !profiles_dir.exists() {
            fs::create_dir_all(&profiles_dir).expect("Failed to create profiles directory");
            info!("Created profiles directory: {:?}", profiles_dir);
        }
        
        Self {
            profiles_dir,
            profiles: Arc::new(Mutex::new(vec![])),
        }
    }

    fn get_profiles_dir() -> PathBuf {
        let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("controller-remapper");
        path.push("profiles");
        path
    }

    pub async fn load_all_profiles(&self) -> Result<Vec<ProfileMetadata>> {
        debug!("Loading all profiles from: {:?}", self.profiles_dir);
        
        let mut profiles = vec![];
        let mut metadata_list = vec![];
        
        // Read all JSON files from profiles directory
        let entries = fs::read_dir(&self.profiles_dir)
            .context("Failed to read profiles directory")?;
        
        for entry in entries {
            let entry = entry.context("Failed to read directory entry")?;
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                debug!("Loading profile from: {:?}", path);
                
                match self.load_profile(&path).await {
                    Ok(profile) => {
                        let metadata = profile.to_metadata(path.to_string_lossy().to_string());
                        metadata_list.push(metadata);
                        profiles.push(profile);
                    }
                    Err(e) => {
                        warn!("Failed to load profile {:?}: {}", path, e);
                    }
                }
            }
        }
        
        *self.profiles.lock().await = profiles;
        info!("Loaded {} profile(s)", metadata_list.len());
        
        Ok(metadata_list)
    }

    pub async fn load_profile(&self, path: &PathBuf) -> Result<Profile> {
        let content = fs::read_to_string(path)
            .context(format!("Failed to read profile file: {:?}", path))?;
        
        let profile = Profile::from_json(&content)
            .context(format!("Failed to parse profile JSON: {:?}", path))?;
        
        debug!("Loaded profile: {}", profile.name);
        Ok(profile)
    }

    pub async fn save_profile(&self, profile: &Profile) -> Result<String> {
        let filename = format!("{}.json", profile.id);
        let path = self.profiles_dir.join(&filename);
        
        let json = profile.to_json()
            .context("Failed to serialize profile to JSON")?;
        
        fs::write(&path, json)
            .context(format!("Failed to write profile to: {:?}", path))?;
        
        info!("Saved profile '{}' to: {:?}", profile.name, path);
        
        // Update cached profiles
        let mut profiles = self.profiles.lock().await;
        if let Some(existing) = profiles.iter().position(|p| p.id == profile.id) {
            profiles[existing] = profile.clone();
        } else {
            profiles.push(profile.clone());
        }
        
        Ok(path.to_string_lossy().to_string())
    }

    pub async fn delete_profile(&self, profile_id: &str) -> Result<()> {
        let filename = format!("{}.json", profile_id);
        let path = self.profiles_dir.join(&filename);
        
        if path.exists() {
            fs::remove_file(&path)
                .context(format!("Failed to delete profile: {:?}", path))?;
            
            info!("Deleted profile: {}", profile_id);
            
            // Remove from cache
            let mut profiles = self.profiles.lock().await;
            profiles.retain(|p| p.id != profile_id);
        }
        
        Ok(())
    }

    pub async fn get_profile(&self, profile_id: &str) -> Option<Profile> {
        let profiles = self.profiles.lock().await;
        profiles.iter().find(|p| p.id == profile_id).cloned()
    }

    pub async fn get_profile_by_name(&self, name: &str) -> Option<Profile> {
        let profiles = self.profiles.lock().await;
        profiles.iter().find(|p| p.name == name).cloned()
    }

    pub async fn get_profiles_for_game(&self, game_name: &str) -> Vec<Profile> {
        let profiles = self.profiles.lock().await;
        profiles
            .iter()
            .filter(|p| p.game_name == game_name)
            .cloned()
            .collect()
    }

    pub async fn get_all_profiles(&self) -> Vec<Profile> {
        self.profiles.lock().await.clone()
    }

    pub async fn create_profile(
        &self,
        name: &str,
        game_name: &str,
        author: &str,
    ) -> Result<Profile> {
        let profile = Profile::new(name, game_name, author);
        self.save_profile(&profile).await?;
        Ok(profile)
    }

    pub async fn update_profile(&self, profile: &Profile) -> Result<()> {
        self.save_profile(profile).await?;
        Ok(())
    }

    pub async fn duplicate_profile(&self, profile_id: &str, new_name: &str) -> Result<Profile> {
        let original = self.get_profile(profile_id).await
            .context("Profile not found")?;
        
        let mut new_profile = original.clone();
        new_profile.id = uuid::Uuid::new_v4().to_string();
        new_profile.name = new_name.to_string();
        new_profile.created_at = chrono::Utc::now();
        new_profile.updated_at = chrono::Utc::now();
        new_profile.steam_workshop_id = None;
        
        self.save_profile(&new_profile).await?;
        Ok(new_profile)
    }

    pub async fn import_profile(&self, json_content: &str) -> Result<Profile> {
        let mut profile = Profile::from_json(json_content)
            .context("Failed to parse profile JSON")?;
        
        // Generate new ID to avoid conflicts
        profile.id = uuid::Uuid::new_v4().to_string();
        profile.steam_workshop_id = None;
        
        self.save_profile(&profile).await?;
        Ok(profile)
    }

    pub async fn export_profile(&self, profile_id: &str) -> Result<String> {
        let profile = self.get_profile(profile_id).await
            .context("Profile not found")?;
        
        profile.to_json().context("Failed to serialize profile")
    }

    pub async fn search_profiles(&self, query: &str) -> Vec<ProfileMetadata> {
        let profiles = self.profiles.lock().await;
        let query_lower = query.to_lowercase();
        
        profiles
            .iter()
            .filter(|p| {
                p.name.to_lowercase().contains(&query_lower)
                    || p.game_name.to_lowercase().contains(&query_lower)
                    || p.description.to_lowercase().contains(&query_lower)
                    || p.tags.iter().any(|t| t.to_lowercase().contains(&query_lower))
            })
            .map(|p| {
                let filename = format!("{}.json", p.id);
                let path = self.profiles_dir.join(&filename);
                p.to_metadata(path.to_string_lossy().to_string())
            })
            .collect()
    }

    pub fn get_profiles_dir(&self) -> PathBuf {
        self.profiles_dir.clone()
    }
}

impl Default for ProfileManager {
    fn default() -> Self {
        Self::new()
    }
}
