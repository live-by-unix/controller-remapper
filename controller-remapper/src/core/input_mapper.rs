use crate::core::controller_types::{AxisMapping, ButtonMapping, ControllerAxis, ControllerButton, InputEvent, KeyboardKey, OutputAction, MouseAction};
use anyhow::Result;
use log::{debug, trace};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct InputMapper {
    button_mappings: HashMap<ControllerButton, ButtonMapping>,
    axis_mappings: HashMap<ControllerAxis, AxisMapping>,
    active_keys: Arc<Mutex<HashMap<KeyboardKey, bool>>>,
    active_mouse_buttons: Arc<Mutex<HashMap<MouseAction, bool>>>,
}

impl InputMapper {
    pub fn new() -> Self {
        Self {
            button_mappings: HashMap::new(),
            axis_mappings: HashMap::new(),
            active_keys: Arc::new(Mutex::new(HashMap::new())),
            active_mouse_buttons: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn set_button_mapping(&mut self, mapping: ButtonMapping) {
        debug!("Setting button mapping: {:?} -> {:?}", mapping.controller_button, mapping.output_action);
        self.button_mappings.insert(mapping.controller_button, mapping);
    }

    pub fn set_axis_mapping(&mut self, mapping: AxisMapping) {
        debug!("Setting axis mapping: {:?} -> {:?} / {:?}", mapping.controller_axis, mapping.output_positive, mapping.output_negative);
        self.axis_mappings.insert(mapping.controller_axis, mapping);
    }

    pub fn remove_button_mapping(&mut self, button: ControllerButton) {
        debug!("Removing button mapping: {:?}", button);
        self.button_mappings.remove(&button);
    }

    pub fn remove_axis_mapping(&mut self, axis: ControllerAxis) {
        debug!("Removing axis mapping: {:?}", axis);
        self.axis_mappings.remove(&axis);
    }

    pub fn get_button_mapping(&self, button: ControllerButton) -> Option<&ButtonMapping> {
        self.button_mappings.get(&button)
    }

    pub fn get_axis_mapping(&self, axis: ControllerAxis) -> Option<&AxisMapping> {
        self.axis_mappings.get(&axis)
    }

    pub fn get_all_button_mappings(&self) -> Vec<ButtonMapping> {
        self.button_mappings.values().cloned().collect()
    }

    pub fn get_all_axis_mappings(&self) -> Vec<AxisMapping> {
        self.axis_mappings.values().cloned().collect()
    }

    pub fn clear_mappings(&mut self) {
        debug!("Clearing all mappings");
        self.button_mappings.clear();
        self.axis_mappings.clear();
    }

    pub async fn process_button_event(&self, event: &InputEvent) -> Result<(Vec<KeyboardKey>, Vec<MouseAction>)> {
        let mut keys_to_press = vec![];
        let mut mouse_actions = vec![];
        
        if let Some(button) = event.button {
            if let Some(mapping) = self.button_mappings.get(&button) {
                if mapping.enabled {
                    if event.value > 0.5 {
                        // Button pressed
                        match mapping.output_action {
                            OutputAction::Keyboard(key) => {
                                keys_to_press.push(key);
                                *self.active_keys.lock().await.entry(key).or_insert(true) = true;
                                trace!("Button {:?} pressed, sending key {:?}", button, key);
                            }
                            OutputAction::Mouse(mouse_action) => {
                                mouse_actions.push(mouse_action);
                                *self.active_mouse_buttons.lock().await.entry(mouse_action).or_insert(true) = true;
                                trace!("Button {:?} pressed, sending mouse action {:?}", button, mouse_action);
                            }
                        }
                    } else {
                        // Button released
                        match mapping.output_action {
                            OutputAction::Keyboard(key) => {
                                *self.active_keys.lock().await.entry(key).or_insert(false) = false;
                                trace!("Button {:?} released, releasing key {:?}", button, key);
                            }
                            OutputAction::Mouse(mouse_action) => {
                                *self.active_mouse_buttons.lock().await.entry(mouse_action).or_insert(false) = false;
                                trace!("Button {:?} released, releasing mouse action {:?}", button, mouse_action);
                            }
                        }
                    }
                }
            }
        }
        
        Ok((keys_to_press, mouse_actions))
    }

    pub async fn process_axis_event(&self, event: &InputEvent) -> Result<(Vec<KeyboardKey>, Vec<MouseAction>)> {
        let mut keys_to_press = vec![];
        let mut mouse_actions = vec![];
        
        if let Some(axis) = event.axis {
            if let Some(mapping) = self.axis_mappings.get(&axis) {
                if mapping.enabled {
                    let value = event.value;
                    let deadzone = mapping.deadzone;
                    let sensitivity = mapping.sensitivity;
                    
                    // Apply deadzone
                    let adjusted_value = if value.abs() < deadzone {
                        0.0
                    } else {
                        // Apply sensitivity
                        let normalized = (value.abs() - deadzone) / (1.0 - deadzone);
                        normalized * sensitivity * value.signum()
                    };
                    
                    // Release previous keys/mouse actions for this axis
                    if let Some(OutputAction::Keyboard(neg_key)) = mapping.output_negative {
                        *self.active_keys.lock().await.entry(neg_key).or_insert(false) = false;
                    }
                    if let Some(OutputAction::Keyboard(pos_key)) = mapping.output_positive {
                        *self.active_keys.lock().await.entry(pos_key).or_insert(false) = false;
                    }
                    if let Some(OutputAction::Mouse(neg_mouse)) = mapping.output_negative {
                        *self.active_mouse_buttons.lock().await.entry(neg_mouse).or_insert(false) = false;
                    }
                    if let Some(OutputAction::Mouse(pos_mouse)) = mapping.output_positive {
                        *self.active_mouse_buttons.lock().await.entry(pos_mouse).or_insert(false) = false;
                    }
                    
                    // Press new key/mouse action based on direction
                    if adjusted_value > 0.1 {
                        if let Some(OutputAction::Keyboard(key)) = mapping.output_positive {
                            keys_to_press.push(key);
                            *self.active_keys.lock().await.entry(key).or_insert(true) = true;
                            trace!("Axis {:?} positive, sending key {:?}", axis, key);
                        }
                        if let Some(OutputAction::Mouse(mouse_action)) = mapping.output_positive {
                            mouse_actions.push(mouse_action);
                            *self.active_mouse_buttons.lock().await.entry(mouse_action).or_insert(true) = true;
                            trace!("Axis {:?} positive, sending mouse action {:?}", axis, mouse_action);
                        }
                    } else if adjusted_value < -0.1 {
                        if let Some(OutputAction::Keyboard(key)) = mapping.output_negative {
                            keys_to_press.push(key);
                            *self.active_keys.lock().await.entry(key).or_insert(true) = true;
                            trace!("Axis {:?} negative, sending key {:?}", axis, key);
                        }
                        if let Some(OutputAction::Mouse(mouse_action)) = mapping.output_negative {
                            mouse_actions.push(mouse_action);
                            *self.active_mouse_buttons.lock().await.entry(mouse_action).or_insert(true) = true;
                            trace!("Axis {:?} negative, sending mouse action {:?}", axis, mouse_action);
                        }
                    }
                }
            }
        }
        
        Ok((keys_to_press, mouse_actions))
    }

    pub async fn process_event(&self, event: &InputEvent) -> Result<(Vec<KeyboardKey>, Vec<MouseAction>)> {
        let mut keys = vec![];
        let mut mouse_actions = vec![];
        
        if event.button.is_some() {
            let (k, m) = self.process_button_event(event).await?;
            keys.extend(k);
            mouse_actions.extend(m);
        }
        
        if event.axis.is_some() {
            let (k, m) = self.process_axis_event(event).await?;
            keys.extend(k);
            mouse_actions.extend(m);
        }
        
        Ok((keys, mouse_actions))
    }

    pub async fn get_active_keys(&self) -> Vec<KeyboardKey> {
        self.active_keys
            .lock()
            .await
            .iter()
            .filter(|(_, &active)| active)
            .map(|(&key, _)| key)
            .collect()
    }

    pub async fn get_active_mouse_actions(&self) -> Vec<MouseAction> {
        self.active_mouse_buttons
            .lock()
            .await
            .iter()
            .filter(|(_, &active)| active)
            .map(|(&action, _)| action)
            .collect()
    }

    pub async fn release_all_keys(&self) -> Result<()> {
        debug!("Releasing all keys");
        *self.active_keys.lock().await = HashMap::new();
        *self.active_mouse_buttons.lock().await = HashMap::new();
        Ok(())
    }

    pub fn from_mappings(
        button_mappings: Vec<ButtonMapping>,
        axis_mappings: Vec<AxisMapping>,
    ) -> Self {
        let mut mapper = Self::new();
        
        for mapping in button_mappings {
            mapper.set_button_mapping(mapping);
        }
        
        for mapping in axis_mappings {
            mapper.set_axis_mapping(mapping);
        }
        
        mapper
    }
}

impl Default for InputMapper {
    fn default() -> Self {
        Self::new()
    }
}
