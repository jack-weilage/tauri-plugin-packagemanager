use tauri::{command, AppHandle, Runtime};

use crate::models::*;
use crate::PackagemanagerExt;
use crate::Result;

#[command]
pub(crate) async fn check_permissions<R: Runtime>(app: AppHandle<R>) -> Result<PermissionStatus> {
    app.packagemanager().check_permissions()
}

#[command]
pub(crate) async fn request_permissions<R: Runtime>(app: AppHandle<R>) -> Result<PermissionStatus> {
    app.packagemanager().request_permissions()
}

#[command]
pub(crate) async fn get_installed_applications<R: Runtime>(
    app: AppHandle<R>,
    payload: GetInstalledApplicationsRequest,
) -> Result<GetInstalledApplicationsResponse> {
    app.packagemanager().get_installed_applications(payload)
}

#[command]
pub(crate) async fn get_visible_applications<R: Runtime>(
    app: AppHandle<R>,
) -> Result<GetVisibleApplicationsResponse> {
    app.packagemanager().get_visible_applications()
}

#[command]
pub(crate) async fn get_application_info<R: Runtime>(
    app: AppHandle<R>,
    payload: GetApplicationInfoRequest,
) -> Result<GetApplicationInfoResponse> {
    app.packagemanager().get_application_info(payload)
}

#[command]
pub(crate) async fn get_application_icon<R: Runtime>(
    app: AppHandle<R>,
    payload: GetApplicationIconRequest,
) -> Result<GetApplicationIconResponse> {
    app.packagemanager().get_application_icon(payload)
}
