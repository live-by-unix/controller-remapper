use crate::profiles::profile::Profile;
use anyhow::{Context, Result};
use log::{debug, error, info, warn};
use std::path::PathBuf;
use steamworks::SingleClient;

#[derive(Debug, Clone)]
pub struct RemoteFile {
    pub name: String,
    pub size: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

pub struct RemoteStorageManager {
    app_id: u32,
    initialized: bool,
    client: Option<SingleClient>,
    cloud_enabled: bool,
}

impl RemoteStorageManager {
    pub fn new() -> Self {
        Self {
            app_id: 0,
            initialized: false,
            client: None,
            cloud_enabled: false,
        }
    }

    pub async fn initialize(&mut self, app_id: u32) -> Result<()> {
        self.app_id = app_id;
        self.initialized = true;
        
        // Check if cloud is enabled
        self.cloud_enabled = self.is_cloud_enabled();
        
        info!("Remote storage manager initialized for app ID: {} (cloud: {})", 
              app_id, self.cloud_enabled);
        Ok(())
    }

    pub async fn initialize_with_client(&mut self, client: SingleClient, app_id: u32) -> Result<()> {
        self.app_id = app_id;
        self.client = Some(client);
        self.initialized = true;
        
        // Check if cloud is enabled
        self.cloud_enabled = self.is_cloud_enabled();
        
        info!("Remote storage manager initialized with Steam client for app ID: {} (cloud: {})", 
              app_id, self.cloud_enabled);
        Ok(())
    }

    pub async fn shutdown(&mut self) -> Result<()> {
        self.initialized = false;
        info!("Remote storage manager shut down");
        Ok(())
    }

    pub fn is_cloud_enabled(&self) -> bool {
        if !self.initialized {
            return false;
        }

        if let Some(ref client) = self.client {
            let remote_storage = client.remote_storage();
            remote_storage.is_cloud_enabled_for_account()
        } else {
            false
        }
    }

    pub async fn is_cloud_enabled_for_account(&self) -> bool {
        if !self.initialized {
            return false;
        }

        if let Some(ref client) = self.client {
            let remote_storage = client.remote_storage();
            remote_storage.is_cloud_enabled_for_account()
        } else {
            false
        }
    }

    pub async fn get_cloud_quota(&self) -> Result<(u64, u64)> {
        if !self.initialized {
            return Err(anyhow::anyhow!("Remote storage not initialized"));
        }

        debug!("Getting cloud quota");

        if let Some(ref client) = self.client {
            let remote_storage = client.remote_storage();
            let (total, available) = remote_storage.quota();
            Ok((total, available))
        } else {
            Ok((1_000_000_000, 500_000_000)) // Fallback values
        }
    }

    pub async fn file_write(&self, filename: &str, data: &[u8]) -> Result<bool> {
        if !self.initialized {
            return Err(anyhow::anyhow!("Remote storage not initialized"));
        }

        info!("Writing file to remote storage: {}", filename);

        if let Some(ref client) = self.client {
            let remote_storage = client.remote_storage();
            
            // Write file to cloud
            let success = remote_storage.file_write(filename, data);
            
            if success {
                info!("File written to cloud: {}", filename);
            } else {
                warn!("Failed to write file to cloud: {}", filename);
            }
            
            Ok(success)
        } else {
            Err(anyhow::anyhow!("Steam client not available"))
        }
    }

    pub async fn file_read(&self, filename: &str) -> Result<Vec<u8>> {
        if !self.initialized {
            return Err(anyhow::anyhow!("Remote storage not initialized"));
        }

        debug!("Reading file from remote storage: {}", filename);

        if let Some(ref client) = self.client {
            let remote_storage = client.remote_storage();
            
            if !remote_storage.file_exists(filename) {
                return Err(anyhow::anyhow!("File not found: {}", filename));
            }
            
            let size = remote_storage.file_size(filename) as usize;
            let mut buffer = vec![0u8; size];
            
            if remote_storage.file_read(filename, &mut buffer) {
                Ok(buffer)
            } else {
                Err(anyhow::anyhow!("Failed to read file: {}", filename))
            }
        } else {
            Err(anyhow::anyhow!("Steam client not available"))
        }
    }

