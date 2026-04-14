# AMSI Bypass (In-Process Patch)

Patches the `amsiInitFailed` field in the current PowerShell process to disable AMSI scanning for that session. Allows loading otherwise detected scripts.

- **Privilege required:** None (user-level, in-process)
- **Scope:** Current PowerShell session only
- **Detection:** Most EDR solutions detect this specific reflection pattern — combine with obfuscation

## Setup
Replace `PAYLOAD_URL` with your script to execute after bypassing AMSI.
