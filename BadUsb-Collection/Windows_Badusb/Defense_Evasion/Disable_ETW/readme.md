# Disable ETW (Event Tracing for Windows)

Patches ETW in the current PowerShell session to suppress telemetry sent to Windows Defender and other security tools.

- **Privilege required:** None (user-level, in-process)
- **Scope:** Current PowerShell session only
- **Effect:** Reduces visibility of executed commands for EDR/AV
