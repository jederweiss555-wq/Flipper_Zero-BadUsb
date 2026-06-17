package com.privacyguard.detector

import android.app.admin.DevicePolicyManager
import android.content.ComponentName
import android.content.Context
import android.content.pm.ApplicationInfo
import android.content.pm.PackageManager
import android.net.ConnectivityManager
import android.net.NetworkCapabilities
import android.os.Build
import android.provider.Settings
import java.security.KeyStore

/**
 * Runs every on-device, no-root check and returns grouped findings.
 *
 * Design constraints (why these checks and not others):
 *  - Android sandboxes apps. Without root we cannot read another app's memory,
 *    its keystrokes, or its network sockets. So instead of trying to "tap" other
 *    apps (which is exactly what spyware does), we inspect the *system signals*
 *    that monitoring tools leave behind: accessibility services, device admins,
 *    notification listeners, dangerous permissions, overlays, VPN/proxy,
 *    user-added CAs and the developer/ADB switches.
 *  - Every check is wrapped so a single failure (vendor ROM quirk, missing API
 *    on old Android) never aborts the whole scan.
 */
class SecurityScanner(private val context: Context) {

    fun runAll(): List<CheckSection> = listOf(
        CheckSection("Bedienungshilfen (Keylogger-Vektor)", checkAccessibility()),
        CheckSection("Geräteadministratoren", checkDeviceAdmins()),
        CheckSection("Benachrichtigungs-Zugriff", checkNotificationListeners()),
        CheckSection("App-Berechtigungen", checkDangerousPermissions()),
        CheckSection("Bildschirm-Overlays", checkOverlayApps()),
        CheckSection("Installationsquellen", checkInstallSources()),
        CheckSection("Netzwerk (VPN / Proxy)", checkNetwork()),
        CheckSection("Zertifikate (MITM)", checkUserCertificates()),
        CheckSection("Entwickler / USB-Debugging", checkDeveloperOptions()),
    )

    // ---------------------------------------------------------------------
    // 1. Accessibility services – the #1 vector for keyloggers / screen readers.
    //    A malicious accessibility service can read everything on screen and
    //    every text field, effectively "reading along" what you type.
    // ---------------------------------------------------------------------
    private fun checkAccessibility(): List<Finding> = safe {
        val enabled = Settings.Secure.getString(
            context.contentResolver,
            Settings.Secure.ENABLED_ACCESSIBILITY_SERVICES
        )
        if (enabled.isNullOrBlank()) {
            return@safe listOf(
                Finding(
                    "Keine Bedienungshilfen aktiv",
                    "Kein Dienst hat Zugriff auf Bildschirminhalte/Eingaben. Das ist der gewünschte Zustand.",
                    Severity.OK
                )
            )
        }
        enabled.split(':').filter { it.isNotBlank() }.map { component ->
            val pkg = ComponentName.unflattenFromString(component)?.packageName ?: component
            val label = appLabel(pkg)
            val system = isSystemApp(pkg)
            Finding(
                "Aktiv: $label",
                buildString {
                    append("Dieser Dienst kann den Bildschirminhalt lesen und Eingaben mitschneiden. ")
                    append("Paket: $pkg. ")
                    append(if (system) "(System-App – meist unkritisch.)"
                           else "Von dir/Drittanbieter installiert – nur erlauben, wenn du es kennst!")
                },
                if (system) Severity.INFO else Severity.DANGER
            )
        }
    }

    // ---------------------------------------------------------------------
    // 2. Device administrators – stalkerware often registers as device admin
    //    so it cannot be uninstalled normally and can lock/wipe the phone.
    // ---------------------------------------------------------------------
    private fun checkDeviceAdmins(): List<Finding> = safe {
        val dpm = context.getSystemService(Context.DEVICE_POLICY_SERVICE) as DevicePolicyManager
        val admins: List<ComponentName> = dpm.activeAdmins ?: emptyList()
        if (admins.isEmpty()) {
            return@safe listOf(
                Finding("Keine Geräteadministratoren", "Keine App hat Admin-Rechte. Gut.", Severity.OK)
            )
        }
        admins.map { admin ->
            val pkg = admin.packageName
            val system = isSystemApp(pkg)
            Finding(
                "Admin: ${appLabel(pkg)}",
                "Hat erweiterte Kontrolle (Sperren/Löschen/Richtlinien). Paket: $pkg. " +
                    if (system) "(System/MDM.)" else "Unbekannte Admin-App ist ein starkes Stalkerware-Indiz.",
                if (system) Severity.INFO else Severity.DANGER
            )
        }
    }

