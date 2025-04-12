use serde::de::DeserializeOwned;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "android")]
const PLUGIN_IDENTIFIER: &str = "dev.weilage.packagemanager";

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_packagemanager);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<Packagemanager<R>> {
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin(PLUGIN_IDENTIFIER, "PackageManagerPlugin")?;
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_packagemanager)?;
    Ok(Packagemanager(handle))
}

/// Access to the packagemanager APIs.
pub struct Packagemanager<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> Packagemanager<R> {
    pub fn check_permissions(&self) -> crate::Result<PermissionStatus> {
        self.0
            .run_mobile_plugin("checkPermissions", ())
            .map_err(Into::into)
    }
    pub fn request_permissions(&self) -> crate::Result<PermissionStatus> {
        self.0
            .run_mobile_plugin("requestPermissions", ())
            .map_err(Into::into)
    }
    pub fn get_installed_applications(
        &self,
        payload: GetInstalledApplicationsRequest,
    ) -> crate::Result<GetInstalledApplicationsResponse> {
        self.0
            .run_mobile_plugin("getInstalledApplications", payload)
            .map_err(Into::into)
    }
    pub fn get_visible_applications(&self) -> crate::Result<GetVisibleApplicationsResponse> {
        self.0
            .run_mobile_plugin("getVisibleApplications", ())
            .map_err(Into::into)
    }
    pub fn get_application_info(
        &self,
        payload: GetApplicationInfoRequest,
    ) -> crate::Result<GetApplicationInfoResponse> {
        self.0
            .run_mobile_plugin("getApplicationInfo", payload)
            .map_err(Into::into)
    }
    pub fn get_application_icon(
        &self,
        payload: GetApplicationIconRequest,
    ) -> crate::Result<GetApplicationIconResponse> {
        self.0
            .run_mobile_plugin("getApplicationIcon", payload)
            .map_err(Into::into)
    }
}
