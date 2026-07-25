use controller_remapper::core::controller_types::{
    AxisMapping, ButtonMapping, ControllerAxis, ControllerButton, InputEvent, KeyboardKey,
};
use controller_remapper::core::input_mapper::InputMapper;

#[test]
fn test_button_mapping_creation() {
    let mapping = ButtonMapping {
        controller_button: ControllerButton::A,
        keyboard_key: KeyboardKey::Space,
        enabled: true,
    };

    assert_eq!(mapping.controller_button, ControllerButton::A);
    assert_eq!(mapping.keyboard_key, KeyboardKey::Space);
    assert!(mapping.enabled);
}

#[test]
fn test_axis_mapping_creation() {
    let mapping = AxisMapping {
        controller_axis: ControllerAxis::LeftX,
        keyboard_positive: Some(KeyboardKey::D),
        keyboard_negative: Some(KeyboardKey::A),
        deadzone: 0.15,
        sensitivity: 1.0,
        enabled: true,
    };

    assert_eq!(mapping.controller_axis, ControllerAxis::LeftX);
    assert_eq!(mapping.keyboard_positive, Some(KeyboardKey::D));
    assert_eq!(mapping.keyboard_negative, Some(KeyboardKey::A));
    assert_eq!(mapping.deadzone, 0.15);
    assert_eq!(mapping.sensitivity, 1.0);
    assert!(mapping.enabled);
}

#[test]
fn test_input_mapper_set_button_mapping() {
    let mut mapper = InputMapper::new();
    let mapping = ButtonMapping {
        controller_button: ControllerButton::A,
        keyboard_key: KeyboardKey::Space,
        enabled: true,
    };

    mapper.set_button_mapping(mapping);

    let retrieved = mapper.get_button_mapping(ControllerButton::A);
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().keyboard_key, KeyboardKey::Space);
}

#[test]
fn test_input_mapper_set_axis_mapping() {
    let mut mapper = InputMapper::new();
    let mapping = AxisMapping {
        controller_axis: ControllerAxis::LeftX,
        keyboard_positive: Some(KeyboardKey::D),
        keyboard_negative: Some(KeyboardKey::A),
        deadzone: 0.15,
        sensitivity: 1.0,
        enabled: true,
    };

    mapper.set_axis_mapping(mapping);

    let retrieved = mapper.get_axis_mapping(ControllerAxis::LeftX);
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().keyboard_positive, Some(KeyboardKey::D));
}

#[test]
fn test_input_mapper_remove_button_mapping() {
    let mut mapper = InputMapper::new();
    let mapping = ButtonMapping {
        controller_button: ControllerButton::A,
        keyboard_key: KeyboardKey::Space,
        enabled: true,
    };

    mapper.set_button_mapping(mapping);
    mapper.remove_button_mapping(ControllerButton::A);

    let retrieved = mapper.get_button_mapping(ControllerButton::A);
    assert!(retrieved.is_none());
}

#[test]
fn test_input_mapper_remove_axis_mapping() {
    let mut mapper = InputMapper::new();
    let mapping = AxisMapping {
        controller_axis: ControllerAxis::LeftX,
        keyboard_positive: Some(KeyboardKey::D),
        keyboard_negative: Some(KeyboardKey::A),
        deadzone: 0.15,
        sensitivity: 1.0,
        enabled: true,
    };

    mapper.set_axis_mapping(mapping);
    mapper.remove_axis_mapping(ControllerAxis::LeftX);

    let retrieved = mapper.get_axis_mapping(ControllerAxis::LeftX);
    assert!(retrieved.is_none());
}

#[test]
fn test_input_mapper_clear_mappings() {
    let mut mapper = InputMapper::new();
    
    mapper.set_button_mapping(ButtonMapping {
        controller_button: ControllerButton::A,
        keyboard_key: KeyboardKey::Space,
        enabled: true,
    });
    
    mapper.set_axis_mapping(AxisMapping {
        controller_axis: ControllerAxis::LeftX,
        keyboard_positive: Some(KeyboardKey::D),
        keyboard_negative: Some(KeyboardKey::A),
        deadzone: 0.15,
        sensitivity: 1.0,
        enabled: true,
    });

    mapper.clear_mappings();

    assert!(mapper.get_button_mapping(ControllerButton::A).is_none());
    assert!(mapper.get_axis_mapping(ControllerAxis::LeftX).is_none());
}