    // ---------------------------------------------------------------------
    // 3. Notification listeners – can read the content of every notification,
    //    including incoming messages, OTP codes, etc.
    // ---------------------------------------------------------------------
    private fun checkNotificationListeners(): List<Finding> = safe {
        val enabled = Settings.Secure.getString(context.contentResolver, "enabled_notification_listeners")
        if (enabled.isNullOrBlank()) {
            return@safe listOf(
                Finding("Kein Benachrichtigungs-Mitleser", "Keine App liest deine Benachrichtigungen mit.", Severity.OK)
            )
        }
        enabled.split(':').filter { it.isNotBlank() }.map { component ->
            val pkg = ComponentName.unflattenFromString(component)?.packageName ?: component
            val system = isSystemApp(pkg)
            Finding(
                "Liest mit: ${appLabel(pkg)}",
                "Kann Inhalte aller Benachrichtigungen lesen (Nachrichten, Codes). Paket: $pkg. " +
                    if (system) "(System.)" else "Nur erlauben, wenn bewusst eingerichtet (z. B. Smartwatch).",
                if (system) Severity.INFO else Severity.WARN
            )
        }
    }

    // ---------------------------------------------------------------------
    // 4. Dangerous permissions – list non-system apps that hold spyware-relevant
    //    permissions (mic, camera, SMS, call log, location, usage stats).
    // ---------------------------------------------------------------------
    private fun checkDangerousPermissions(): List<Finding> = safe {
        val watched = mapOf(
            "android.permission.RECORD_AUDIO" to "Mikrofon",
            "android.permission.CAMERA" to "Kamera",
            "android.permission.READ_SMS" to "SMS lesen",
            "android.permission.RECEIVE_SMS" to "SMS empfangen",
            "android.permission.READ_CALL_LOG" to "Anrufliste",
            "android.permission.PROCESS_OUTGOING_CALLS" to "Ausgehende Anrufe",
            "android.permission.ACCESS_FINE_LOCATION" to "Genauer Standort",
            "android.permission.READ_CONTACTS" to "Kontakte",
            "android.permission.PACKAGE_USAGE_STATS" to "App-Nutzung verfolgen",
        )
        val pm = context.packageManager
        val packages = pm.getInstalledPackages(PackageManager.GET_PERMISSIONS)
        val results = mutableListOf<Finding>()
        for (pkgInfo in packages) {
            val appInfo = pkgInfo.applicationInfo ?: continue
            if (isSystemApp(appInfo)) continue           // focus on user-installed apps
            if (pkgInfo.packageName == context.packageName) continue
            val requested = pkgInfo.requestedPermissions ?: continue
            val flags = pkgInfo.requestedPermissionsFlags
            val granted = mutableListOf<String>()
            requested.forEachIndexed { i, perm ->
                val human = watched[perm] ?: return@forEachIndexed
                val isGranted = flags != null && i < flags.size &&
                    (flags[i] and PackageManager.REQUESTED_PERMISSION_GRANTED) != 0
                if (isGranted) granted.add(human)
            }
            if (granted.size >= 2) {                       // combos are the real risk
                val sev = if (granted.contains("Mikrofon") &&
                    (granted.contains("Genauer Standort") || granted.contains("Kamera")))
                    Severity.WARN else Severity.INFO
                results.add(
                    Finding(
                        appLabel(appInfo.packageName),
                        "Hält: ${granted.joinToString(", ")}. Paket: ${appInfo.packageName}. " +
                            "Prüfe, ob diese App diese Rechte wirklich braucht.",
                        sev
                    )
                )
            }
        }
        if (results.isEmpty())
            listOf(Finding("Unauffällig", "Keine Drittanbieter-App mit kritischer Rechte-Kombination.", Severity.OK))
        else results.sortedByDescending { it.severity.ordinal }
    }

