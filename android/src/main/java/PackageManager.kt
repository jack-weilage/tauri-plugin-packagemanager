package dev.weilage.packagemanager

import android.content.Context
import android.content.pm.ApplicationInfo
import app.tauri.plugin.JSObject

class PackageManager(context: Context) {
    private val packageManager = context.packageManager
    fun applicationInfoToObject(applicationInfo: ApplicationInfo): JSObject {
        val info = JSObject()

        info.put("packageName", applicationInfo.packageName)
        info.put("name", applicationInfo.loadLabel(packageManager))
//        info.put("description", applicationInfo.loadDescription(packageManager))
        info.put("flags", applicationInfo.flags)

        return info
    }
}
