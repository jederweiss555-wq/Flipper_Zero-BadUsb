# FTP Upload Exfiltration

Collects system information and uploads it as a text file to an FTP server.

- **Output:** `report.txt` uploaded to FTP server root
- **Requirements:** FTP server with credentials (replace `FTP_SERVER_IP`, `FTP_USERNAME`, `FTP_PASSWORD`)
- **Note:** Uses built-in .NET `FtpWebRequest` — no external tools required
