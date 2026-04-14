# Scheduled Task Persistence

Creates a Windows Scheduled Task named `WindowsDefenderUpdate` that runs the payload every hour.

- **Privilege required:** Admin (for elevated task)
- **Detection:** Medium — task name mimics legitimate Windows tasks
- **Trigger:** Every 1 hour

## Setup
Replace `PAYLOAD_URL` with your hosted PowerShell script URL.
