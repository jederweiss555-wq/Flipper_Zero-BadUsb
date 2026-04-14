# Steal Browser Cookies

Copies cookie database files from Chrome, Edge, Brave, and Firefox to a ZIP archive in TEMP.

- **Output:** `%TEMP%\cookies.zip` containing raw cookie SQLite/DB files
- **Note:** Cookie files are locked while the browser is open — close browsers first or combine with a browser-kill step
- **Post-processing:** Use a tool like `sqlite3` or `editthiscookie` to read the SQLite databases. Chrome/Edge cookies are encrypted with DPAPI and require decryption using the `Local State` file's `encrypted_key`
