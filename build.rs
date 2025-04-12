const COMMANDS: &[&str] = &[
    "check_permissions",
    "request_permissions",
    "get_installed_applications",
    "get_application_info",
    "get_application_icon",
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .ios_path("ios")
        .build();
}
