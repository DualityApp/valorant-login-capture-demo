// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

const VAL_SCRIPT: &str = r#"
    document.addEventListener("DOMContentLoaded", () => {
        document.addEventListener('click', function(event) {
            if (event.target && event.target.matches('button[data-testid="btn-signin-submit"]')) {
                var username = document.querySelector('[name="username"]').value;
                    var password = document.querySelector('[name="password"]').value;
                    window.__TAURI__.event.emit('valorant_loggin', {username: username, password: password});
            }
        });
    });
"#;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn valorant_login(handle: tauri::AppHandle) {
    let handle_clone = handle.clone();

    tauri::WindowBuilder::new(
        &handle,
        "external", /* the unique window label */
        tauri::WindowUrl::External("https://auth.riotgames.com/authorize?redirect_uri=https%3A%2F%2Fplayvalorant.com%2Fopt_in&client_id=play-valorant-web-prod&response_type=token%20id_token&nonce=1&scope=account%20openid".parse().unwrap()),
    )
    .initialization_script(VAL_SCRIPT)
    .on_navigation(move |url| {
        let str = url.to_string();
        println!("on_navigation: {}", str);
        handle_clone.emit_to("main", "on_nav", str).unwrap();
        true
    })
    .focused(true)
    .build()
    .unwrap();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![valorant_login])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
