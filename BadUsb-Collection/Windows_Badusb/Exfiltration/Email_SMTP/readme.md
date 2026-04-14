# Email Exfiltration via SMTP

Collects basic system info (hostname, user, OS, IP) and sends it as an email via SMTP.

- **Output:** Email sent to configured recipient
- **Requirements:** SMTP credentials (replace `SENDER_EMAIL`, `RECEIVER_EMAIL`, `SMTP_PASSWORD`)
- **Note:** Default config uses Gmail SMTP (smtp.gmail.com:587). For Gmail, use an App Password, not your main password. Less Secure App access must be enabled or App Passwords used.
