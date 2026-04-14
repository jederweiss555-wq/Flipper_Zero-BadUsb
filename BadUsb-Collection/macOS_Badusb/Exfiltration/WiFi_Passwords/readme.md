# macOS WiFi Password Dump

Extracts all saved WiFi passwords from the macOS keychain and sends them to a Discord webhook.

- **Output:** WiFi SSID and password pairs posted to Discord
- **Requirements:** Internet connection, Discord webhook URL (replace `DISCORD_WEBHOOK_URL`)
- **Note:** macOS will show a keychain access prompt for each network — requires user interaction or prior keychain unlock. Tested on macOS Big Sur and later.