    // ---------------------------------------------------------------------
    // 5. Overlay apps (SYSTEM_ALERT_WINDOW) – can draw over other apps to
    //    capture taps / fake login screens.
    // ---------------------------------------------------------------------
    private fun checkOverlayApps(): List<Finding> = safe {
        val pm = context.packageManager
        val packages = pm.getInstalledPackages(PackageManager.GET_PERMISSIONS)
        val results = mutableListOf<Finding>()
        for (pkgInfo in packages) {
            val appInfo = pkgInfo.applicationInfo ?: continue
            if (isSystemApp(appInfo) || pkgInfo.packageName == context.packageName) continue
            val requested = pkgInfo.requestedPermissions ?: continue
            val flags = pkgInfo.requestedPermissionsFlags ?: continue
            val idx = requested.indexOf("android.permission.SYSTEM_ALERT_WINDOW")
            if (idx >= 0 && idx < flags.size &&
                (flags[idx] and PackageManager.REQUESTED_PERMISSION_GRANTED) != 0) {
                results.add(
                    Finding(
                        appLabel(appInfo.packageName),
                        "Darf über andere Apps zeichnen (Overlay). Kann zum Abgreifen von Eingaben " +
                            "missbraucht werden. Paket: ${appInfo.packageName}.",
                        Severity.WARN
                    )
                )
            }
        }
        if (results.isEmpty())
            listOf(Finding("Keine Overlay-Apps", "Keine Drittanbieter-App darf über andere Apps zeichnen.", Severity.OK))
        else results
    }

    // ---------------------------------------------------------------------
    // 6. Install sources – apps not from the Play Store / a known store are
    //    "sideloaded"; not bad per se, but spyware is usually sideloaded.
    // ---------------------------------------------------------------------
    private fun checkInstallSources(): List<Finding> = safe {
        val pm = context.packageManager
        val trusted = setOf(
            "com.android.vending",      // Play Store
            "com.google.android.feedback",
            "com.amazon.venezia",       // Amazon Appstore
            "com.sec.android.app.samsungapps", // Galaxy Store
            "com.huawei.appmarket",
        )
        val packages = pm.getInstalledPackages(0)
        val sideloaded = mutableListOf<Finding>()
        for (pkgInfo in packages) {
            val appInfo = pkgInfo.applicationInfo ?: continue
            if (isSystemApp(appInfo) || pkgInfo.packageName == context.packageName) continue
            val installer = installerOf(pkgInfo.packageName)
            if (installer == null || installer !in trusted) {
                sideloaded.add(
                    Finding(
                        appLabel(appInfo.packageName),
                        "Nicht aus einem bekannten Store installiert (Quelle: ${installer ?: "unbekannt"}). " +
                            "Paket: ${pkgInfo.packageName}.",
                        Severity.INFO
                    )
                )
            }
        }
        if (sideloaded.isEmpty())
            listOf(Finding("Alle aus bekannten Stores", "Keine seitlich geladenen Drittanbieter-Apps gefunden.", Severity.OK))
        else sideloaded
    }

    // ---------------------------------------------------------------------
    // 7. Network – VPN active (traffic could be routed/inspected) and a
    //    global HTTP proxy (classic MITM interception point).
    // ---------------------------------------------------------------------
    private fun checkNetwork(): List<Finding> = safe {
        val results = mutableListOf<Finding>()
        val cm = context.getSystemService(Context.CONNECTIVITY_SERVICE) as ConnectivityManager
        val active = cm.activeNetwork
        val caps = active?.let { cm.getNetworkCapabilities(it) }
        val vpn = caps?.hasTransport(NetworkCapabilities.TRANSPORT_VPN) == true
        results.add(
            if (vpn)
                Finding("VPN aktiv", "Dein Datenverkehr läuft über ein VPN. Wenn du es nicht selbst eingerichtet hast, kann jemand mitlesen.", Severity.WARN)
            else
                Finding("Kein VPN aktiv", "Datenverkehr wird nicht über ein VPN umgeleitet.", Severity.OK)
        )

        val proxyHost = System.getProperty("http.proxyHost")
        val proxyPort = System.getProperty("http.proxyPort")
        if (!proxyHost.isNullOrBlank()) {
            results.add(
                Finding("HTTP-Proxy gesetzt", "Proxy: $proxyHost:$proxyPort. Ein Proxy kann unverschlüsselten Verkehr mitlesen/verändern.", Severity.WARN)
            )
        } else {
            results.add(Finding("Kein Proxy", "Es ist kein System-Proxy konfiguriert.", Severity.OK))
        }
        results
    }