#[tokio::test]
async fn test_input_mapper_process_button_event_press() {
    let mut mapper = InputMapper::new();
    mapper.set_button_mapping(ButtonMapping {
        controller_button: ControllerButton::A,
        keyboard_key: KeyboardKey::Space,
        enabled: true,
    });

    let event = InputEvent {
        controller_id: "test".to_string(),
        button: Some(ControllerButton::A),
        axis: None,
        value: 1.0,
        timestamp: 0,
    };

    let keys = mapper.process_button_event(&event).await.unwrap();
    assert_eq!(keys.len(), 1);
    assert_eq!(keys[0], KeyboardKey::Space);
}

#[tokio::test]
async fn test_input_mapper_process_button_event_release() {
    let mut mapper = InputMapper::new();
    mapper.set_button_mapping(ButtonMapping {
        controller_button: ControllerButton::A,
        keyboard_key: KeyboardKey::Space,
        enabled: true,
    });

    let event = InputEvent {
        controller_id: "test".to_string(),
        button: Some(ControllerButton::A),
        axis: None,
        value: 0.0,
        timestamp: 0,
    };

    let keys = mapper.process_button_event(&event).await.unwrap();
    assert_eq!(keys.len(), 0);
}

#[tokio::test]
async fn test_input_mapper_process_button_event_disabled() {
    let mut mapper = InputMapper::new();
    mapper.set_button_mapping(ButtonMapping {
        controller_button: ControllerButton::A,
        keyboard_key: KeyboardKey::Space,
        enabled: false,
    });

    let event = InputEvent {
        controller_id: "test".to_string(),
        button: Some(ControllerButton::A),
        axis: None,
        value: 1.0,
        timestamp: 0,
    };

    let keys = mapper.process_button_event(&event).await.unwrap();
    assert_eq!(keys.len(), 0);
}

#[tokio::test]
async fn test_input_mapper_process_axis_event_positive() {
    let mut mapper = InputMapper::new();
    mapper.set_axis_mapping(AxisMapping {
        controller_axis: ControllerAxis::LeftX,
        keyboard_positive: Some(KeyboardKey::D),
        keyboard_negative: Some(KeyboardKey::A),
        deadzone: 0.15,
        sensitivity: 1.0,
        enabled: true,
    });

    let event = InputEvent {
        controller_id: "test".to_string(),
        button: None,
        axis: Some(ControllerAxis::LeftX),
        value: 0.8,
        timestamp: 0,
    };

    let keys = mapper.process_axis_event(&event).await.unwrap();
    assert_eq!(keys.len(), 1);
    assert_eq!(keys[0], KeyboardKey::D);
}

#[tokio::test]
async fn test_input_mapper_process_axis_event_negative() {
    let mut mapper = InputMapper::new();
    mapper.set_axis_mapping(AxisMapping {
        controller_axis: ControllerAxis::LeftX,
        keyboard_positive: Some(KeyboardKey::D),
        keyboard_negative: Some(KeyboardKey::A),
        deadzone: 0.15,
        sensitivity: 1.0,
        enabled: true,
    });

    let event = InputEvent {
        controller_id: "test".to_string(),
        button: None,
        axis: Some(ControllerAxis::LeftX),
        value: -0.8,
        timestamp: 0,
    };

    let keys = mapper.process_axis_event(&event).await.unwrap();
    assert_eq!(keys.len(), 1);
    assert_eq!(keys[0], KeyboardKey::A);
}

