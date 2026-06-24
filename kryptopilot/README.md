# KryptoPilot

KI-gesteuerte Krypto-Analyse und Alerts für Android (Tauri 2 + Svelte 5 + Rust).

## Features

- Watchlist mit Live-Preisen (CoinGecko)
- KI-Analyse on demand (Gemini 2.5 Flash / Groq Llama 3.3 70B)
- Smart Alerts (RSI, Preis, Volumen)
- Tages-Briefing 8:00 Uhr
- API-Keys verschlüsselt gespeichert

## Setup (Desktop Dev)

```bash
pnpm install
pnpm tauri dev
```

## Android Setup

### Voraussetzungen

1. **Android Studio** installieren: https://developer.android.com/studio
2. **ANDROID_HOME** setzen (in `~/.bashrc` oder `~/.zshrc`):
   ```bash
   export ANDROID_HOME=$HOME/Android/Sdk
   export PATH=$PATH:$ANDROID_HOME/tools:$ANDROID_HOME/platform-tools
   ```
3. **NDK** installieren (in Android Studio → SDK Manager → SDK Tools → NDK):
   ```bash
   export NDK_HOME=$ANDROID_HOME/ndk/$(ls $ANDROID_HOME/ndk | head -1)
   ```
4. **Rust Android Targets** hinzufügen:
   ```bash
   rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
   ```

### Android Build

```bash
# Initialisieren (einmalig)
pnpm tauri android init

# Entwicklung (Emulator oder USB-Gerät)
pnpm tauri android dev

# Release Build
pnpm tauri android build
```

## API-Keys

Alle Keys werden verschlüsselt in der lokalen SQLite-DB gespeichert:

| Provider     | Woher?                                              | Limits        |
|-------------|-----------------------------------------------------|---------------|
| Gemini      | https://aistudio.google.com/app/apikey              | 1500 req/day  |
| Groq        | https://console.groq.com/keys                       | 14400 req/day |
| CryptoPanic | https://cryptopanic.com/developers/api/             | Free Tier     |

## Stack

- Tauri 2 (Android Target)
- Svelte 5 (Runes) + SvelteKit SPA
- TypeScript strict
- Tailwind CSS v4
- Rust + tokio + reqwest + sqlx (SQLite)
