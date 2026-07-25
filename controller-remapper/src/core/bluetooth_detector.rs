use crate::core::controller_types::ControllerInfo;
use anyhow::{Context, Result};
use log::{debug, info, warn};
use std::collections::HashMap;

#[cfg(target_os = "linux")]
use bluez::{
    management::{Management, ManagementCommand},
    session::Session,
};

#[cfg(target_os = "windows")]
use windows::{
    core::HSTRING,
    Win32::Gaming::Input::{Gamepad, GamepadVibration, GamepadButtons},
};

#[cfg(target_os = "macos")]
use corebluetooth::{
    central::CentralDelegate,
    peripheral::Peripheral,
};

pub struct BluetoothDetector {
    controllers: HashMap<String, ControllerInfo>,
}

impl BluetoothDetector {
    pub fn new() -> Self {
        Self {
            controllers: HashMap::new(),
        }
    }

    pub async fn scan(&mut self) -> Result<Vec<ControllerInfo>> {
        debug!("Starting Bluetooth controller scan");
        
        #[cfg(target_os = "linux")]
        {
            self.scan_linux().await
        }
        
        #[cfg(target_os = "windows")]
        {
            self.scan_windows()
        }
        
        #[cfg(target_os = "macos")]
        {
            self.scan_macos().await
        }
        
        #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
        {
            warn!("Bluetooth scanning not implemented for this platform");
            Ok(vec![])
        }
    }

    #[cfg(target_os = "linux")]
    async fn scan_linux(&mut self) -> Result<Vec<ControllerInfo>> {
        info!("Scanning for Bluetooth controllers on Linux using BlueZ");
        
        // Try to connect to BlueZ via D-Bus
        match Session::new().await {
            Ok(session) => {
                debug!("Connected to BlueZ session");
                
                // Get adapters
                let adapters = session.adapters().await?;
                debug!("Found {} Bluetooth adapter(s)", adapters.len());
                
                let mut found_controllers = vec![];
                
                for adapter in adapters {
                    debug!("Scanning adapter: {}", adapter.address()?);
                    
                    // Start discovery
                    if let Err(e) = adapter.start_discovery().await {
                        warn!("Failed to start discovery on {}: {}", adapter.address()?, e);
                        continue;
                    }
                    
                    // Wait for devices
                    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
                    
                    // Get devices
                    let devices = adapter.devices().await?;
                    debug!("Found {} devices on {}", devices.len(), adapter.address()?);
                    
                    for device in devices {
                        if let Some(name) = device.name().await? {
                            if self.is_controller(&name) {
                                let info = ControllerInfo {
                                    id: device.address().await?.to_string(),
                                    name: name.clone(),
                                    vendor_id: self.guess_vendor_id(&name),
                                    product_id: self.guess_product_id(&name),
                                    is_bluetooth: true,
                                    connected: device.connected().await?,
                                    path: device.object_path().to_string(),
                                };
                                
                                debug!("Found controller: {}", info.name);
                                found_controllers.push(info.clone());
                                self.controllers.insert(info.id.clone(), info);
                            }
                        }
                    }
                    
                    // Stop discovery
                    let _ = adapter.stop_discovery().await;
                }
                
                Ok(found_controllers)
            }
            Err(e) => {
                warn!("Failed to connect to BlueZ: {}. Falling back to HID scan.", e);
                self.scan_hid_fallback().await
            }
        }
    }

    #[cfg(target_os = "linux")]
    async fn scan_hid_fallback(&mut self) -> Result<Vec<ControllerInfo>> {
        debug!("Using HID fallback for controller detection");
        
        // Use hidapi to scan for HID devices
        let api = hidapi::HidApi::new().context("Failed to initialize HID API")?;
        
        let mut found_controllers = vec![];
        
        for device_info in api.device_list() {
            if self.is_game_controller(device_info.vendor_id(), device_info.product_id()) {
                let info = ControllerInfo {
                    id: format!("{}:{}", device_info.vendor_id(), device_info.product_id()),
                    name: device_info.product_string().unwrap_or("Unknown Controller").to_string(),
                    vendor_id: device_info.vendor_id(),
                    product_id: device_info.product_id(),
                    is_bluetooth: false,
                    connected: true,
                    path: device_info.path().to_string_lossy().to_string(),
                };
                
                debug!("Found HID controller: {}", info.name);
                found_controllers.push(info.clone());
                self.controllers.insert(info.id.clone(), info);
            }
        }
        
        Ok(found_controllers)
    }