    // ---------------------------------------------------------------------
    // 8. User-added CA certificates – a custom root CA lets the owner decrypt
    //    HTTPS traffic (corporate MITM, or interception spyware).
    // ---------------------------------------------------------------------
    private fun checkUserCertificates(): List<Finding> = safe {
        val ks = KeyStore.getInstance("AndroidCAStore")
        ks.load(null, null)
        val userCerts = ks.aliases().toList().filter { it.startsWith("user:") }
        if (userCerts.isEmpty())
            listOf(Finding("Keine eigenen Zertifikate", "Es sind keine vom Nutzer hinzugefügten Root-Zertifikate installiert.", Severity.OK))
        else
            listOf(
                Finding(
                    "${userCerts.size} eigenes Zertifikat(e)",
                    "Selbst hinzugefügte Root-Zertifikate können dazu benutzt werden, deinen verschlüsselten " +
                        "Verkehr mitzulesen. Prüfe sie unter Einstellungen → Sicherheit → Zertifikate.",
                    Severity.WARN
                )
            )
    }

    // ---------------------------------------------------------------------
    // 9. Developer options & USB debugging – an open attack surface; many
    //    BadUSB/ADB-based attacks need these enabled.
    // ---------------------------------------------------------------------
    private fun checkDeveloperOptions(): List<Finding> = safe {
        val results = mutableListOf<Finding>()
        val adb = Settings.Global.getInt(context.contentResolver, Settings.Global.ADB_ENABLED, 0)
        results.add(
            if (adb == 1)
                Finding("USB-Debugging aktiv", "ADB ist eingeschaltet. Über USB kann auf das Gerät zugegriffen werden. Ausschalten, wenn nicht benötigt.", Severity.WARN)
            else
                Finding("USB-Debugging aus", "ADB ist deaktiviert.", Severity.OK)
        )
        val dev = Settings.Global.getInt(context.contentResolver, Settings.Global.DEVELOPMENT_SETTINGS_ENABLED, 0)
        if (dev == 1)
            results.add(Finding("Entwickleroptionen aktiv", "Die Entwickleroptionen sind aktiviert. Für Normalnutzung besser deaktivieren.", Severity.INFO))
        results
    }

    // ---------------------------------------------------------------------
    // Helpers
    // ---------------------------------------------------------------------

    /** Runs a check, converting any unexpected error into a single INFO finding. */
    private inline fun safe(block: () -> List<Finding>): List<Finding> = try {
        block()
    } catch (t: Throwable) {
        listOf(Finding("Prüfung nicht möglich", "Diese Prüfung konnte auf diesem Gerät nicht ausgeführt werden (${t.javaClass.simpleName}).", Severity.INFO))
    }

    private fun appLabel(pkg: String): String = try {
        val pm = context.packageManager
        pm.getApplicationLabel(pm.getApplicationInfo(pkg, 0)).toString()
    } catch (e: Exception) {
        pkg
    }

    private fun isSystemApp(pkg: String): Boolean = try {
        isSystemApp(context.packageManager.getApplicationInfo(pkg, 0))
    } catch (e: Exception) {
        false
    }

    private fun isSystemApp(info: ApplicationInfo): Boolean =
        (info.flags and (ApplicationInfo.FLAG_SYSTEM or ApplicationInfo.FLAG_UPDATED_SYSTEM_APP)) != 0

    @Suppress("DEPRECATION")
    private fun installerOf(pkg: String): String? = try {
        val pm = context.packageManager
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.R)
            pm.getInstallSourceInfo(pkg).installingPackageName
        else
            pm.getInstallerPackageName(pkg)
    } catch (e: Exception) {
        null
    }
}
