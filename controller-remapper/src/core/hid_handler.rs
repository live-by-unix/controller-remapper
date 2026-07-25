use crate::core::controller_types::{ControllerInfo, InputEvent};
use anyhow::{Context, Result};
use hidapi::{HidApi, HidDevice};
use log::{debug, error, info, warn};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct HidHandler {
    api: Arc<Mutex<HidApi>>,
    devices: Arc<Mutex<Vec<HidDevice>>>,
    device_info_cache: Arc<Mutex<HashMap<String, ControllerInfo>>>,
}

impl HidHandler {
    pub fn new() -> Result<Self> {
        let api = HidApi::new().context("Failed to initialize HID API")?;
        
        Ok(Self {
            api: Arc::new(Mutex::new(api)),
            devices: Arc::new(Mutex::new(vec![])),
            device_info_cache: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    pub async fn open_device(&self, vendor_id: u16, product_id: u16) -> Result<HidDevice> {
        let api = self.api.lock().await;
        
        let device = api
            .open(vendor_id, product_id)
            .context(format!("Failed to open HID device {:04x}:{:04x}", vendor_id, product_id))?;
        
        info!("Opened HID device {:04x}:{:04x}", vendor_id, product_id);
        
        Ok(device)
    }

    pub async fn open_device_by_path(&self, path: &str) -> Result<HidDevice> {
        let api = self.api.lock().await;
        
        let device = api
            .open_path(path.as_ref())
            .context(format!("Failed to open HID device at path: {}", path))?;
        
        info!("Opened HID device at path: {}", path);
        
        Ok(device)
    }

    pub async fn read_input(&self, device: &mut HidDevice, buffer: &mut [u8]) -> Result<usize> {
        match device.read_timeout(buffer, 100) {
            Ok(len) => Ok(len),
            Err(e) => {
                if e.kind() == std::io::ErrorKind::TimedOut {
                    Ok(0)
                } else {
                    Err(e.into())
                }
            }
        }
    }

    pub async fn write_output(&self, device: &mut HidDevice, data: &[u8]) -> Result<()> {
        device.write(data).context("Failed to write to HID device")?;
        Ok(())
    }

    pub async fn list_devices(&self) -> Result<Vec<ControllerInfo>> {
        let api = self.api.lock().await;
        let mut devices = vec![];
        
        for device_info in api.device_list() {
            let device_id = format!("{}:{}", device_info.vendor_id(), device_info.product_id());
            
            let info = ControllerInfo {
                id: device_id.clone(),
                name: self.get_device_name(&device_info),
                vendor_id: device_info.vendor_id(),
                product_id: device_info.product_id(),
                is_bluetooth: self.is_bluetooth_device(device_info.vendor_id(), device_info.product_id()),
                connected: true,
                path: device_info.path().to_string_lossy().to_string(),
            };
            
            debug!("Found HID device: {} ({:04x}:{:04x})", info.name, info.vendor_id, info.product_id);
            devices.push(info);
            
            // Cache the device info
            self.device_info_cache.lock().await.insert(device_id, info);
        }
        
        Ok(devices)
    }

    fn get_device_name(&self, device_info: &hidapi::DeviceInfo) -> String {
        // Try to get the product string first
        if let Some(product_string) = device_info.product_string() {
            if !product_string.is_empty() {
                return product_string.to_string();
            }
        }
        
        // Fallback to known controller database
        let vid = device_info.vendor_id();
        let pid = device_info.product_id();
        
        // Known controller mappings
        match (vid, pid) {
            // Xbox controllers
            (0x045e, 0x028e) => "Xbox 360 Controller".to_string(),
            (0x045e, 0x02d1) => "Xbox One Controller".to_string(),
            (0x045e, 0x02dd) => "Xbox One S Controller".to_string(),
            (0x045e, 0x0b12) => "Xbox Series X|S Controller".to_string(),
            (0x045e, 0x0719) => "Xbox 360 Wireless Receiver".to_string(),
            
            // PlayStation controllers
            (0x054c, 0x0268) => "DualShock 3".to_string(),
            (0x054c, 0x05c4) => "DualShock 4".to_string(),
            (0x054c, 0x0ce6) => "DualSense".to_string(),
            (0x054c, 0x0df2) => "DualSense Edge".to_string(),
            
            // Nintendo controllers
            (0x057e, 0x2009) => "Switch Pro Controller".to_string(),
            (0x057e, 0x2006) => "Joy-Con (L)".to_string(),
            (0x057e, 0x2007) => "Joy-Con (R)".to_string(),
            
            // Generic gamepad
            _ => format!("Generic Controller ({:04x}:{:04x})", vid, pid),
        }
    }

    fn is_bluetooth_device(&self, vendor_id: u16, product_id: u16) -> bool {
        // Known Bluetooth controller VID/PID combinations
        match (vendor_id, product_id) {
            // PlayStation Bluetooth
            (0x054c, 0x0268) => true,
            (0x054c, 0x05c4) => true,
            (0x054c, 0x0ce6) => true,
            
            // Nintendo Switch Bluetooth
            (0x057e, 0x2006) => true,
            (0x057e, 0x2007) => true,
            
            _ => false,
        }
    }

    pub async fn set_non_blocking(&self, device: &mut HidDevice, non_blocking: bool) -> Result<()> {
        device.set_blocking_mode(!non_blocking)
            .context("Failed to set device blocking mode")?;
        Ok(())
    }

    pub async fn get_manufacturer_string(&self, device: &HidDevice) -> Result<String> {
        let manufacturer = device.get_manufacturer_string()
            .context("Failed to get manufacturer string")?;
        Ok(manufacturer.unwrap_or_else(|| "Unknown".to_string()))
    }

    pub async fn get_product_string(&self, device: &HidDevice) -> Result<String> {
        let product = device.get_product_string()
            .context("Failed to get product string")?;
        Ok(product.unwrap_or_else(|| "Unknown".to_string()))
    }

    pub async fn get_serial_number(&self, device: &HidDevice) -> Result<String> {
        let serial = device.get_serial_number_string()
            .context("Failed to get serial number")?;
        Ok(serial.unwrap_or_else(|| "Unknown".to_string()))
    }

    pub async fn get_device_info(&self, vendor_id: u16, product_id: u16) -> Option<ControllerInfo> {
        let cache = self.device_info_cache.lock().await;
        cache.get(&format!("{}:{}", vendor_id, product_id)).cloned()
    }

    pub async fn refresh_devices(&self) -> Result<Vec<ControllerInfo>> {
        debug!("Refreshing HID device list");
        self.list_devices().await
    }

    pub async fn is_game_controller(&self, vendor_id: u16, product_id: u16) -> bool {
        // Check if device is likely a game controller based on known VIDs
        let vid = vendor_id;
        
        matches!(vid,
            0x045e | // Microsoft (Xbox)
            0x054c | // Sony (PlayStation)
            0x057e | // Nintendo
            0x0079 | // DragonRise
            0x046d | // Logitech
            0x056e | // Elecom
            0x044f | // Thrustmaster
            0x0e6f | // Hori
            0x146b | // BigBen
            0x0f0d | // Hori (alternative)
            0x20d6 | // PowerA
            0x2dc8 | // 8BitDo
            0x0b05 | // ASUS
            _ => true // Assume it's a game controller if we can't identify it
        )
    }
}

impl Default for HidHandler {
    fn default() -> Self {
        Self::new().expect("Failed to create HidHandler")
    }
}