    pub async fn file_exists(&self, filename: &str) -> bool {
        if !self.initialized {
            return false;
        }

        debug!("Checking if file exists in remote storage: {}", filename);

        if let Some(ref client) = self.client {
            let remote_storage = client.remote_storage();
            remote_storage.file_exists(filename)
        } else {
            false
        }
    }

    pub async fn file_delete(&self, filename: &str) -> Result<bool> {
        if !self.initialized {
            return Err(anyhow::anyhow!("Remote storage not initialized"));
        }

        info!("Deleting file from remote storage: {}", filename);

        if let Some(ref client) = self.client {
            let remote_storage = client.remote_storage();
            let success = remote_storage.file_delete(filename);
            
            if success {
                info!("File deleted from cloud: {}", filename);
            } else {
                warn!("Failed to delete file from cloud: {}", filename);
            }
            
            Ok(success)
        } else {
            Err(anyhow::anyhow!("Steam client not available"))
        }
    }

    pub async fn get_file_size(&self, filename: &str) -> Result<i32> {
        if !self.initialized {
            return Err(anyhow::anyhow!("Remote storage not initialized"));
        }

        debug!("Getting file size from remote storage: {}", filename);

        if let Some(ref client) = self.client {
            let remote_storage = client.remote_storage();
            Ok(remote_storage.file_size(filename) as i32)
        } else {
            Ok(0)
        }
    }

    pub async fn sync_cloud(&self) -> Result<()> {
        if !self.initialized {
            return Err(anyhow::anyhow!("Remote storage not initialized"));
        }

        info!("Syncing with Steam Cloud");

        if let Some(ref client) = self.client {
            let remote_storage = client.remote_storage();
            remote_storage.force_sync();
            info!("Cloud sync forced");
            Ok(())
        } else {
            Err(anyhow::anyhow!("Steam client not available"))
        }
    }

    pub async fn list_files(&self, pattern: &str) -> Result<Vec<RemoteFile>> {
        if !self.initialized {
            return Err(anyhow::anyhow!("Remote storage not initialized"));
        }

        debug!("Listing files in remote storage with pattern: {}", pattern);

        if let Some(ref client) = self.client {
            let remote_storage = client.remote_storage();
            let num_files = remote_storage.file_count();
            
            let mut files = Vec::new();
            for i in 0..num_files {
                if let Some(filename) = remote_storage.file_name(i) {
                    if filename.contains(pattern) || pattern == "*" {
                        files.push(RemoteFile {
                            name: filename.clone(),
                            size: remote_storage.file_size(&filename) as i64,
                            created_at: 0, // Would need additional API call
                            updated_at: 0, // Would need additional API call
                        });
                    }
                }
            }
            
            Ok(files)
        } else {
            Ok(vec![])
        }
    }

    // Profile-specific methods
    pub async fn save_profile_to_cloud(&self, profile: &Profile) -> Result<()> {
        let filename = format!("profiles/{}.json", profile.id);
        let json = profile.to_json()
            .context("Failed to serialize profile")?;
        
        self.file_write(&filename, json.as_bytes()).await?;
        info!("Profile '{}' saved to cloud", profile.name);
        Ok(())
    }

    pub async fn load_profile_from_cloud(&self, profile_id: &str) -> Result<Profile> {
        let filename = format!("profiles/{}.json", profile_id);
        let data = self.file_read(&filename).await?;
        let json = String::from_utf8(data)
            .context("Failed to decode profile data")?;
        
        let profile = Profile::from_json(&json)
            .context("Failed to parse profile")?;
        
        info!("Profile '{}' loaded from cloud", profile.name);
        Ok(profile)
    }

    pub async fn delete_profile_from_cloud(&self, profile_id: &str) -> Result<()> {
        let filename = format!("profiles/{}.json", profile_id);
        self.file_delete(&filename).await?;
        info!("Profile {} deleted from cloud", profile_id);
        Ok(())
    }

    pub async fn list_cloud_profiles(&self) -> Result<Vec<String>> {
        self.list_files("profiles/*.json").await?
            .iter()
            .map(|f| {
                f.name.strip_prefix("profiles/")
                    .unwrap_or(&f.name)
                    .strip_suffix(".json")
                    .unwrap_or(&f.name)
                    .to_string()
            })
            .collect()
    }
}

impl Default for RemoteStorageManager {
    fn default() -> Self {
        Self::new()
    }
}
