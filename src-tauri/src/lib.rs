use std::{env, io::{BufRead, BufReader}, os::unix::net::UnixStream, process::Command, thread};
use tauri::{AppHandle, Emitter, Manager, WebviewUrl, WebviewWindowBuilder};

fn hyprctl_json(argument: &str) -> Result<serde_json::Value, String> {
    let output = Command::new("hyprctl").args(["-j", argument]).output().map_err(|e| format!("Could not run hyprctl: {e}"))?;
    if !output.status.success() { return Err(String::from_utf8_lossy(&output.stderr).trim().to_owned()); }
    serde_json::from_slice(&output.stdout).map_err(|e| format!("Hyprland returned invalid JSON: {e}"))
}

#[tauri::command]
fn snapshot() -> Result<serde_json::Value, String> {
    Ok(serde_json::json!({
        "clients": hyprctl_json("clients")?,
        "monitors": hyprctl_json("monitors")?,
        "workspaces": hyprctl_json("workspaces")?,
        "active": hyprctl_json("activewindow").unwrap_or(serde_json::json!({})),
    }))
}

#[tauri::command]
fn dispatch(command: String) -> Result<String, String> {
    let output = Command::new("hyprctl").args(["dispatch", &command]).output().map_err(|e| e.to_string())?;
    if output.status.success() { Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned()) } else { Err(String::from_utf8_lossy(&output.stderr).trim().to_owned()) }
}

fn listen_events(app: AppHandle) {
    thread::spawn(move || loop {
        let signature = match env::var("HYPRLAND_INSTANCE_SIGNATURE") { Ok(v) => v, Err(_) => { thread::sleep(std::time::Duration::from_secs(2)); continue; } };
        let runtime = env::var("XDG_RUNTIME_DIR").unwrap_or_else(|_| "/tmp".into());
        let path = format!("{runtime}/hypr/{signature}/.socket2.sock");
        match UnixStream::connect(path) {
            Ok(stream) => {
                for line in BufReader::new(stream).lines().map_while(Result::ok) { let _ = app.emit("hypr-event", line); }
            }
            Err(_) => thread::sleep(std::time::Duration::from_secs(2)),
        }
    });
}

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if std::env::args().any(|arg| arg == "--settings") {
                if let Some(main) = app.get_webview_window("main") { let _ = main.hide(); }
                let _ = WebviewWindowBuilder::new(app, "settings", WebviewUrl::App("index.html".into()))
                    .title("Hyprland Inspector — Settings").inner_size(740.0, 620.0).min_inner_size(600.0, 500.0)
                    .decorations(false).transparent(true).initialization_script("window.__HYPR_INSPECTOR_SETTINGS__ = true;").build();
            }
            listen_events(app.handle().clone()); Ok(())
        })
        .invoke_handler(tauri::generate_handler![snapshot, dispatch])
        .run(tauri::generate_context!())
        .expect("error while running Hyprland Inspector");
}
