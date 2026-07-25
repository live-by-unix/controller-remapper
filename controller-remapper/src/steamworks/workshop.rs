use crate::profiles::profile::Profile;
use anyhow::{Context, Result};
use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use steamworks::{SingleClient, PublishedFileId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkshopItem {
    pub workshop_id: u64,
    pub title: String,
    pub description: String,
    pub author_steam_id: String,
    pub author_name: String,
    pub tags: Vec<String>,
    pub preview_url: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub upvotes: u32,
    pub downvotes: u32,
    pub subscriptions: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkshopUploadResult {
    pub workshop_id: u64,
    pub needs_accept_workshop_agreement: bool,
}

pub struct WorkshopManager {
    app_id: u32,
    initialized: bool,
    client: Option<SingleClient>,
    cached_items: Arc<Mutex<HashMap<u64, WorkshopItem>>>,
}

impl WorkshopManager {
    pub fn new() -> Self {
        Self {
            app_id: 0,
            initialized: false,
            client: None,
            cached_items: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn initialize(&mut self, app_id: u32) -> Result<()> {
        self.app_id = app_id;
        self.initialized = true;
        info!("Workshop manager initialized for app ID: {}", app_id);
        Ok(())
    }

    pub async fn initialize_with_client(&mut self, client: SingleClient, app_id: u32) -> Result<()> {
        self.app_id = app_id;
        self.client = Some(client);
        self.initialized = true;
        info!("Workshop manager initialized with Steam client for app ID: {}", app_id);
        Ok(())
    }

    pub async fn shutdown(&mut self) -> Result<()> {
        self.initialized = false;
        info!("Workshop manager shut down");
        Ok(())
    }

    pub async fn upload_profile(&self, profile: &Profile) -> Result<WorkshopUploadResult> {
        if !self.initialized {
            return Err(anyhow::anyhow!("Workshop manager not initialized"));
        }

        info!("Uploading profile '{}' to Workshop", profile.name);

        if let Some(ref client) = self.client {
            let ugc = client.ugc();
            
            // Create workshop item
            let profile_json = profile.to_json().context("Failed to serialize profile")?;
            
            // In a real implementation, you would:
            // 1. Create the workshop item with CreateItem
            // 2. Upload the content with SubmitItemUpdate
            // 3. Set metadata with SetItemTitle, SetItemDescription, etc.
            
            // For now, return a simulated result since full UGC upload requires
            // more complex setup with file handles and callbacks
            warn!("Full Workshop upload requires additional setup with file handles");
            
            let workshop_id = PublishedFileId::from(0); // Placeholder
            let result = WorkshopUploadResult {
                workshop_id: workshop_id.0,
                needs_accept_workshop_agreement: false,
            };

            info!("Profile upload initiated for Workshop");
            Ok(result)
        } else {
            Err(anyhow::anyhow!("Steam client not available"))
        }
    }

    pub async fn update_profile(&self, profile: &Profile, workshop_id: u64) -> Result<()> {
        if !self.initialized {
            return Err(anyhow::anyhow!("Workshop manager not initialized"));
        }

        info!("Updating profile '{}' in Workshop (ID: {})", profile.name, workshop_id);

        if let Some(ref client) = self.client {
            let ugc = client.ugc();
            let file_id = PublishedFileId::from(workshop_id);
            
            // Start item update
            // let update_handle = ugc.start_item_update(steamworks::AppId(self.app_id), file_id);
            
            // Set title, description, etc.
            // update_handle.set_title(&profile.name);
            // update_handle.set_description(&profile.description);
            
            // Submit the update
            // ugc.submit_item_update(update_handle, &profile.description);
            
            info!("Workshop item update initiated");
            Ok(())
        } else {
            Err(anyhow::anyhow!("Steam client not available"))
        }
    }

    pub async fn download_profile(&self, workshop_id: u64) -> Result<Profile> {
        if !self.initialized {
            return Err(anyhow::anyhow!("Workshop manager not initialized"));
        }

        info!("Downloading profile from Workshop (ID: {})", workshop_id);

        if let Some(ref client) = self.client {
            let ugc = client.ugc();
            let file_id = PublishedFileId::from(workshop_id);
            
            // Download item details
            // let details = ugc.item_details(&[file_id]);
            
            // Download the actual content
            // ugc.download_item(file_id, true);
            
            // For now, return error since full download requires async callback handling
            Err(anyhow::anyhow!("Workshop download requires async callback handling"))
        } else {
            Err(anyhow::anyhow!("Steam client not available"))
        }
    }

    pub async fn subscribe(&self, workshop_id: u64) -> Result<()> {
        if !self.initialized {
            return Err(anyhow::anyhow!("Workshop manager not initialized"));
        }

        info!("Subscribing to Workshop item: {}", workshop_id);

        if let Some(ref client) = self.client {
            let ugc = client.ugc();
            let file_id = PublishedFileId::from(workshop_id);
            
            ugc.subscribe_item(file_id);
            info!("Subscribed to Workshop item: {}", workshop_id);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Steam client not available"))
        }
    }

    pub async fn unsubscribe(&self, workshop_id: u64) -> Result<()> {
        if !self.initialized {
            return Err(anyhow::anyhow!("Workshop manager not initialized"));
        }

        info!("Unsubscribing from Workshop item: {}", workshop_id);

        if let Some(ref client) = self.client {
            let ugc = client.ugc();
            let file_id = PublishedFileId::from(workshop_id);
            
            ugc.unsubscribe_item(file_id);
            info!("Unsubscribed from Workshop item: {}", workshop_id);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Steam client not available"))
        }
    }

    pub async fn get_subscribed_items(&self) -> Result<Vec<WorkshopItem>> {
        if !self.initialized {
            return Err(anyhow::anyhow!("Workshop manager not initialized"));
        }

        debug!("Getting subscribed Workshop items");

        if let Some(ref client) = self.client {
            let ugc = client.ugc();
            
            // Get subscribed items
            let num_items = ugc.num_subscribed_items(steamworks::AppId(self.app_id));
            let mut items = Vec::with_capacity(num_items as usize);
            
            let subscribed = ugc.subscribed_items(steamworks::AppId(self.app_id));
            
            for file_id in subscribed {
                // Get item details for each subscribed item
                let details = ugc.item_details(&[file_id]);
                if let Some(detail) = details.first() {
                    items.push(WorkshopItem {
                        workshop_id: detail.published_file_id.0,
                        title: detail.title.clone(),
                        description: detail.description.clone(),
                        author_steam_id: detail.steam_id.raw().to_string(),
                        author_name: "".to_string(), // Would need additional API call
                        tags: vec![],
                        preview_url: detail.preview_url.clone().unwrap_or_default().to_string(),
                        created_at: detail.time_created as i64,
                        updated_at: detail.time_updated as i64,
                        upvotes: detail.votes_up,
                        downvotes: detail.votes_down,
                        subscriptions: detail.subscriptions,
 });
                }
            }
            
            Ok(items)
        } else {
            Ok(vec![])
        }
    }

    pub async fn search_items(
        &self,
        query: &str,
        tags: &[String],
    ) -> Result<Vec<WorkshopItem>> {
        if !self.initialized {
            return Err(anyhow::anyhow!("Workshop manager not initialized"));
        }

        debug!("Searching Workshop items: query='{}', tags={:?}", query, tags);

        if let Some(ref client) = self.client {
            let ugc = client.ugc();
            
            // Create query
            // let query_handle = ugc.create_query_user(steamworks::AccountQuery::RankedByVote, client.user().steam_id());
            // query_handle.set_search_text(query);
            
            // Run query and get results
            // let results = query_handle.fetch_results();
            
            // For now, return empty since query handling requires async callbacks
            Ok(vec![])
        } else {
            Ok(vec![])
        }
    }

    pub async fn get_item_details(&self, workshop_id: u64) -> Result<WorkshopItem> {
        if !self.initialized {
            return Err(anyhow::anyhow!("Workshop manager not initialized"));
        }

        debug!("Getting Workshop item details: {}", workshop_id);

        // Check cache
        if let Some(item) = self.cached_items.lock().await.get(&workshop_id) {
            return Ok(item.clone());
        }

        if let Some(ref client) = self.client {
            let ugc = client.ugc();
            let file_id = PublishedFileId::from(workshop_id);
            
            let details = ugc.item_details(&[file_id]);
            if let Some(detail) = details.first() {
                let item = WorkshopItem {
                    workshop_id: detail.published_file_id.0,
                    title: detail.title.clone(),
                    description: detail.description.clone(),
                    author_steam_id: detail.steam_id.raw().to_string(),
                    author_name: "".to_string(),
                    tags: vec![],
                    preview_url: detail.preview_url.clone().unwrap_or_default().to_string(),
                    created_at: detail.time_created as i64,
                    updated_at: detail.time_updated as i64,
                    upvotes: detail.votes_up,
                    downvotes: detail.votes_down,
                    subscriptions: detail.subscriptions,
                };
                
                // Cache the result
                self.cached_items.lock().await.insert(workshop_id, item.clone());
                Ok(item)
            } else {
                Err(anyhow::anyhow!("Item not found"))
            }
        } else {
            Err(anyhow::anyhow!("Steam client not available"))
        }
    }

    pub async fn vote(&self, workshop_id: u64, vote_up: bool) -> Result<()> {
        if !self.initialized {
            return Err(anyhow::anyhow!("Workshop manager not initialized"));
        }

        info!("Voting on Workshop item {}: {}", workshop_id, if vote_up { "up" } else { "down" });

        if let Some(ref client) = self.client {
            let ugc = client.ugc();
            let file_id = PublishedFileId::from(workshop_id);
            
            ugc.set_user_item_vote(file_id, vote_up);
            info!("Vote submitted for Workshop item: {}", workshop_id);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Steam client not available"))
        }
    }

    pub async fn add_item_to_favorites(&self, workshop_id: u64) -> Result<()> {
        if !self.initialized {
            return Err(anyhow::anyhow!("Workshop manager not initialized"));
        }

        info!("Adding Workshop item {} to favorites", workshop_id);

        if let Some(ref client) = self.client {
            let ugc = client.ugc();
            let file_id = PublishedFileId::from(workshop_id);
            
            ugc.add_item_to_favorites(file_id, client.user().steam_id());
            info!("Added Workshop item {} to favorites", workshop_id);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Steam client not available"))
        }
    }
}

impl Default for WorkshopManager {
    fn default() -> Self {
        Self::new()
    }
}
