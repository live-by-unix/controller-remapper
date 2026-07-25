use crate::core::controller_types::{AxisMapping, ButtonMapping};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub description: String,
    pub game_name: String,
    pub author: String,
    pub version: String,
    pub button_mappings: Vec<ButtonMapping>,
    pub axis_mappings: Vec<AxisMapping>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub tags: Vec<String>,
    pub is_public: bool,
    pub steam_workshop_id: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileMetadata {
    pub id: String,
    pub name: String,
    pub description: String,
    pub game_name: String,
    pub author: String,
    pub version: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub tags: Vec<String>,
    pub is_public: bool,
    pub steam_workshop_id: Option<u64>,
    pub local_path: String,
}

impl Profile {
    pub fn new(name: &str, game_name: &str, author: &str) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            description: String::new(),
            game_name: game_name.to_string(),
            author: author.to_string(),
            version: "1.0.0".to_string(),
            button_mappings: vec![],
            axis_mappings: vec![],
            created_at: now,
            updated_at: now,
            tags: vec![],
            is_public: false,
            steam_workshop_id: None,
        }
    }

    pub fn with_mappings(
        name: &str,
        game_name: &str,
        author: &str,
        button_mappings: Vec<ButtonMapping>,
        axis_mappings: Vec<AxisMapping>,
    ) -> Self {
        let mut profile = Self::new(name, game_name, author);
        profile.button_mappings = button_mappings;
        profile.axis_mappings = axis_mappings;
        profile
    }

    pub fn add_button_mapping(&mut self, mapping: ButtonMapping) {
        // Remove existing mapping for the same button
        self.button_mappings.retain(|m| m.controller_button != mapping.controller_button);
        self.button_mappings.push(mapping);
        self.updated_at = Utc::now();
    }

    pub fn add_axis_mapping(&mut self, mapping: AxisMapping) {
        // Remove existing mapping for the same axis
        self.axis_mappings.retain(|m| m.controller_axis != mapping.controller_axis);
        self.axis_mappings.push(mapping);
        self.updated_at = Utc::now();
    }

    pub fn remove_button_mapping(&mut self, button: &crate::core::controller_types::ControllerButton) {
        self.button_mappings.retain(|m| &m.controller_button != button);
        self.updated_at = Utc::now();
    }

    pub fn remove_axis_mapping(&mut self, axis: &crate::core::controller_types::ControllerAxis) {
        self.axis_mappings.retain(|m| &m.controller_axis != axis);
        self.updated_at = Utc::now();
    }

    pub fn clear_mappings(&mut self) {
        self.button_mappings.clear();
        self.axis_mappings.clear();
        self.updated_at = Utc::now();
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    pub fn to_metadata(&self, local_path: String) -> ProfileMetadata {
        ProfileMetadata {
            id: self.id.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
            game_name: self.game_name.clone(),
            author: self.author.clone(),
            version: self.version.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
            tags: self.tags.clone(),
            is_public: self.is_public,
            steam_workshop_id: self.steam_workshop_id,
            local_path,
        }
    }
}

impl Default for Profile {
    fn default() -> Self {
        Self::new("Default Profile", "Unknown Game", "Unknown Author")
    }
}
