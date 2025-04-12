package dev.weilage.packagemanager

import android.content.Context
import android.content.Intent
import android.content.pm.ApplicationInfo
import android.os.Build
import app.tauri.plugin.JSObject

class PackageManager(context: Context) {
    private val packageManager = context.packageManager
    fun applicationInfoToObject(applicationInfo: ApplicationInfo): JSObject {
        val info = JSObject()

        info.put("packageName", applicationInfo.packageName)
        info.put("name", applicationInfo.loadLabel(packageManager))
//        info.put("description", applicationInfo.loadDescription(packageManager))
        info.put("flags", applicationInfo.flags)

        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            info.put("category", applicationInfo.category)
        }

        return info
    }

    fun getVisibleApplications(): List<ApplicationInfo>{
        val intent = Intent(Intent.ACTION_MAIN)
        intent.addCategory(Intent.CATEGORY_LAUNCHER)

        val activities = packageManager.queryIntentActivities(intent, 0)

        return activities.map { it.activityInfo.applicationInfo }.distinctBy { it.packageName }
    }
}
