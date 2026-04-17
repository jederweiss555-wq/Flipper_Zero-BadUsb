# KryptoPilot – Claude Briefing

## Projekt-Kontext

Android-App für KI-gesteuerte Krypto-Analyse und Alerts.

**Stack:** Tauri 2 Mobile · Svelte 5 Runes · TypeScript strict · Tailwind CSS v4 · Rust (tokio, reqwest, sqlx, anyhow, tracing)

## KI-Provider

| Provider | Endpoint | Auth | Limits |
|---------|----------|------|--------|
| Gemini 2.5 Flash (Primary) | `https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent` | `?key=API_KEY` | 1500/day, 10 RPM |
| Groq Llama 3.3 70B (Fallback) | `https://api.groq.com/openai/v1/chat/completions` | `Bearer` | 30 RPM, 14400/day |

**Router:** Gemini → bei 429/5xx → Groq → bei Fehler → Error an UI.

## Daten-APIs

- **CoinGecko** (kein Key): `/coins/markets`, `/coins/{id}/market_chart`
- **CryptoPanic** (Key in Settings): `/api/v1/posts/`

## Code-Regeln

**Rust:** `anyhow::Result<T>`, `tracing::{info,warn,error}`, kein `.unwrap()` außer Tests, API-Keys nur Backend, XOR+base64 Verschlüsselung.

**Svelte:** Runes ($state, $derived, $effect), kein `export let`, `invoke()` aus `@tauri-apps/api/core`.

## System-Prompt für KI

```
Du bist ein sachlicher Krypto-Analyst. Antworte strukturiert in Markdown mit Sektionen:
Marktlage, Technische Indikatoren, Unterstützung/Widerstand, News-Sentiment, Fazit.
Keine Finanzberatung, keine Kauf-Empfehlungen.
```

## SQLite-Schema

- `watchlist(coin_id PK, symbol, name, added_at)`
- `alert_rules(id PK, coin_id, metric, operator, value, enabled, last_triggered)`
- `settings(key PK, value_encrypted)` ← XOR+base64
- `analysis_cache(coin_id PK, analysis, created_at)`

## Hinweis: cargo check auf Linux

`cargo check` auf Desktop-Linux benötigt GTK3-Entwicklungspakete (`libgtk-3-dev`, `libgdk-pixbuf2.0-dev`, `libpango1.0-dev`). Für Android-only-Builds: `cargo check --target aarch64-linux-android` (Android NDK + Rust-Targets nötig, siehe README).

## Phasen-Status

- [x] Phase 1 – Setup & Gerüst
- [ ] Phase 2 – Daten-Layer (Rust)
- [ ] Phase 3 – KI-Layer (Rust)
- [ ] Phase 4 – UI (Svelte)
- [ ] Phase 5 – Background & Notifications
- [ ] Phase 6 – Android-Build
