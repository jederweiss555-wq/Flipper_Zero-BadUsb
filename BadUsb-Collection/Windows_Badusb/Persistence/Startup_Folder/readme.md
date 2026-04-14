# Startup Folder Persistence

Drops a VBScript (`update.vbs`) into the user's Startup folder. Executes silently on every login.

- **Privilege required:** None (user-level)
- **Detection:** Low-Medium — startup folder is commonly monitored
- **File dropped:** `%APPDATA%\Microsoft\Windows\Start Menu\Programs\Startup\update.vbs`

## Setup
Replace `PAYLOAD_URL` with your hosted PowerShell script URL.
