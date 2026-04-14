# Linux Cron Job Persistence

Adds a cron job to the current user's crontab that fetches and executes a remote payload every minute.

- **Requirements:** Replace `PAYLOAD_URL` with the URL of your shell script payload
- **Schedule:** `* * * * *` = every minute (change as needed, e.g. `@reboot` for run-at-boot)
- **Note:** Appends to existing crontab — does not overwrite other jobs. No root required for user-level cron.
