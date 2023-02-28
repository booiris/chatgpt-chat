#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// use openai_api::{api::CompletionArgs, Client};

use tauri::Manager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    app::AppBuilder::new()
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
                // window.close_devtools();
            }
            Ok(())
        })
        .run();
    Ok(())
}
