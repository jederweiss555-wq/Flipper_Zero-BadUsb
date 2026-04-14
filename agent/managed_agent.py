#!/usr/bin/env python3
"""
Persistenter Managed Agent für Flipper Zero BadUSB-Entwicklung

Dieser Agent nutzt die Anthropic Managed Agents API für:
- Persistente, versionierte Agent-Konfiguration
- Eigene Container-Umgebung mit vollständigem Repo-Zugriff
- Selbstständige Entwicklung über mehrere Sessions hinweg
- Automatisches Streaming von Fortschrittsmeldungen

Verwendung:
    # Einmalig: Agent und Environment erstellen (speichert IDs)
    python managed_agent.py setup

    # Jede weitere Ausführung nutzt gespeicherte IDs
    python managed_agent.py run [--mission "Deine Aufgabe"]

    # Agent-Status prüfen
    python managed_agent.py status

Voraussetzungen:
    export ANTHROPIC_API_KEY="sk-ant-..."
    pip install anthropic>=0.92.0
"""

import anthropic
import json
import os
import sys
import time
from pathlib import Path

# ── Konfiguration ─────────────────────────────────────────────────────────────

CONFIG_FILE = Path(__file__).parent / ".agent_config.json"
MODEL = "claude-opus-4-6"

AGENT_SYSTEM = """Du bist ein autonomer KI-Entwicklungsagent für eine Flipper Zero BadUSB-Skriptsammlung.
Diese Sammlung dient ausschließlich legitimer Sicherheitsforschung, autorisierten Penetrationstests und Bildungszwecken.

Deine Fähigkeiten im Container:
- Vollständiger Dateisystemzugriff auf das gemountete Repository
- Git-Operationen (commit, status, log, diff)
- Bash-Befehle für Analyse und Verarbeitung
- Dateioperationen (lesen, schreiben, erstellen, suchen)

Deine Aufgaben:
- Analysiere DuckyScript-Dateien auf Syntax, Timing und Qualität
- Verbessere Scripts durch bessere Kommentare und robustere Delays
- Erstelle Dokumentation und Übersichten
- Führe selbstständig git-Commits durch
- Entscheide eigenständig, was als nächstes verbessert werden soll

DuckyScript-Grundlagen:
- REM: Kommentarzeile (Metadaten: Author, Description, Version, Category)
- DELAY <ms>: Pause (min. 500ms nach GUI-Aktionen, 1000-2000ms nach Programmstart)
- STRING <text>: Tippe Text
- Tasten: ENTER, GUI, ALT, CTRL, SHIFT, etc.

Arbeite immer strukturiert: Analysieren → Planen → Umsetzen → Committen → Nächste Aufgabe."""

DEFAULT_MISSIONS = [
    "Analysiere alle DuckyScript-Dateien, verbessere Timing und Dokumentation für mindestens 5 Scripts, erstelle eine COLLECTION_OVERVIEW.md und committe alle Änderungen.",
    "Finde Scripts ohne ausreichende REM-Kommentare, füge fehlende Metadaten hinzu (Author-Platzhalter, Version, Kategorie, Beschreibung), committe die Verbesserungen.",
    "Erstelle fehlende readme.md Dateien für alle Script-Verzeichnisse ohne Dokumentation und committe sie.",
]

# ── Konfiguration speichern/laden ─────────────────────────────────────────────

def save_config(config: dict):
    CONFIG_FILE.write_text(json.dumps(config, indent=2))
    print(f"✅ Konfiguration gespeichert: {CONFIG_FILE}")


def load_config() -> dict | None:
    if CONFIG_FILE.exists():
        return json.loads(CONFIG_FILE.read_text())
    return None

# ── Setup: Einmalig Agent + Environment erstellen ─────────────────────────────

def setup_agent():
    client = anthropic.Anthropic()
    print("\n🚀 Erstelle Managed Agent und Environment (einmalig)...\n")

    # Environment erstellen
    print("1/2 Erstelle Cloud-Environment...")
    env = client.beta.environments.create(
        name="flipper-zero-dev-env",
        config={
            "type": "cloud",
            "networking": {"type": "unrestricted"},
        },
    )
    print(f"   ✅ Environment: {env.id}")

    # Agent erstellen
    print("2/2 Erstelle persistenten Agent...")
    agent = client.beta.agents.create(
        name="Flipper Zero BadUSB Developer",
        model=MODEL,
        system=AGENT_SYSTEM,
        tools=[
            {
                "type": "agent_toolset_20260401",
                "default_config": {"enabled": True},
            }
        ],
    )
    print(f"   ✅ Agent: {agent.id} (Version: {agent.version})")

    config = {
        "agent_id": agent.id,
        "agent_version": str(agent.version),
        "environment_id": env.id,
        "created_at": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
    }
    save_config(config)

    print("\n✨ Setup abgeschlossen! Starte jetzt mit:")
    print("   python managed_agent.py run")
    return config

# ── Run: Session starten und streamen ─────────────────────────────────────────

