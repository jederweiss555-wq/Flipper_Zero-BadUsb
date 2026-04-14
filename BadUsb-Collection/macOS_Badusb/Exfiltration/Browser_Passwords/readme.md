# macOS Browser Password Dump

Copies Chrome, Edge, and Safari credential/history databases and exports keychain web passwords.

- **Output:** `/tmp/browser_dump.tar.gz` containing DB files and keychain export
- **Note:** Chrome/Edge `Login Data` is encrypted with a key stored in the macOS keychain. Requires the `chrome-decrypt` or similar tool for decryption. Browsers must be closed.
- **Post-processing:** Use [chromium-credential-extractor](https://github.com/manwhoami/Keychain-Dumper) or similar on macOS to decrypt
