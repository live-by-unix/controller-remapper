use controller_remapper::core::controller_manager::ControllerManager;
use controller_remapper::core::controller_types::{ControllerButton, ControllerAxis, KeyboardKey};
use controller_remapper::profiles::profile::{Profile, ButtonMapping, AxisMapping};

#[tokio::test]
async fn test_controller_manager_initialization() {
    let manager = ControllerManager::new();
    assert!(manager.get_controllers().await.is_empty());
}

#[tokio::test]
async fn test_controller_manager_button_mapping() {
    let mut manager = ControllerManager::new();
    
    let result = manager.map_button("A", "Space").await;
    assert!(result.is_ok());
    
    let mapper = manager.get_input_mapper().await;
    let mapping = mapper.get_button_mapping(ControllerButton::A);
    assert!(mapping.is_some());
    assert_eq!(mapping.unwrap().keyboard_key, KeyboardKey::Space);
}

#[tokio::test]
async fn test_controller_manager_axis_mapping() {
    let mut manager = ControllerManager::new();
    
    let result = manager.map_axis("LeftX", Some("D"), Some("A")).await;
    assert!(result.is_ok());
    
    let mapper = manager.get_input_mapper().await;
    let mapping = mapper.get_axis_mapping(ControllerAxis::LeftX);
    assert!(mapping.is_some());
    assert_eq!(mapping.unwrap().keyboard_positive, Some(KeyboardKey::D));
    assert_eq!(mapping.unwrap().keyboard_negative, Some(KeyboardKey::A));
}

#[tokio::test]
async fn test_controller_manager_clear_mappings() {
    let mut manager = ControllerManager::new();
    
    manager.map_button("A", "Space").await.unwrap();
    manager.map_axis("LeftX", Some("D"), Some("A")).await.unwrap();
    
    manager.clear_mappings().await;
    
    let mapper = manager.get_input_mapper().await;
    assert!(mapper.get_button_mapping(ControllerButton::A).is_none());
    assert!(mapper.get_axis_mapping(ControllerAxis::LeftX).is_none());
}

#[tokio::test]
async fn test_profile_creation() {
    let profile = Profile::new("Test Profile", "Test Game", "Test Author");
    
    assert_eq!(profile.name, "Test Profile");
    assert_eq!(profile.game_name, "Test Game");
    assert_eq!(profile.author, "Test Author");
    assert!(!profile.id.is_empty());
    assert!(profile.button_mappings.is_empty());
    assert!(profile.axis_mappings.is_empty());
}

#[tokio::test]
async fn test_profile_with_mappings() {
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

    let profile = Profile::with_mappings(
        "Test Profile",
        "Test Game",
        "Test Author",
        button_mappings,
        axis_mappings,
    );

    assert_eq!(profile.button_mappings.len(), 1);
    assert_eq!(profile.axis_mappings.len(), 1);
}

#[tokio::test]
async fn test_profile_add_button_mapping() {
    let mut profile = Profile::new("Test Profile", "Test Game", "Test Author");
    
    let mapping = ButtonMapping {
        controller_button: ControllerButton::A,
        keyboard_key: KeyboardKey::Space,
        enabled: true,
    };
    
    profile.add_button_mapping(mapping);
    
    assert_eq!(profile.button_mappings.len(), 1);
    assert_eq!(profile.button_mappings[0].controller_button, ControllerButton::A);
}

#[tokio::test]
async fn test_profile_add_axis_mapping() {
    let mut profile = Profile::new("Test Profile", "Test Game", "Test Author");
    
    let mapping = AxisMapping {
        controller_axis: ControllerAxis::LeftX,
        keyboard_positive: Some(KeyboardKey::D),
        keyboard_negative: Some(KeyboardKey::A),
        deadzone: 0.15,
        sensitivity: 1.0,
        enabled: true,
    };
    
    profile.add_axis_mapping(mapping);
    
    assert_eq!(profile.axis_mappings.len(), 1);
    assert_eq!(profile.axis_mappings[0].controller_axis, ControllerAxis::LeftX);
}

