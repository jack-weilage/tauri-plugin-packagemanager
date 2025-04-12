use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<Packagemanager<R>> {
    Ok(Packagemanager(app.clone()))
}

/// Access to the packagemanager APIs.
pub struct Packagemanager<R: Runtime>(AppHandle<R>);

impl<R: Runtime> Packagemanager<R> {
    pub fn check_permissions(&self) -> crate::Result<PermissionStatus> {
        unimplemented!()
    }
    pub fn request_permissions(&self) -> crate::Result<PermissionStatus> {
        unimplemented!()
    }
    pub fn get_installed_applications(
        &self,
        payload: GetInstalledApplicationsRequest,
    ) -> crate::Result<GetInstalledApplicationsResponse> {
        unimplemented!()
    }
    pub fn get_application_info(
        &self,
        payload: GetApplicationInfoRequest,
    ) -> crate::Result<GetApplicationInfoResponse> {
        unimplemented!()
    }
    pub fn get_application_icon(
        &self,
        payload: GetApplicationIconRequest,
    ) -> crate::Result<GetApplicationIconResponse> {
        unimplemented!()
    }
}
