# Netcat Reverse Shell

Downloads `ncat.exe` and connects back to an attacker listener with a CMD shell.

- **Requirements:**
  - Replace `ATTACKER_IP` with your IP address
  - Replace `ATTACKER_PORT` with your listener port (e.g. `4444`)
  - Replace `NCAT_DOWNLOAD_URL` with a URL hosting `ncat.exe` (from nmap's ncat package)
- **Listener:** `nc -lvnp 4444` on attacker machine
- **Note:** Many AV solutions detect `nc.exe` — consider obfuscating or using PowerShell-native reverse shells instead
