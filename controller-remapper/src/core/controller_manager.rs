use crate::core::bluetooth_detector::BluetoothDetector;
use crate::core::controller_types::{ControllerInfo, InputEvent};
use crate::core::hid_handler::HidHandler;
use crate::core::input_mapper::InputMapper;
use anyhow::{Context, Result};
use log::{debug, error, info, warn};
use sdl2::controller::{GameController, GameControllerSubsystem};
use sdl2::event::Event;
use sdl2::EventPump;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::Mutex;
use tokio::time::{interval, Duration};

pub struct ControllerManager {
    bluetooth_detector: BluetoothDetector,
    hid_handler: HidHandler,
    controllers: Arc<Mutex<HashMap<String, ControllerInfo>>>,
    input_mapper: Arc<Mutex<InputMapper>>,
    sdl_controller: Arc<Mutex<Option<GameController>>>,
    running: Arc<Mutex<bool>>,
}

impl ControllerManager {
    pub fn new() -> Self {
        Self {
            bluetooth_detector: BluetoothDetector::new(),
            hid_handler: HidHandler::new().expect("Failed to create HID handler"),
            controllers: Arc::new(Mutex::new(HashMap::new())),
            input_mapper: Arc::new(Mutex::new(InputMapper::new())),
            sdl_controller: Arc::new(Mutex::new(None)),
            running: Arc::new(Mutex::new(false)),
        }
    }

    pub async fn start_detection(&mut self) -> Result<()> {
        info!("Starting controller detection");
        *self.running.lock().await = true;
        
        // Initialize SDL2
        let sdl_context = sdl2::init().context("Failed to initialize SDL2")?;
        let controller_subsystem = sdl_context.game_controller().context("Failed to initialize SDL2 controller subsystem")?;
        
        // Scan for controllers initially
        self.scan_controllers(&controller_subsystem).await?;
        
        // Start continuous detection loop
        let running = self.running.clone();
        let controllers = self.controllers.clone();
        let bluetooth_detector = BluetoothDetector::new();
        let controller_subsystem = Arc::new(controller_subsystem);
        
        tokio::spawn(async move {
            let mut scan_interval = interval(Duration::from_secs(5));
            
            while *running.lock().await {
                scan_interval.tick().await;
                
                debug!("Running periodic controller scan");
                if let Err(e) = Self::scan_controllers_internal(
                    &controllers,
                    &bluetooth_detector,
                    &controller_subsystem,
                ).await {
                    error!("Controller scan error: {}", e);
                }
            }
        });
        
        Ok(())
    }

    pub async fn stop_detection(&self) {
        info!("Stopping controller detection");
        *self.running.lock().await = false;
    }

    async fn scan_controllers(&self, controller_subsystem: &GameControllerSubsystem) -> Result<()> {
        Self::scan_controllers_internal(
            &self.controllers,
            &self.bluetooth_detector,
            controller_subsystem,
        ).await
    }

    async fn scan_controllers_internal(
        controllers: &Arc<Mutex<HashMap<String, ControllerInfo>>>,
        bluetooth_detector: &BluetoothDetector,
        controller_subsystem: &GameControllerSubsystem,
    ) -> Result<()> {
        // Scan Bluetooth controllers
        if let Ok(bt_controllers) = bluetooth_detector.scan().await {
            for controller in bt_controllers {
                debug!("Found Bluetooth controller: {}", controller.name);
                controllers.lock().await.insert(controller.id.clone(), controller);
            }
        }
        
        // Scan SDL2 controllers
        let num_controllers = controller_subsystem.num_joysticks().unwrap_or(0);
        debug!("SDL2 found {} joystick(s)", num_controllers);
        
        for i in 0..num_controllers {
            if controller_subsystem.is_game_controller(i) {
                if let Some(controller) = controller_subsystem.open(i) {
                    let name = controller.name();
                    let info = ControllerInfo {
                        id: format!("sdl_{}", i),
                        name: name.to_string(),
                        vendor_id: 0,
                        product_id: 0,
                        is_bluetooth: false,
                        connected: controller.attached(),
                        path: format!("sdl://{}", i),
                    };
                    
                    debug!("Found SDL2 controller: {}", info.name);
                    controllers.lock().await.insert(info.id.clone(), info);
                }
            }
        }
        
        Ok(())
    }

