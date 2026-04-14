# Linux SSH Key Injection

Injects an attacker's public SSH key into the current user's `~/.ssh/authorized_keys`.

- **Requirements:**
  - Replace `ATTACKER_SSH_PUBLIC_KEY` with your full public key string (`ssh-ed25519 AAAA...`)
  - SSH daemon (`sshd`) must be running on the target
- **Access:** `ssh user@target-ip` from attacker machine using corresponding private key
- **Note:** Creates `~/.ssh/` directory if it doesn't exist. Persistent across reboots.
