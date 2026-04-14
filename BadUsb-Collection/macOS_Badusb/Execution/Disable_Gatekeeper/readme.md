# Disable macOS Gatekeeper

Disables Gatekeeper so that unsigned and unnotarized applications can run without warnings.

- **Requirements:** Admin (sudo) access — Terminal will prompt for password
- **Note:** Also disables the GKAutoRearm feature so Gatekeeper doesn't re-enable itself
- **Re-enable:** `sudo spctl --master-enable`
- **macOS 13+:** Gatekeeper enforcement is stricter; System Settings > Privacy & Security may still block some apps
