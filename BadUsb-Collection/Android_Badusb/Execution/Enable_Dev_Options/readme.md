# Enable Android Developer Options

Attempts to navigate to Android Settings > About Phone to enable Developer Options by tapping Build Number.

- **Note:** Android's UI varies heavily between manufacturers and versions. This script opens the Settings search for "about" — actual tapping of Build Number 7 times must be done manually or combined with a custom tap macro (requires ADB/root for reliable automation).
- **Alternative:** Use ADB: `adb shell settings put global development_settings_enabled 1`
- **Requirements:** Device unlocked, Settings app accessible via keyboard
