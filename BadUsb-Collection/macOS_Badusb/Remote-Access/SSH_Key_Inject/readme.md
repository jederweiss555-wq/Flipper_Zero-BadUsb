# macOS SSH Key Injection

Adds an attacker's SSH public key to the target's `~/.ssh/authorized_keys` for persistent SSH access.

- **Requirements:**
  - Replace `ATTACKER_SSH_PUBLIC_KEY` with your full `id_rsa.pub` or `id_ed25519.pub` content
  - Remote Login (SSH) must be enabled on target: System Preferences > Sharing > Remote Login
- **Access:** `ssh user@target-ip` using the corresponding private key
- **Note:** Works only if SSH is enabled; combine with a script that enables Remote Login first
