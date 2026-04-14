# UAC Bypass via fodhelper.exe

Uses the fodhelper.exe auto-elevation trick to run a payload with high integrity (bypasses UAC prompt).

- **Works on:** Windows 10 / 11 (all versions as of 2024)
- **Privilege required:** Medium integrity user
- **Result:** Elevated (High integrity) PowerShell session
- **Detection:** Medium — registry key creation is logged

## Setup
Replace `PAYLOAD_URL` with your hosted PowerShell script URL.