    pub async fn get_controllers(&self) -> Vec<ControllerInfo> {
        self.controllers.lock().await.values().cloned().collect()
    }

    pub async fn get_controller(&self, id: &str) -> Option<ControllerInfo> {
        self.controllers.lock().await.get(id).cloned()
    }

    pub async fn set_input_mapper(&self, mapper: InputMapper) {
        *self.input_mapper.lock().await = mapper;
    }

    pub async fn get_input_mapper(&self) -> InputMapper {
        self.input_mapper.lock().await.clone()
    }

    pub async fn process_sdl_events(&self) -> Result<Vec<InputEvent>> {
        let mut events = vec![];
        
        // This would normally be called from the SDL event loop
        // For now, return empty as the actual SDL event processing
        // happens in a separate thread
        
        Ok(events)
    }

    pub async fn map_button(&self, button: &str, key: &str) -> Result<()> {
        use crate::core::controller_types::{ControllerButton, KeyboardKey};
        
        let button = ControllerButton::from_str(button)
            .context(format!("Invalid controller button: {}", button))?;
        
        let key = KeyboardKey::from_str(key)
            .context(format!("Invalid keyboard key: {}", key))?;
        
        let mut mapper = self.input_mapper.lock().await;
        mapper.set_button_mapping(crate::core::controller_types::ButtonMapping {
            controller_button: button,
            keyboard_key: key,
            enabled: true,
        });
        
        Ok(())
    }

    pub async fn map_axis(&self, axis: &str, positive_key: Option<&str>, negative_key: Option<&str>) -> Result<()> {
        use crate::core::controller_types::{ControllerAxis, KeyboardKey};
        
        let axis = ControllerAxis::from_str(axis)
            .context(format!("Invalid controller axis: {}", axis))?;
        
        let positive_key = positive_key.map(|k| KeyboardKey::from_str(k))
            .transpose()
            .context("Invalid positive keyboard key")?;
        
        let negative_key = negative_key.map(|k| KeyboardKey::from_str(k))
            .transpose()
            .context("Invalid negative keyboard key")?;
        
        let mut mapper = self.input_mapper.lock().await;
        mapper.set_axis_mapping(crate::core::controller_types::AxisMapping {
            controller_axis: axis,
            keyboard_positive: positive_key,
            keyboard_negative: negative_key,
            deadzone: 0.1,
            sensitivity: 1.0,
            enabled: true,
        });
        
        Ok(())
    }

    pub async fn clear_mappings(&self) {
        self.input_mapper.lock().await.clear_mappings();
    }
}

// Add from_str methods for ControllerButton and ControllerAxis
impl ControllerButton {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "A" => Some(ControllerButton::A),
            "B" => Some(ControllerButton::B),
            "X" => Some(ControllerButton::X),
            "Y" => Some(ControllerButton::Y),
            "BACK" => Some(ControllerButton::Back),
            "GUIDE" => Some(ControllerButton::Guide),
            "START" => Some(ControllerButton::Start),
            "LEFTSTICK" => Some(ControllerButton::LeftStick),
            "RIGHTSTICK" => Some(ControllerButton::RightStick),
            "LEFTSHOULDER" => Some(ControllerButton::LeftShoulder),
            "RIGHTSHOULDER" => Some(ControllerButton::RightShoulder),
            "DPADUP" => Some(ControllerButton::DpadUp),
            "DPADDOWN" => Some(ControllerButton::DpadDown),
            "DPADLEFT" => Some(ControllerButton::DpadLeft),
            "DPADRIGHT" => Some(ControllerButton::DpadRight),
            _ => None,
        }
    }
}

impl ControllerAxis {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "LEFTX" => Some(ControllerAxis::LeftX),
            "LEFTY" => Some(ControllerAxis::LeftY),
            "RIGHTX" => Some(ControllerAxis::RightX),
            "RIGHTY" => Some(ControllerAxis::RightY),
            "LEFTTRIGGER" => Some(ControllerAxis::LeftTrigger),
            "RIGHTTRIGGER" => Some(ControllerAxis::RightTrigger),
            _ => None,
        }
    }
}
