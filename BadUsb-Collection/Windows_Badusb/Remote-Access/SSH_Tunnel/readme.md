# Reverse SSH Tunnel

Creates a reverse SSH tunnel that forwards the target's RDP port (3389) to the attacker's SSH server.

- **Requirements:**
  - Replace `SSH_USERNAME`, `SSH_SERVER_IP`, `REMOTE_PORT` with your server details
  - Replace `SSH_PRIVATE_KEY_BASE64` with base64-encoded private key content
  - OpenSSH client must be installed on target (built-in on Windows 10 1809+)
- **Access:** After tunnel is up, connect via `localhost:REMOTE_PORT` on your SSH server using RDP
- **Note:** Ensure `GatewayPorts yes` and `AllowTcpForwarding yes` in server's `sshd_config`
