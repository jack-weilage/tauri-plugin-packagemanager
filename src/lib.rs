use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::Packagemanager;
#[cfg(mobile)]
use mobile::Packagemanager;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the packagemanager APIs.
pub trait PackagemanagerExt<R: Runtime> {
    fn packagemanager(&self) -> &Packagemanager<R>;
}

impl<R: Runtime, T: Manager<R>> crate::PackagemanagerExt<R> for T {
    fn packagemanager(&self) -> &Packagemanager<R> {
        self.state::<Packagemanager<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("packagemanager")
        .invoke_handler(tauri::generate_handler![
            commands::check_permissions,
            commands::request_permissions,
            commands::get_installed_applications,
            commands::get_visible_applications,
            commands::get_application_info,
            commands::get_application_icon,
        ])
        .setup(|app, api| {
            #[cfg(mobile)]
            let packagemanager = mobile::init(app, api)?;
            #[cfg(desktop)]
            let packagemanager = desktop::init(app, api)?;
            app.manage(packagemanager);
            Ok(())
        })
        .build()
}
