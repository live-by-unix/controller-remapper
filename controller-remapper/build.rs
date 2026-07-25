fn main() {
    tauri_build::build();
    
    // Configure Steamworks SDK linking
    configure_steamworks();
}

fn configure_steamworks() {
    println!("cargo:rerun-if-changed=sdk");
    
    // Set Steamworks SDK path
    let sdk_path = std::env::var("STEAMWORKS_SDK").unwrap_or_else(|_| "sdk".to_string());
    
    if std::path::Path::new(&sdk_path).exists() {
        println!("cargo:rustc-env=STEAMWORKS_SDK={}", sdk_path);
        
        // Add Steamworks SDK include path
        let include_path = format!("{}/public", sdk_path);
        println!("cargo:rustc-cfg=steamworks");
        
        // Platform-specific linking
        #[cfg(target_os = "windows")]
        {
            println!("cargo:rustc-link-search={}/redistributable_bin", sdk_path);
            println!("cargo:rustc-link-lib=steam_api64");
        }
        
        #[cfg(target_os = "linux")]
        {
            println!("cargo:rustc-link-search={}/redistributable_bin/linux64", sdk_path);
            println!("cargo:rustc-link-lib=steam_api");
        }
        
        #[cfg(target_os = "macos")]
        {
            println!("cargo:rustc-link-search={}/redistributable_bin/osx64", sdk_path);
            println!("cargo:rustc-link-lib=steam_api");
            println!("cargo:rustc-link-framework=CoreFoundation");
            println!("cargo:rustc-link-framework=CoreGraphics");
        }
    } else {
        println!("cargo:warning=Steamworks SDK not found at {}. Steam features will be disabled.", sdk_path);
    }
}