    #[cfg(target_os = "windows")]
    fn scan_windows(&mut self) -> Result<Vec<ControllerInfo>> {
        info!("Scanning for controllers on Windows using Windows.Gaming.Input");
        
        let mut found_controllers = vec![];
        
        // Use Windows.Gaming.Input API
        unsafe {
            for i in 0..16 {
                if let Some(gamepad) = Gamepad::from_index(i) {
                    let info = ControllerInfo {
                        id: format!("windows_gamepad_{}", i),
                        name: format!("Windows Gamepad {}", i + 1),
                        vendor_id: 0,
                        product_id: 0,
                        is_bluetooth: false,
                        connected: true,
                        path: format!("windows://gamepad/{}", i),
                    };
                    
                    debug!("Found Windows gamepad: {}", info.name);
                    found_controllers.push(info.clone());
                    self.controllers.insert(info.id.clone(), info);
                }
            }
        }
        
        Ok(found_controllers)
    }

    #[cfg(target_os = "macos")]
    async fn scan_macos(&mut self) -> Result<Vec<ControllerInfo>> {
        info!("Scanning for Bluetooth controllers on macOS using CoreBluetooth");
        
        // CoreBluetooth implementation would go here
        // For now, use HID fallback
        self.scan_hid_fallback().await
    }

    #[cfg(target_os = "macos")]
    async fn scan_hid_fallback(&mut self) -> Result<Vec<ControllerInfo>> {
        debug!("Using HID fallback for controller detection on macOS");
        
        let api = hidapi::HidApi::new().context("Failed to initialize HID API")?;
        
        let mut found_controllers = vec![];
        
        for device_info in api.device_list() {
            if self.is_game_controller(device_info.vendor_id(), device_info.product_id()) {
                let info = ControllerInfo {
                    id: format!("{}:{}", device_info.vendor_id(), device_info.product_id()),
                    name: device_info.product_string().unwrap_or("Unknown Controller").to_string(),
                    vendor_id: device_info.vendor_id(),
                    product_id: device_info.product_id(),
                    is_bluetooth: false,
                    connected: true,
                    path: device_info.path().to_string_lossy().to_string(),
                };
                
                debug!("Found HID controller: {}", info.name);
                found_controllers.push(info.clone());
                self.controllers.insert(info.id.clone(), info);
            }
        }
        
        Ok(found_controllers)
    }

    fn is_controller(&self, name: &str) -> bool {
        let name_lower = name.to_lowercase();
        name_lower.contains("controller") ||
        name_lower.contains("gamepad") ||
        name_lower.contains("xbox") ||
        name_lower.contains("playstation") ||
        name_lower.contains("dualsense") ||
        name_lower.contains("dualshock") ||
        name_lower.contains("switch") ||
        name_lower.contains("pro controller") ||
        name_lower.contains("joy-con") ||
        name_lower.contains("steam")
    }

    fn is_game_controller(&self, vendor_id: u16, product_id: u16) -> bool {
        // Known controller vendor/product IDs
        match (vendor_id, product_id) {
            // Microsoft (Xbox)
            (0x045e, _) => true,
            // Sony (PlayStation)
            (0x054c, _) => true,
            // Nintendo
            (0x057e, _) => true,
            // Valve (Steam Controller)
            (0x28de, _) => true,
            // Other common controller vendors
            (0x046d, _) => true, // Logitech
            (0x0f0d, _) => true, // Hori
            (0x1532, _) => true, // Razer
            _ => false,
        }
    }

    fn guess_vendor_id(&self, name: &str) -> u16 {
        let name_lower = name.to_lowercase();
        if name_lower.contains("xbox") {
            0x045e // Microsoft
        } else if name_lower.contains("playstation") || name_lower.contains("dualsense") || name_lower.contains("dualshock") {
            0x054c // Sony
        } else if name_lower.contains("nintendo") || name_lower.contains("switch") {
            0x057e // Nintendo
        } else if name_lower.contains("steam") {
            0x28de // Valve
        } else {
            0x0000
        }
    }

    fn guess_product_id(&self, name: &str) -> u16 {
        let name_lower = name.to_lowercase();
        if name_lower.contains("xbox") {
            if name_lower.contains("series") {
                0x0b12 // Xbox Series X|S
            } else if name_lower.contains("one") {
                0x02d1 // Xbox One
            } else {
                0x028e // Xbox 360
            }
        } else if name_lower.contains("dualsense") {
            0x0ce6 // PS5 DualSense
        } else if name_lower.contains("dualshock") {
            0x09cc // PS4 DualShock
        } else if name_lower.contains("pro controller") {
            0x2009 // Switch Pro Controller
        } else {
            0x0000
        }
    }

    pub fn get_controllers(&self) -> Vec<ControllerInfo> {
        self.controllers.values().cloned().collect()
    }

    pub fn get_controller(&self, id: &str) -> Option<&ControllerInfo> {
        self.controllers.get(id)
    }
}

impl Default for BluetoothDetector {
    fn default() -> Self {
        Self::new()
    }
}