#[tokio::test]
async fn test_profile_remove_button_mapping() {
    let mut profile = Profile::new("Test Profile", "Test Game", "Test Author");
    
    profile.add_button_mapping(ButtonMapping {
        controller_button: ControllerButton::A,
        keyboard_key: KeyboardKey::Space,
        enabled: true,
    });
    
    profile.remove_button_mapping(&ControllerButton::A);
    
    assert_eq!(profile.button_mappings.len(), 0);
}

#[tokio::test]
async fn test_profile_remove_axis_mapping() {
    let mut profile = Profile::new("Test Profile", "Test Game", "Test Author");
    
    profile.add_axis_mapping(AxisMapping {
        controller_axis: ControllerAxis::LeftX,
        keyboard_positive: Some(KeyboardKey::D),
        keyboard_negative: Some(KeyboardKey::A),
        deadzone: 0.15,
        sensitivity: 1.0,
        enabled: true,
    });
    
    profile.remove_axis_mapping(&ControllerAxis::LeftX);
    
    assert_eq!(profile.axis_mappings.len(), 0);
}

#[tokio::test]
async fn test_profile_clear_mappings() {
    let mut profile = Profile::new("Test Profile", "Test Game", "Test Author");
    
    profile.add_button_mapping(ButtonMapping {
        controller_button: ControllerButton::A,
        keyboard_key: KeyboardKey::Space,
        enabled: true,
    });
    
    profile.add_axis_mapping(AxisMapping {
        controller_axis: ControllerAxis::LeftX,
        keyboard_positive: Some(KeyboardKey::D),
        keyboard_negative: Some(KeyboardKey::A),
        deadzone: 0.15,
        sensitivity: 1.0,
        enabled: true,
    });
    
    profile.clear_mappings();
    
    assert_eq!(profile.button_mappings.len(), 0);
    assert_eq!(profile.axis_mappings.len(), 0);
}

#[tokio::test]
async fn test_profile_serialization() {
    let profile = Profile::new("Test Profile", "Test Game", "Test Author");
    
    let json = profile.to_json();
    assert!(json.is_ok());
    
    let deserialized = Profile::from_json(&json.unwrap());
    assert!(deserialized.is_ok());
    
    let deserialized_profile = deserialized.unwrap();
    assert_eq!(deserialized_profile.name, profile.name);
    assert_eq!(deserialized_profile.game_name, profile.game_name);
    assert_eq!(deserialized_profile.author, profile.author);
}

#[tokio::test]
async fn test_profile_button_mapping_replacement() {
    let mut profile = Profile::new("Test Profile", "Test Game", "Test Author");
    
    profile.add_button_mapping(ButtonMapping {
        controller_button: ControllerButton::A,
        keyboard_key: KeyboardKey::Space,
        enabled: true,
    });
    
    profile.add_button_mapping(ButtonMapping {
        controller_button: ControllerButton::A,
        keyboard_key: KeyboardKey::Enter,
        enabled: true,
    });
    
    assert_eq!(profile.button_mappings.len(), 1);
    assert_eq!(profile.button_mappings[0].keyboard_key, KeyboardKey::Enter);
}

#[tokio::test]
async fn test_profile_axis_mapping_replacement() {
    let mut profile = Profile::new("Test Profile", "Test Game", "Test Author");
    
    profile.add_axis_mapping(AxisMapping {
        controller_axis: ControllerAxis::LeftX,
        keyboard_positive: Some(KeyboardKey::D),
        keyboard_negative: Some(KeyboardKey::A),
        deadzone: 0.15,
        sensitivity: 1.0,
        enabled: true,
    });
    
    profile.add_axis_mapping(AxisMapping {
        controller_axis: ControllerAxis::LeftX,
        keyboard_positive: Some(KeyboardKey::Right),
        keyboard_negative: Some(KeyboardKey::Left),
        deadzone: 0.2,
        sensitivity: 1.5,
        enabled: true,
    });
    
    assert_eq!(profile.axis_mappings.len(), 1);
    assert_eq!(profile.axis_mappings[0].keyboard_positive, Some(KeyboardKey::Right));
}
