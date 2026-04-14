# Autonomer Flipper Zero Entwicklungsagent

Ein KI-Agent, der selbstständig die Flipper Zero BadUSB-Skriptsammlung analysiert, verbessert und weiterentwickelt.

> **Hinweis:** Alle Scripts in dieser Sammlung dienen ausschließlich legitimer Sicherheitsforschung und autorisierten Penetrationstests.

---

## Zwei Varianten

### 1. `agent.py` — Einfacher Tool-Use Agent

Direkter Claude API Agent mit manuell implementierter Tool-Schleife.

```bash
export ANTHROPIC_API_KEY="sk-ant-..."
pip install anthropic>=0.92.0

# Zufällige Standard-Mission
python agent.py

# Eigene Mission
python agent.py --mission "Analysiere alle Scripts und verbessere die Dokumentation"
```

**Funktionsweise:**
- Nutzt `claude-opus-4-6` mit adaptivem Thinking
- Eigene Tool-Implementierungen: Dateien lesen/schreiben, Verzeichnisse auflisten, Bash-Befehle, Suche
- Prompt-Caching für den System-Prompt (Kostenoptimierung)
- Bis zu 30 Iterationen pro Run

---

### 2. `managed_agent.py` — Persistenter Managed Agent

Nutzt die Anthropic Managed Agents API für persistente, versionierte Agent-Konfiguration.

```bash
export ANTHROPIC_API_KEY="sk-ant-..."

# Einmalig: Agent und Environment erstellen
python managed_agent.py setup

# Agent ausführen (nutzt gespeicherte IDs)
python managed_agent.py run

# Mit eigener Mission
python managed_agent.py run --mission "Erstelle fehlende readme.md Dateien"

# Status anzeigen
python managed_agent.py status
```

**Vorteile gegenüber agent.py:**
- Agent-Konfiguration persistent gespeichert (kein Neu-Erstellen bei jedem Run)
- Versionierung: Änderungen am Agent erstellen neue Versionen
- Eigener Container mit vollständigem Bash/Datei-Zugriff
- Streaming von Fortschrittsmeldungen in Echtzeit

---

## Was der Agent automatisch tut

Der Agent entscheidet selbstständig, was er verbessert:

- **Script-Analyse:** Prüft DuckyScript auf Syntax-Fehler und Timing-Probleme
- **Dokumentation:** Fügt fehlende REM-Kommentare (Author, Description, Version, Category) hinzu
- **Timing-Optimierung:** Verbessert zu kurze Delays (min. 500ms nach GUI-Aktionen)
- **Übersichten:** Erstellt strukturierte Dokumentationsdateien
- **Git-Commits:** Committet alle Änderungen mit aussagekräftigen Messages

## Standard-Missionen

Wenn keine Mission angegeben wird, wählt der Agent zufällig eine aus:

1. Analyse + Timing-Verbesserung + `COLLECTION_OVERVIEW.md` erstellen
2. Fehlende Metadaten (REM-Kommentare) in allen Scripts ergänzen
3. Fehlende `readme.md` Dateien in Script-Verzeichnissen erstellen

## Eigene Mission definieren

```bash
python agent.py --mission "Durchsuche alle Scripts nach dem Befehl 'DELAY' und stelle sicher, dass nach GUI r immer mindestens 500ms Delay folgt. Committe die Änderungen."
```
