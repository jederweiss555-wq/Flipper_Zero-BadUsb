# Telegram Bot Exfiltration

Sends system information (hostname, user, OS, local IP, public IP) to a Telegram chat via a bot.

- **Output:** Formatted Telegram message to specified chat
- **Requirements:** Telegram bot token and chat ID (replace `TELEGRAM_BOT_TOKEN` and `TELEGRAM_CHAT_ID`)
- **Setup:** Create a bot via @BotFather to get the token; send a message to your bot and use `getUpdates` to find your chat ID
