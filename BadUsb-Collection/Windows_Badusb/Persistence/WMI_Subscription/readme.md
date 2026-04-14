# WMI Event Subscription Persistence

Creates a WMI event filter + consumer that triggers at 12:00 daily. Very stealthy — not stored on disk as a file.

- **Privilege required:** Admin
- **Detection:** Low — WMI subscriptions are rarely monitored
- **Trigger:** Daily at 12:00

## Setup
Replace `PAYLOAD_URL` with your hosted PowerShell script URL.

## Cleanup
```powershell
Get-WMIObject -Namespace root\subscription -Class __EventFilter | Where-Object {$_.Name -eq 'WinUpdate'} | Remove-WmiObject
Get-WMIObject -Namespace root\subscription -Class CommandLineEventConsumer | Where-Object {$_.Name -eq 'WinUpdate'} | Remove-WmiObject
Get-WMIObject -Namespace root\subscription -Class __FilterToConsumerBinding | Remove-WmiObject
```
