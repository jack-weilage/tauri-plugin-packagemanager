import { invoke } from "@tauri-apps/api/core";

export interface ApplicationInfo {
	packageName: string;
	name: string;
	flags: number;
}

export async function checkPermissions(): Promise<PermissionState> {
	const { state } = await invoke<{ state: PermissionState }>(
		"plugin:packagemanager|check_permissions",
	);

	return state;
}
export async function requestPermissions(): Promise<PermissionState> {
	const { state } = await invoke<{ state: PermissionState }>(
		"plugin:packagemanager|request_permissions",
	);

	return state;
}
export async function getInstalledApplications(flags: number = 0): Promise<ApplicationInfo[]> {
	const { applications } = await invoke<{ applications: ApplicationInfo[] }>(
		"plugin:packagemanager|get_installed_applications",
		{ payload: { flags } },
	);

	return applications;
}
export async function getApplicationInfo(packageName: string): Promise<ApplicationInfo> {
	const { application } = await invoke<{ application: ApplicationInfo }>(
		"plugin:packagemanager|get_application_info",
		{ payload: { packageName } },
	);

	return application;
}
export async function getApplicationIcon(packageName: string): Promise<string> {
	const { icon } = await invoke<{ icon: string }>("plugin:packagemanager|get_application_icon", {
		payload: { packageName },
	});

	return icon;
}
