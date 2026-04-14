# Registry Run Key Persistence

Adds a registry entry to `HKCU\Software\Microsoft\Windows\CurrentVersion\Run` so the payload executes every time the user logs in.

- **Privilege required:** None (user-level)
- **Detection:** Medium — standard AV checks this key
- **Persistence location:** `HKCU\...\Run\WindowsUpdate`

## Setup
Replace `PAYLOAD_URL` with your hosted PowerShell script URL.
