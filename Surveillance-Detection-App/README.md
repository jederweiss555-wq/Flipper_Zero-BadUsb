# PrivacyGuard – Überwachungs- & Spyware-Check für Android

Eine **defensive** Android-App, die *dein eigenes* Telefon lokal auf Anzeichen
von Überwachung, Stalkerware oder Kompromittierung prüft. Sie läuft komplett
offline, fordert **keine** gefährlichen Berechtigungen an und sendet **nichts**
ins Internet — die gesamte Auswertung bleibt auf dem Gerät.

> Gedacht zum Schutz von Geräten, die dir gehören oder für die du autorisiert
> bist. Nutze sie nicht, um andere Menschen ohne deren Wissen auszuspähen.

---

## Wichtig zuerst: Was eine App auf Android (ohne Root) kann – und was nicht

Android kapselt jede App in einer **Sandbox**. Eine normale App kann deshalb
**nicht**:

- die Tastatureingaben anderer Apps mitlesen,
- den Netzwerkverkehr oder die Prozesse anderer Apps direkt einsehen,
- den Speicher anderer Apps auslesen.

Das ist eine Schutzfunktion des Betriebssystems – genau diese Grenze ist es, die
echte Spyware umgehen will. Eine seriöse Schutz-App tut das **nicht**.

Stattdessen prüft PrivacyGuard die **System-Signale**, an denen man Überwachung
in der Praxis erkennt. Stalkerware und Keylogger brauchen fast immer einen
dieser Mechanismen, und genau die macht die App sichtbar.

## Was geprüft wird

| Prüfung | Warum es relevant ist (deine 4 Schwerpunkte) |
|---|---|
| **Bedienungshilfen / Accessibility-Dienste** | Wichtigster **Keylogger-Vektor**: ein solcher Dienst kann Bildschirminhalte lesen und Eingaben mitschneiden. |
| **Geräteadministratoren** | Stalkerware registriert sich oft als Admin (**Persistenz**: lässt sich schwer löschen, kann sperren/wipen). |
| **Benachrichtigungs-Mitleser** | Kann Inhalte aller Benachrichtigungen lesen (Nachrichten, OTP-Codes) – **Mitlesen**. |
| **App-Berechtigungen** | Drittanbieter-Apps mit Mikrofon/SMS/Anrufliste/Standort-Kombinationen – **Eingaben/Abhören**. |
| **Bildschirm-Overlays (SYSTEM_ALERT_WINDOW)** | Können Eingaben über gefälschte Oberflächen abgreifen – **Keylogger-/Phishing-Vektor**. |
| **Installationsquellen** | Seitlich geladene Apps (nicht aus dem Store) – häufiger **Persistenz**-Weg für Spyware. |
| **VPN / Proxy aktiv** | Datenverkehr könnte umgeleitet/mitgelesen werden – **Netzwerk-Überwachung**. |
| **Eigene Zertifikate (User-CAs)** | Selbst hinzugefügte Root-Zertifikate ermöglichen **MITM** auf verschlüsselten Verkehr – **Netzwerk/Integrität**. |
| **USB-Debugging / Entwickleroptionen** | Offene Angriffsfläche (u. a. für BadUSB/ADB-Angriffe) – **Integrität**. |

Jeder Befund wird eingestuft als **OK / INFO / ACHTUNG / GEFAHR**, mit einer
kurzen Erklärung, was zu tun ist.

## Bauen

Voraussetzung: **Android Studio** (Giraffe oder neuer) oder das Android SDK mit
Gradle.

1. Ordner `Surveillance-Detection-App/` in Android Studio öffnen
   (*File → Open*).
2. Android Studio lädt Gradle und Abhängigkeiten automatisch.
3. Telefon per USB anschließen (USB-Debugging temporär aktivieren) **oder** einen
   Emulator starten.
4. Auf **Run ▶** klicken.

Per Kommandozeile (wenn das Gradle-Wrapper-JAR vorhanden ist – Android Studio
erzeugt es beim ersten Öffnen automatisch, alternativ `gradle wrapper` ausführen):

```bash
./gradlew assembleDebug
# Ergebnis: app/build/outputs/apk/debug/app-debug.apk
```

APK installieren:

```bash
adb install -r app/build/outputs/apk/debug/app-debug.apk
```

## Bedienung

App öffnen → **Scan starten**. Nach wenigen Sekunden erscheint eine Liste
gruppierter Befunde. Beginne mit allen **GEFAHR**- und **ACHTUNG**-Einträgen.

## Wenn du echten Verdacht hast

Diese App ist ein **Indikator**, kein forensisches Gutachten. Bei konkretem
Verdacht zusätzlich:

- Verdächtige Bedienungshilfen/Geräteadmins/Benachrichtigungszugriffe
  **deaktivieren** und die zugehörige App deinstallieren.
- Eigene/unbekannte Zertifikate unter *Einstellungen → Sicherheit →
  Zertifikate* entfernen.
- Im Ernstfall: Daten sichern und **Werksreset** durchführen; danach Passwörter
  von einem sauberen Gerät aus ändern und 2-Faktor-Authentifizierung aktivieren.
- Android und alle Apps aktuell halten.

## Datenschutz

Keine Internet-Berechtigung, keine Telemetrie, keine Datenspeicherung außerhalb
des laufenden Scans. `QUERY_ALL_PACKAGES` wird nur benötigt, um installierte
Apps für die Analyse aufzulisten.
