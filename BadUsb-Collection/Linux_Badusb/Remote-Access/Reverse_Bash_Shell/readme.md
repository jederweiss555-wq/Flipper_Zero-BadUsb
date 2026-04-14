# Linux Reverse Bash Shell

Opens a reverse bash shell to an attacker-controlled netcat listener.

- **Requirements:**
  - Replace `ATTACKER_IP` with your IP address
  - Replace `ATTACKER_PORT` with your listener port (e.g. `4444`)
  - Listener: `nc -lvnp 4444` on attacker machine
- **Note:** Uses `/dev/tcp` — built into bash, no extra tools needed. Runs in background (`&`).
- **CTRL+ALT+T** opens terminal on most GNOME/KDE desktops. Adjust for other DEs.