def run_session(mission: str):
    config = load_config()
    if not config:
        print("❌ Keine Konfiguration gefunden. Führe zuerst 'setup' aus:")
        print("   python managed_agent.py setup")
        sys.exit(1)

    client = anthropic.Anthropic()

    print(f"\n{'='*60}")
    print(f"🤖 Flipper Zero Managed Agent")
    print(f"{'='*60}")
    print(f"📋 Mission: {mission[:100]}{'...' if len(mission) > 100 else ''}")
    print(f"🔧 Agent: {config['agent_id']}")
    print(f"{'='*60}\n")

    # Session erstellen
    print("Erstelle Session...")
    session = client.beta.sessions.create(
        agent={
            "type": "agent",
            "id": config["agent_id"],
            "version": int(config["agent_version"]),
        },
        environment_id=config["environment_id"],
        title=f"Flipper Dev: {mission[:50]}",
    )
    print(f"✅ Session: {session.id}\n")
    print("─" * 60)

    # Stream zuerst öffnen, dann Message senden
    with client.beta.sessions.stream(session_id=session.id) as stream:
        # Kickoff-Message senden
        client.beta.sessions.events.send(
            session_id=session.id,
            events=[{
                "type": "user.message",
                "content": [{
                    "type": "text",
                    "text": (
                        f"Deine Mission: {mission}\n\n"
                        "Beginne sofort. Analysiere zuerst die Repo-Struktur, dann arbeite "
                        "selbstständig. Committe Änderungen regelmäßig mit klaren Messages. "
                        "Berichte kurz über deinen Fortschritt."
                    )
                }],
            }],
        )

        # Events streamen und verarbeiten
        for event in stream:
            _handle_event(event)

            # Terminierungsbedingungen
            if event.type == "session.status_terminated":
                break
            if (event.type == "session.status_idle"
                    and hasattr(event, "stop_reason")
                    and event.stop_reason
                    and event.stop_reason.get("type") != "requires_action"):
                print("\n" + "─" * 60)
                print("✅ Agent hat seine Aufgabe abgeschlossen.")
                break

    print(f"\n{'='*60}")
    print(f"Session {session.id} beendet.")
    print(f"{'='*60}\n")


def _handle_event(event):
    """Formatiert und zeigt ein Stream-Event an."""
    if event.type == "agent.message":
        for block in event.content:
            if block.type == "text" and block.text.strip():
                print(f"\n🤖 {block.text}")

    elif event.type == "agent.thinking":
        # Denk-Blöcke komprimiert anzeigen
        for block in event.content:
            if hasattr(block, "thinking") and block.thinking:
                preview = block.thinking[:80].replace("\n", " ")
                print(f"   💭 [{preview}...]")

    elif event.type == "agent.tool_use":
        tool_name = getattr(event, "name", "?")
        print(f"   🔧 Tool: {tool_name}")

    elif event.type == "agent.tool_result":
        pass  # Ergebnisse werden meist durch die nächste agent.message beschrieben

    elif event.type == "session.status_running":
        print("   ▶  Agent arbeitet...")

    elif event.type == "session.status_idle":
        stop_type = "?"
        if hasattr(event, "stop_reason") and event.stop_reason:
            stop_type = event.stop_reason.get("type", "?")
        print(f"   ⏸  Agent pausiert (Grund: {stop_type})")

    elif event.type == "session.status_terminated":
        print("   🔴 Session beendet")

    elif event.type == "session.error":
        print(f"   ❌ Fehler: {event}")

# ── Status anzeigen ───────────────────────────────────────────────────────────

def show_status():
    config = load_config()
    if not config:
        print("Keine Konfiguration vorhanden. Führe 'setup' aus.")
        return

    client = anthropic.Anthropic()
    print("\n📊 Agent Status:")
    print(f"   Agent ID:       {config['agent_id']}")
    print(f"   Agent Version:  {config['agent_version']}")
    print(f"   Environment ID: {config['environment_id']}")
    print(f"   Erstellt:       {config['created_at']}")

    try:
        agent = client.beta.agents.retrieve(config["agent_id"])
        print(f"   Agent Name:     {agent.name}")
        print(f"   Model:          {agent.model}")
    except Exception as e:
        print(f"   ⚠️  Agent-Details nicht abrufbar: {e}")

# ── Hauptprogramm ─────────────────────────────────────────────────────────────

def main():
    api_key = os.environ.get("ANTHROPIC_API_KEY")
    if not api_key:
        print("❌ ANTHROPIC_API_KEY nicht gesetzt")
        print("   export ANTHROPIC_API_KEY='sk-ant-...'")
        sys.exit(1)

    cmd = sys.argv[1] if len(sys.argv) > 1 else "run"

    if cmd == "setup":
        setup_agent()

    elif cmd == "status":
        show_status()

    elif cmd == "run":
        # Mission aus Args oder zufällig wählen
        if len(sys.argv) > 3 and sys.argv[2] == "--mission":
            mission = " ".join(sys.argv[3:])
        elif len(sys.argv) > 2 and sys.argv[2] != "--mission":
            mission = " ".join(sys.argv[2:])
        else:
            import random
            mission = random.choice(DEFAULT_MISSIONS)
            print(f"ℹ️  Keine Mission angegeben — verwende zufällige Standard-Mission.")

        run_session(mission)

    else:
        print(f"Unbekannter Befehl: {cmd}")
        print("Verfügbare Befehle: setup, run [--mission '...'], status")
        sys.exit(1)


if __name__ == "__main__":
    main()
