use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const SDL_CONTROLLER_BUTTON_MAX: u32 = 15;
const SDL_CONTROLLER_AXIS_MAX: u32 = 6;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ControllerButton {
    A,
    B,
    X,
    Y,
    Back,
    Guide,
    Start,
    LeftStick,
    RightStick,
    LeftShoulder,
    RightShoulder,
    DpadUp,
    DpadDown,
    DpadLeft,
    DpadRight,
    Misc1,
    Paddle1,
    Paddle2,
    Paddle3,
    Paddle4,
    Touchpad,
}

impl ControllerButton {
    pub fn from_sdl(button: u32) -> Option<Self> {
        match button {
            0 => Some(ControllerButton::A),
            1 => Some(ControllerButton::B),
            2 => Some(ControllerButton::X),
            3 => Some(ControllerButton::Y),
            4 => Some(ControllerButton::Back),
            5 => Some(ControllerButton::Guide),
            6 => Some(ControllerButton::Start),
            7 => Some(ControllerButton::LeftStick),
            8 => Some(ControllerButton::RightStick),
            9 => Some(ControllerButton::LeftShoulder),
            10 => Some(ControllerButton::RightShoulder),
            11 => Some(ControllerButton::DpadUp),
            12 => Some(ControllerButton::DpadDown),
            13 => Some(ControllerButton::DpadLeft),
            14 => Some(ControllerButton::DpadRight),
            _ => None,
        }
    }

