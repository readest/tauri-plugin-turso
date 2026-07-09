use std::path::PathBuf;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Get current working directory for database storage
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));

    // Check for optional encryption key from environment variable
    // Set LIBSQL_ENCRYPTION_KEY to Hex-encoded encryption key (e.g. 64 hex chars for a 32-byte key)
    // Example: export LIBSQL_ENCRYPTION_KEY=0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef
    let encryption = std::env::var("LIBSQL_ENCRYPTION_KEY").ok().map(|key_str| {
        let mut hexkey = key_str.clone();
        // Pad by repeating the key
        if key_str.len() < 64 {
            while hexkey.len() < 64 {
                let remaining = 64 - hexkey.len();
                let take = remaining.min(key_str.len());
                hexkey.push_str(&key_str[..take]);
            }
        } else {
            hexkey.truncate(64);
        }

        eprintln!("hexkey: {}", hexkey);
        tauri_plugin_turso::EncryptionConfig {
            cipher: "aes256gcm".to_string(),
            hexkey: hexkey,
        }
    });

    if encryption.is_some() {
        eprintln!("Database encryption: ENABLED");
    } else {
        eprintln!("Database encryption: DISABLED (set LIBSQL_ENCRYPTION_KEY to enable)");
    }

    let config = tauri_plugin_turso::Config {
        base_path: Some(cwd),
        encryption,
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_turso::init_with_config(config))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