#[tokio::test]
async fn test_input_mapper_process_axis_event_deadzone() {
    let mut mapper = InputMapper::new();
    mapper.set_axis_mapping(AxisMapping {
        controller_axis: ControllerAxis::LeftX,
        keyboard_positive: Some(KeyboardKey::D),
        keyboard_negative: Some(KeyboardKey::A),
        deadzone: 0.15,
        sensitivity: 1.0,
        enabled: true,
    });

    let event = InputEvent {
        controller_id: "test".to_string(),
        button: None,
        axis: Some(ControllerAxis::LeftX),
        value: 0.1, // Within deadzone
        timestamp: 0,
    };

    let keys = mapper.process_axis_event(&event).await.unwrap();
    assert_eq!(keys.len(), 0);
}

#[tokio::test]
async fn test_input_mapper_process_axis_event_disabled() {
    let mut mapper = InputMapper::new();
    mapper.set_axis_mapping(AxisMapping {
        controller_axis: ControllerAxis::LeftX,
        keyboard_positive: Some(KeyboardKey::D),
        keyboard_negative: Some(KeyboardKey::A),
        deadzone: 0.15,
        sensitivity: 1.0,
        enabled: false,
    });

    let event = InputEvent {
        controller_id: "test".to_string(),
        button: None,
        axis: Some(ControllerAxis::LeftX),
        value: 0.8,
        timestamp: 0,
    };

    let keys = mapper.process_axis_event(&event).await.unwrap();
    assert_eq!(keys.len(), 0);
}

#[tokio::test]
async fn test_input_mapper_process_event() {
    let mut mapper = InputMapper::new();
    mapper.set_button_mapping(ButtonMapping {
        controller_button: ControllerButton::A,
        keyboard_key: KeyboardKey::Space,
        enabled: true,
    });

    let event = InputEvent {
        controller_id: "test".to_string(),
        button: Some(ControllerButton::A),
        axis: None,
        value: 1.0,
        timestamp: 0,
    };

    let keys = mapper.process_event(&event).await.unwrap();
    assert_eq!(keys.len(), 1);
    assert_eq!(keys[0], KeyboardKey::Space);
}

#[tokio::test]
async fn test_input_mapper_get_active_keys() {
    let mut mapper = InputMapper::new();
    mapper.set_button_mapping(ButtonMapping {
        controller_button: ControllerButton::A,
        keyboard_key: KeyboardKey::Space,
        enabled: true,
    });

    let event = InputEvent {
        controller_id: "test".to_string(),
        button: Some(ControllerButton::A),
        axis: None,
        value: 1.0,
        timestamp: 0,
    };

    mapper.process_event(&event).await.unwrap();
    let active_keys = mapper.get_active_keys().await;
    
    assert_eq!(active_keys.len(), 1);
    assert_eq!(active_keys[0], KeyboardKey::Space);
}

#[tokio::test]
async fn test_input_mapper_release_all_keys() {
    let mut mapper = InputMapper::new();
    mapper.set_button_mapping(ButtonMapping {
        controller_button: ControllerButton::A,
        keyboard_key: KeyboardKey::Space,
        enabled: true,
    });

    let event = InputEvent {
        controller_id: "test".to_string(),
        button: Some(ControllerButton::A),
        axis: None,
        value: 1.0,
        timestamp: 0,
    };

    mapper.process_event(&event).await.unwrap();
    mapper.release_all_keys().await.unwrap();
    
    let active_keys = mapper.get_active_keys().await;
    assert_eq!(active_keys.len(), 0);
}

#[test]
fn test_controller_button_from_sdl() {
    assert_eq!(ControllerButton::from_sdl(0), Some(ControllerButton::A));
    assert_eq!(ControllerButton::from_sdl(1), Some(ControllerButton::B));
    assert_eq!(ControllerButton::from_sdl(2), Some(ControllerButton::X));
    assert_eq!(ControllerButton::from_sdl(3), Some(ControllerButton::Y));
    assert_eq!(ControllerButton::from_sdl(99), None);
}

