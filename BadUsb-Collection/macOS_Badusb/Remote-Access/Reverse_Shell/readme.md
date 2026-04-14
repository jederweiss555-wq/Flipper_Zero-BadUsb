# macOS Reverse Bash Shell

Spawns a reverse bash shell connecting back to an attacker listener.

- **Requirements:**
  - Replace `ATTACKER_IP` with your IP address
  - Replace `ATTACKER_PORT` with your port (e.g. `4444`)
  - Listener: `nc -lvnp 4444` on attacker machine
- **Note:** Runs in background (`&`) so terminal can be closed without killing shell. macOS firewall or Little Snitch may block outbound connections.
