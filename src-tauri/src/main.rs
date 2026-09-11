fn main() {
    // Required on NVIDIA/Wayland before Tauri creates the WebKit webview.
    std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
    hyprland_inspector_lib::run();
}
