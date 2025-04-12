import { invoke } from "@tauri-apps/api/core";

export interface ApplicationInfo {
	packageName: string;
	name: string;
	flags: number;
	category?: number;
}

export async function checkPermissions(): Promise<PermissionState> {
	const { queryAllPackages } = await invoke<{ queryAllPackages: PermissionState }>(
		"plugin:packagemanager|check_permissions",
	);

	return queryAllPackages;
}
export async function requestPermissions(): Promise<PermissionState> {
	const { queryAllPackages } = await invoke<{ queryAllPackages: PermissionState }>(
		"plugin:packagemanager|request_permissions",
	);

	return queryAllPackages;
}
export async function getInstalledApplications(flags: number = 0): Promise<ApplicationInfo[]> {
	const { applications } = await invoke<{ applications: ApplicationInfo[] }>(
		"plugin:packagemanager|get_installed_applications",
		{ payload: { flags } },
	);

	return applications;
}
export async function getVisibleApplications(): Promise<ApplicationInfo[]> {
	const { applications } = await invoke<{ applications: ApplicationInfo[] }>(
		"plugin:packagemanager|get_visible_applications",
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