    pub fn to_sdl(&self) -> u32 {
        match self {
            ControllerButton::A => 0,
            ControllerButton::B => 1,
            ControllerButton::X => 2,
            ControllerButton::Y => 3,
            ControllerButton::Back => 4,
            ControllerButton::Guide => 5,
            ControllerButton::Start => 6,
            ControllerButton::LeftStick => 7,
            ControllerButton::RightStick => 8,
            ControllerButton::LeftShoulder => 9,
            ControllerButton::RightShoulder => 10,
            ControllerButton::DpadUp => 11,
            ControllerButton::DpadDown => 12,
            ControllerButton::DpadLeft => 13,
            ControllerButton::DpadRight => 14,
            _ => 0,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            ControllerButton::A => "A",
            ControllerButton::B => "B",
            ControllerButton::X => "X",
            ControllerButton::Y => "Y",
            ControllerButton::Back => "Back",
            ControllerButton::Guide => "Guide",
            ControllerButton::Start => "Start",
            ControllerButton::LeftStick => "Left Stick",
            ControllerButton::RightStick => "Right Stick",
            ControllerButton::LeftShoulder => "Left Shoulder",
            ControllerButton::RightShoulder => "Right Shoulder",
            ControllerButton::DpadUp => "D-Pad Up",
            ControllerButton::DpadDown => "D-Pad Down",
            ControllerButton::DpadLeft => "D-Pad Left",
            ControllerButton::DpadRight => "D-Pad Right",
            ControllerButton::Misc1 => "Misc 1",
            ControllerButton::Paddle1 => "Paddle 1",
            ControllerButton::Paddle2 => "Paddle 2",
            ControllerButton::Paddle3 => "Paddle 3",
            ControllerButton::Paddle4 => "Paddle 4",
            ControllerButton::Touchpad => "Touchpad",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ControllerAxis {
    LeftX,
    LeftY,
    RightX,
    RightY,
    LeftTrigger,
    RightTrigger,
}

impl ControllerAxis {
    pub fn from_sdl(axis: u32) -> Option<Self> {
        match axis {
            0 => Some(ControllerAxis::LeftX),
            1 => Some(ControllerAxis::LeftY),
            2 => Some(ControllerAxis::RightX),
            3 => Some(ControllerAxis::RightY),
            4 => Some(ControllerAxis::LeftTrigger),
            5 => Some(ControllerAxis::RightTrigger),
            _ => None,
        }
    }

    pub fn to_sdl(&self) -> u32 {
        match self {
            ControllerAxis::LeftX => 0,
            ControllerAxis::LeftY => 1,
            ControllerAxis::RightX => 2,
            ControllerAxis::RightY => 3,
            ControllerAxis::LeftTrigger => 4,
            ControllerAxis::RightTrigger => 5,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            ControllerAxis::LeftX => "Left Stick X",
            ControllerAxis::LeftY => "Left Stick Y",
            ControllerAxis::RightX => "Right Stick X",
            ControllerAxis::RightY => "Right Stick Y",
            ControllerAxis::LeftTrigger => "Left Trigger",
            ControllerAxis::RightTrigger => "Right Trigger",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KeyboardKey {
    KeyA,
    KeyB,
    KeyC,
    KeyD,
    KeyE,
    KeyF,
    KeyG,
    KeyH,
    KeyI,
    KeyJ,
    KeyK,
    KeyL,
    KeyM,
    KeyN,
    KeyO,
    KeyP,
    KeyQ,
    KeyR,
    KeyS,
    KeyT,
    KeyU,
    KeyV,
    KeyW,
    KeyX,
    KeyY,
    KeyZ,
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    Space,
    Enter,
    Escape,
    Tab,
    Backspace,
    Delete,
    Insert,
    Home,
    End,
    PageUp,
    PageDown,
    Left,
    Right,
    Up,
    Down,
    ShiftLeft,
    ShiftRight,
    ControlLeft,
    ControlRight,
    AltLeft,
    AltRight,
    MetaLeft,
    MetaRight,
    CapsLock,
    NumLock,
    ScrollLock,
    PrintScreen,
    Pause,
    Menu,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    X1,
    X2,
}

impl MouseButton {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "left" => Some(MouseButton::Left),
            "right" => Some(MouseButton::Right),
            "middle" => Some(MouseButton::Middle),
            "x1" => Some(MouseButton::X1),
            "x2" => Some(MouseButton::X2),
            _ => None,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            MouseButton::Left => "Left Click",
            MouseButton::Right => "Right Click",
            MouseButton::Middle => "Middle Click",
            MouseButton::X1 => "Mouse Button 4",
            MouseButton::X2 => "Mouse Button 5",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MouseAction {
    Button(MouseButton),
    MoveX,
    MoveY,
    WheelUp,
    WheelDown,
}

impl MouseAction {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "mouse_left" => Some(MouseAction::Button(MouseButton::Left)),
            "mouse_right" => Some(MouseAction::Button(MouseButton::Right)),
            "mouse_middle" => Some(MouseAction::Button(MouseButton::Middle)),
            "mouse_x1" => Some(MouseAction::Button(MouseButton::X1)),
            "mouse_x2" => Some(MouseAction::Button(MouseButton::X2)),
            "mouse_move_x" => Some(MouseAction::MoveX),
            "mouse_move_y" => Some(MouseAction::MoveY),
            "mouse_wheel_up" => Some(MouseAction::WheelUp),
            "mouse_wheel_down" => Some(MouseAction::WheelDown),
            _ => None,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            MouseAction::Button(btn) => btn.name(),
            MouseAction::MoveX => "Mouse Move X",
            MouseAction::MoveY => "Mouse Move Y",
            MouseAction::WheelUp => "Mouse Wheel Up",
            MouseAction::WheelDown => "Mouse Wheel Down",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OutputAction {
    Keyboard(KeyboardKey),
    Mouse(MouseAction),
}

impl OutputAction {
    pub fn from_str(s: &str) -> Option<Self> {
        if let Some(key) = KeyboardKey::from_str(s) {
            return Some(OutputAction::Keyboard(key));
        }
        if let Some(mouse) = MouseAction::from_str(s) {
            return Some(OutputAction::Mouse(mouse));
        }
        None
    }

    pub fn name(&self) -> String {
        match self {
            OutputAction::Keyboard(key) => key.name().to_string(),
            OutputAction::Mouse(action) => action.name().to_string(),
        }
    }
}

impl KeyboardKey {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "A" => Some(KeyboardKey::KeyA),
            "B" => Some(KeyboardKey::KeyB),
            "C" => Some(KeyboardKey::KeyC),
            "D" => Some(KeyboardKey::KeyD),
            "E" => Some(KeyboardKey::KeyE),
            "F" => Some(KeyboardKey::KeyF),
            "G" => Some(KeyboardKey::KeyG),
            "H" => Some(KeyboardKey::KeyH),
            "I" => Some(KeyboardKey::KeyI),
            "J" => Some(KeyboardKey::KeyJ),
            "K" => Some(KeyboardKey::KeyK),
            "L" => Some(KeyboardKey::KeyL),
            "M" => Some(KeyboardKey::KeyM),
            "N" => Some(KeyboardKey::KeyN),
            "O" => Some(KeyboardKey::KeyO),
            "P" => Some(KeyboardKey::KeyP),
            "Q" => Some(KeyboardKey::KeyQ),
            "R" => Some(KeyboardKey::KeyR),
            "S" => Some(KeyboardKey::KeyS),
            "T" => Some(KeyboardKey::KeyT),
            "U" => Some(KeyboardKey::KeyU),
            "V" => Some(KeyboardKey::KeyV),
            "W" => Some(KeyboardKey::KeyW),
            "X" => Some(KeyboardKey::KeyX),
            "Y" => Some(KeyboardKey::KeyY),
            "Z" => Some(KeyboardKey::KeyZ),
            "0" => Some(KeyboardKey::Key0),
            "1" => Some(KeyboardKey::Key1),
            "2" => Some(KeyboardKey::Key2),
            "3" => Some(KeyboardKey::Key3),
            "4" => Some(KeyboardKey::Key4),
            "5" => Some(KeyboardKey::Key5),
            "6" => Some(KeyboardKey::Key6),
            "7" => Some(KeyboardKey::Key7),
            "8" => Some(KeyboardKey::Key8),
            "9" => Some(KeyboardKey::Key9),
            "F1" => Some(KeyboardKey::F1),
            "F2" => Some(KeyboardKey::F2),
            "F3" => Some(KeyboardKey::F3),
            "F4" => Some(KeyboardKey::F4),
            "F5" => Some(KeyboardKey::F5),
            "F6" => Some(KeyboardKey::F6),
            "F7" => Some(KeyboardKey::F7),
            "F8" => Some(KeyboardKey::F8),
            "F9" => Some(KeyboardKey::F9),
            "F10" => Some(KeyboardKey::F10),
            "F11" => Some(KeyboardKey::F11),
            "F12" => Some(KeyboardKey::F12),
            "SPACE" => Some(KeyboardKey::Space),
            "ENTER" => Some(KeyboardKey::Enter),
            "RETURN" => Some(KeyboardKey::Enter),
            "ESC" => Some(KeyboardKey::Escape),
            "TAB" => Some(KeyboardKey::Tab),
            "BACKSPACE" => Some(KeyboardKey::Backspace),
            "DELETE" => Some(KeyboardKey::Delete),
            "DEL" => Some(KeyboardKey::Delete),
            "INSERT" => Some(KeyboardKey::Insert),
            "HOME" => Some(KeyboardKey::Home),
            "END" => Some(KeyboardKey::End),
            "PAGEUP" => Some(KeyboardKey::PageUp),
            "PAGEDOWN" => Some(KeyboardKey::PageDown),
            "LEFT" => Some(KeyboardKey::Left),
            "RIGHT" => Some(KeyboardKey::Right),
            "UP" => Some(KeyboardKey::Up),
            "DOWN" => Some(KeyboardKey::Down),
            "LSHIFT" => Some(KeyboardKey::ShiftLeft),
            "RSHIFT" => Some(KeyboardKey::ShiftRight),
            "LCTRL" => Some(KeyboardKey::ControlLeft),
            "RCTRL" => Some(KeyboardKey::ControlRight),
            "LALT" => Some(KeyboardKey::AltLeft),
            "RALT" => Some(KeyboardKey::AltRight),
            "LWIN" => Some(KeyboardKey::MetaLeft),
            "RWIN" => Some(KeyboardKey::MetaRight),
            "CAPSLOCK" => Some(KeyboardKey::CapsLock),
            "NUMLOCK" => Some(KeyboardKey::NumLock),
            "SCROLLLOCK" => Some(KeyboardKey::ScrollLock),
            "PRINTSCREEN" => Some(KeyboardKey::PrintScreen),
            "PAUSE" => Some(KeyboardKey::Pause),
            "MENU" => Some(KeyboardKey::Menu),
            _ => None,
        }
    }

    pub fn to_sdl_scancode(&self) -> u32 {
        // SDL scancode values
        match self {
            KeyboardKey::KeyA => 4,
            KeyboardKey::KeyB => 5,
            KeyboardKey::KeyC => 6,
            KeyboardKey::KeyD => 7,
            KeyboardKey::KeyE => 8,
            KeyboardKey::KeyF => 9,
            KeyboardKey::KeyG => 10,
            KeyboardKey::KeyH => 11,
            KeyboardKey::KeyI => 12,
            KeyboardKey::KeyJ => 13,
            KeyboardKey::KeyK => 14,
            KeyboardKey::KeyL => 15,
            KeyboardKey::KeyM => 16,
            KeyboardKey::KeyN => 17,
            KeyboardKey::KeyO => 18,
            KeyboardKey::KeyP => 19,
            KeyboardKey::KeyQ => 20,
            KeyboardKey::KeyR => 21,
            KeyboardKey::KeyS => 22,
            KeyboardKey::KeyT => 23,
            KeyboardKey::KeyU => 24,
            KeyboardKey::KeyV => 25,
            KeyboardKey::KeyW => 26,
            KeyboardKey::KeyX => 27,
            KeyboardKey::KeyY => 28,
            KeyboardKey::KeyZ => 29,
            KeyboardKey::Key0 => 30,
            KeyboardKey::Key1 => 31,
            KeyboardKey::Key2 => 32,
            KeyboardKey::Key3 => 33,
            KeyboardKey::Key4 => 34,
            KeyboardKey::Key5 => 35,
            KeyboardKey::Key6 => 36,
            KeyboardKey::Key7 => 37,
            KeyboardKey::Key8 => 38,
            KeyboardKey::Key9 => 39,
            KeyboardKey::F1 => 58,
            KeyboardKey::F2 => 59,
            KeyboardKey::F3 => 60,
            KeyboardKey::F4 => 61,
            KeyboardKey::F5 => 62,
            KeyboardKey::F6 => 63,
            KeyboardKey::F7 => 64,
            KeyboardKey::F8 => 65,
            KeyboardKey::F9 => 66,
            KeyboardKey::F10 => 67,
            KeyboardKey::F11 => 68,
            KeyboardKey::F12 => 69,
            KeyboardKey::Space => 44,
            KeyboardKey::Enter => 40,
            KeyboardKey::Escape => 41,
            KeyboardKey::Tab => 43,
            KeyboardKey::Backspace => 42,
            KeyboardKey::Delete => 76,
            KeyboardKey::Insert => 75,
            KeyboardKey::Home => 74,
            KeyboardKey::End => 77,
            KeyboardKey::PageUp => 73,
            KeyboardKey::PageDown => 78,
            KeyboardKey::Left => 80,
            KeyboardKey::Right => 79,
            KeyboardKey::Up => 82,
            KeyboardKey::Down => 81,
            KeyboardKey::ShiftLeft => 225,
            KeyboardKey::ShiftRight => 229,
            KeyboardKey::ControlLeft => 224,
            KeyboardKey::ControlRight => 228,
            KeyboardKey::AltLeft => 226,
            KeyboardKey::AltRight => 230,
            KeyboardKey::MetaLeft => 231,
            KeyboardKey::MetaRight => 235,
            KeyboardKey::CapsLock => 57,
            KeyboardKey::NumLock => 83,
            KeyboardKey::ScrollLock => 71,
            KeyboardKey::PrintScreen => 70,
            KeyboardKey::Pause => 72,
            KeyboardKey::Menu => 118,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            KeyboardKey::KeyA => "A",
            KeyboardKey::KeyB => "B",
            KeyboardKey::KeyC => "C",
            KeyboardKey::KeyD => "D",
            KeyboardKey::KeyE => "E",
            KeyboardKey::KeyF => "F",
            KeyboardKey::KeyG => "G",
            KeyboardKey::KeyH => "H",
            KeyboardKey::KeyI => "I",
            KeyboardKey::KeyJ => "J",
            KeyboardKey::KeyK => "K",
            KeyboardKey::KeyL => "L",
            KeyboardKey::KeyM => "M",
            KeyboardKey::KeyN => "N",
            KeyboardKey::KeyO => "O",
            KeyboardKey::KeyP => "P",
            KeyboardKey::KeyQ => "Q",
            KeyboardKey::KeyR => "R",
            KeyboardKey::KeyS => "S",
            KeyboardKey::KeyT => "T",
            KeyboardKey::KeyU => "U",
            KeyboardKey::KeyV => "V",
            KeyboardKey::KeyW => "W",
            KeyboardKey::KeyX => "X",
            KeyboardKey::KeyY => "Y",
            KeyboardKey::KeyZ => "Z",
            KeyboardKey::Key0 => "0",
            KeyboardKey::Key1 => "1",
            KeyboardKey::Key2 => "2",
            KeyboardKey::Key3 => "3",
            KeyboardKey::Key4 => "4",
            KeyboardKey::Key5 => "5",
            KeyboardKey::Key6 => "6",
            KeyboardKey::Key7 => "7",
            KeyboardKey::Key8 => "8",
            KeyboardKey::Key9 => "9",
            KeyboardKey::F1 => "F1",
            KeyboardKey::F2 => "F2",
            KeyboardKey::F3 => "F3",
            KeyboardKey::F4 => "F4",
            KeyboardKey::F5 => "F5",
            KeyboardKey::F6 => "F6",
            KeyboardKey::F7 => "F7",
            KeyboardKey::F8 => "F8",
            KeyboardKey::F9 => "F9",
            KeyboardKey::F10 => "F10",
            KeyboardKey::F11 => "F11",
            KeyboardKey::F12 => "F12",
            KeyboardKey::Space => "Space",
            KeyboardKey::Enter => "Enter",
            KeyboardKey::Escape => "Escape",
            KeyboardKey::Tab => "Tab",
            KeyboardKey::Backspace => "Backspace",
            KeyboardKey::Delete => "Delete",
            KeyboardKey::Insert => "Insert",
            KeyboardKey::Home => "Home",
            KeyboardKey::End => "End",
            KeyboardKey::PageUp => "Page Up",
            KeyboardKey::PageDown => "Page Down",
            KeyboardKey::Left => "Left",
            KeyboardKey::Right => "Right",
            KeyboardKey::Up => "Up",
            KeyboardKey::Down => "Down",
            KeyboardKey::ShiftLeft => "Left Shift",
            KeyboardKey::ShiftRight => "Right Shift",
            KeyboardKey::ControlLeft => "Left Ctrl",
            KeyboardKey::ControlRight => "Right Ctrl",
            KeyboardKey::AltLeft => "Left Alt",
            KeyboardKey::AltRight => "Right Alt",
            KeyboardKey::MetaLeft => "Left Meta",
            KeyboardKey::MetaRight => "Right Meta",
            KeyboardKey::CapsLock => "Caps Lock",
            KeyboardKey::NumLock => "Num Lock",
            KeyboardKey::ScrollLock => "Scroll Lock",
            KeyboardKey::PrintScreen => "Print Screen",
            KeyboardKey::Pause => "Pause",
            KeyboardKey::Menu => "Menu",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ButtonMapping {
    pub controller_button: ControllerButton,
    pub output_action: OutputAction,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AxisMapping {
    pub controller_axis: ControllerAxis,
    pub output_positive: Option<OutputAction>,
    pub output_negative: Option<OutputAction>,
    pub deadzone: f32,
    pub sensitivity: f32,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControllerInfo {
    pub id: String,
    pub name: String,
    pub vendor_id: u16,
    pub product_id: u16,
    pub is_bluetooth: bool,
    pub connected: bool,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputEvent {
    pub controller_id: String,
    pub button: Option<ControllerButton>,
    pub axis: Option<ControllerAxis>,
    pub value: f32,
    pub timestamp: i64,
}
