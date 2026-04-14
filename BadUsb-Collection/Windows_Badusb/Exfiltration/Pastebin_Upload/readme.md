# Pastebin Upload Exfiltration

Collects system info and posts it as a private, 1-day expiry paste on Pastebin.

- **Output:** Paste URL saved to `%TEMP%\pb_url.txt`
- **Requirements:** Pastebin API key (replace `PASTEBIN_API_KEY`); free account works
- **Note:** Paste is set to private (`api_paste_private=1`) and expires in 1 day
