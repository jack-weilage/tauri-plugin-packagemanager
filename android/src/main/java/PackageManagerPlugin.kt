package dev.weilage.packagemanager

import android.Manifest
import android.app.Activity
import android.content.pm.ApplicationInfo
import android.graphics.Bitmap
import android.graphics.drawable.Drawable
import android.util.Base64
import androidx.core.graphics.drawable.toBitmap
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.Permission
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.JSArray
import java.io.ByteArrayOutputStream


@InvokeArg
class GetInstalledApplicationsArgs {
    var flags: Int? = null
}

@InvokeArg
class GetApplicationInfoArgs {
    var packageName: String? = null
}

@InvokeArg
class GetApplicationIconArgs {
    var packageName: String? = null
//    var width: Int? = null
//    var height: Int? = null
}

@TauriPlugin(
    permissions = [
        Permission(strings = [Manifest.permission.QUERY_ALL_PACKAGES], alias = "queryAllPackages")
    ]
)
class PackageManagerPlugin(private val activity: Activity): Plugin(activity) {
    private val implementation = PackageManager(activity.applicationContext)
    private val packageManager = activity.packageManager

    @Command
    fun getInstalledApplications(invoke: Invoke) {
        val args = invoke.parseArgs(GetInstalledApplicationsArgs::class.java)

        val applicationList = JSArray()
        val applications = packageManager.getInstalledApplications(args.flags ?: 0)
        for (application in applications) {
            applicationList.put(implementation.applicationInfoToObject(application))
        }

        val ret = JSObject()
        ret.put("applications", applicationList)
        invoke.resolve(ret)
    }

    @Command
    fun getApplicationInfo(invoke: Invoke) {
        val args = invoke.parseArgs(GetApplicationInfoArgs::class.java)

        val ret = JSObject()
        ret.put("application", null)
        try {
            val applicationInfo = packageManager.getApplicationInfo(args.packageName!!, 0)
            val info = implementation.applicationInfoToObject(applicationInfo)

            ret.put("application", info)
        } catch (_: Exception) {}

        invoke.resolve(ret)
    }

    @Command
    fun getApplicationIcon(invoke: Invoke) {
        val args = invoke.parseArgs(GetApplicationIconArgs::class.java)

        val ret = JSObject()
        ret.put("icon", null)
        try {
            val icon = packageManager.getApplicationIcon(args.packageName!!)
            val bitmap = icon.toBitmap()
            val stream = ByteArrayOutputStream()
            bitmap.compress(Bitmap.CompressFormat.PNG, 100, stream)

            ret.put("icon", Base64.encodeToString(stream.toByteArray(), Base64.DEFAULT))
        } catch (_: Exception) {}

        invoke.resolve(ret)
    }
}