#[test]
fn test_controller_button_to_sdl() {
    assert_eq!(ControllerButton::A.to_sdl(), 0);
    assert_eq!(ControllerButton::B.to_sdl(), 1);
    assert_eq!(ControllerButton::X.to_sdl(), 2);
    assert_eq!(ControllerButton::Y.to_sdl(), 3);
}

#[test]
fn test_controller_button_from_str() {
    assert_eq!(ControllerButton::from_str("A"), Some(ControllerButton::A));
    assert_eq!(ControllerButton::from_str("B"), Some(ControllerButton::B));
    assert_eq!(ControllerButton::from_str("X"), Some(ControllerButton::X));
    assert_eq!(ControllerButton::from_str("Y"), Some(ControllerButton::Y));
    assert_eq!(ControllerButton::from_str("INVALID"), None);
}

#[test]
fn test_controller_axis_from_sdl() {
    assert_eq!(ControllerAxis::from_sdl(0), Some(ControllerAxis::LeftX));
    assert_eq!(ControllerAxis::from_sdl(1), Some(ControllerAxis::LeftY));
    assert_eq!(ControllerAxis::from_sdl(2), Some(ControllerAxis::RightX));
    assert_eq!(ControllerAxis::from_sdl(3), Some(ControllerAxis::RightY));
    assert_eq!(ControllerAxis::from_sdl(99), None);
}

#[test]
fn test_controller_axis_to_sdl() {
    assert_eq!(ControllerAxis::LeftX.to_sdl(), 0);
    assert_eq!(ControllerAxis::LeftY.to_sdl(), 1);
    assert_eq!(ControllerAxis::RightX.to_sdl(), 2);
    assert_eq!(ControllerAxis::RightY.to_sdl(), 3);
}

#[test]
fn test_controller_axis_from_str() {
    assert_eq!(ControllerAxis::from_str("LEFTX"), Some(ControllerAxis::LeftX));
    assert_eq!(ControllerAxis::from_str("LEFTY"), Some(ControllerAxis::LeftY));
    assert_eq!(ControllerAxis::from_str("RIGHTX"), Some(ControllerAxis::RightX));
    assert_eq!(ControllerAxis::from_str("RIGHTY"), Some(ControllerAxis::RightY));
    assert_eq!(ControllerAxis::from_str("INVALID"), None);
}

#[test]
fn test_keyboard_key_from_str() {
    assert_eq!(KeyboardKey::from_str("A"), Some(KeyboardKey::KeyA));
    assert_eq!(KeyboardKey::from_str("SPACE"), Some(KeyboardKey::Space));
    assert_eq!(KeyboardKey::from_str("ENTER"), Some(KeyboardKey::Enter));
    assert_eq!(KeyboardKey::from_str("F1"), Some(KeyboardKey::F1));
    assert_eq!(KeyboardKey::from_str("INVALID"), None);
}

#[test]
fn test_keyboard_key_to_sdl_scancode() {
    assert_eq!(KeyboardKey::KeyA.to_sdl_scancode(), 4);
    assert_eq!(KeyboardKey::Space.to_sdl_scancode(), 44);
    assert_eq!(KeyboardKey::Enter.to_sdl_scancode(), 40);
    assert_eq!(KeyboardKey::F1.to_sdl_scancode(), 58);
}

#[test]
fn test_input_mapper_from_mappings() {
    let button_mappings = vec![
        ButtonMapping {
            controller_button: ControllerButton::A,
            keyboard_key: KeyboardKey::Space,
            enabled: true,
        },
    ];

    let axis_mappings = vec![
        AxisMapping {
            controller_axis: ControllerAxis::LeftX,
            keyboard_positive: Some(KeyboardKey::D),
            keyboard_negative: Some(KeyboardKey::A),
            deadzone: 0.15,
            sensitivity: 1.0,
            enabled: true,
        },
    ];

    let mapper = InputMapper::from_mappings(button_mappings, axis_mappings);

    assert!(mapper.get_button_mapping(ControllerButton::A).is_some());
    assert!(mapper.get_axis_mapping(ControllerAxis::LeftX).is_some());
}
