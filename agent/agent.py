#!/usr/bin/env python3
"""
Autonomer Flipper Zero BadUSB Entwicklungsagent

Dieser Agent analysiert, verbessert und entwickelt selbstständig die
Flipper Zero DuckyScript-Sammlung unter Verwendung der Claude API.

Verwendung:
    export ANTHROPIC_API_KEY="sk-ant-..."
    python agent.py [--mission "Beschreibe die Aufgabe"]

Voraussetzungen (Sicherheitshinweis):
    Diese Skripte sind für legitime Sicherheitsforschung und autorisierte
    Penetrationstests bestimmt. Verwende sie nur auf Systemen, für die du
    eine ausdrückliche Genehmigung hast.
"""

import anthropic
import json
import os
import subprocess
import sys
from pathlib import Path

# ── Konstanten ────────────────────────────────────────────────────────────────

REPO_ROOT = Path(__file__).parent.parent.resolve()
MODEL = "claude-opus-4-6"
MAX_TOKENS = 16384
MAX_ITERATIONS = 30  # Schutzmechanismus gegen Endlosschleifen

SYSTEM_PROMPT = """Du bist ein autonomer KI-Entwicklungsagent für eine Flipper Zero BadUSB-Skriptsammlung.
Diese Sammlung dient ausschließlich legitimer Sicherheitsforschung, autorisierten Penetrationstests und Bildungszwecken.

Deine Aufgaben:
- Analysiere DuckyScript-Dateien auf Syntax-Fehler, Timing-Probleme und Verbesserungspotenzial
- Verbessere bestehende Scripts (Kommentare, robustere Delays, klarere Dokumentation)
- Organisiere und kategorisiere die Sammlung
- Erstelle Zusammenfassungen und Berichte über die Sammlung
- Führe git-Operationen durch (add, commit mit aussagekräftigen Messages)
- Entscheide selbstständig, was als nächstes verbessert werden soll

DuckyScript-Grundlagen:
- REM: Kommentarzeile
- DELAY <ms>: Warte in Millisekunden
- STRING <text>: Tippe Text
- ENTER, GUI, ALT, CTRL, SHIFT: Sondertasten
- Typische Delays: 500ms nach GUI-Aktionen, 1000-2000ms nach Programmstart

Wichtig: Arbeite selbstständig und gründlich. Wenn du eine Aufgabe abgeschlossen hast,
entscheide eigenständig, was als nächstes verbessert werden soll.
Committe regelmäßig deine Änderungen mit aussagekräftigen git-Commit-Messages."""

# ── Tool-Definitionen ─────────────────────────────────────────────────────────

TOOLS = [
    {
        "name": "list_directory",
        "description": "Listet alle Dateien und Ordner in einem Verzeichnis auf. Gibt Pfade relativ zum Repo-Root zurück.",
        "input_schema": {
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Pfad relativ zum Repo-Root (z.B. 'BadUsb-Collection' oder '.')"
                },
                "recursive": {
                    "type": "boolean",
                    "description": "Wenn true, werden auch Unterverzeichnisse aufgelistet"
                }
            },
            "required": ["path"]
        }
    },
    {
        "name": "read_file",
        "description": "Liest den Inhalt einer Datei aus dem Repository.",
        "input_schema": {
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Dateipfad relativ zum Repo-Root"
                }
            },
            "required": ["path"]
        }
    },
    {
        "name": "write_file",
        "description": "Schreibt oder überschreibt eine Datei im Repository. Erstellt das Verzeichnis automatisch, falls nötig.",
        "input_schema": {
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Dateipfad relativ zum Repo-Root"
                },
                "content": {
                    "type": "string",
                    "description": "Inhalt der Datei"
                }
            },
            "required": ["path", "content"]
        }
    },
    {
        "name": "bash_command",
        "description": "Führt einen Shell-Befehl im Repo-Root aus. Geeignet für git-Operationen, grep, find, etc.",
        "input_schema": {
            "type": "object",
            "properties": {
                "command": {
                    "type": "string",
                    "description": "Shell-Befehl (wird im Repo-Root ausgeführt)"
                }
            },
            "required": ["command"]
        }
    },
    {
        "name": "search_files",
        "description": "Durchsucht Dateien im Repository nach einem Muster.",
        "input_schema": {
            "type": "object",
            "properties": {
                "pattern": {
                    "type": "string",
                    "description": "Suchmuster (regulärer Ausdruck oder einfacher Text)"
                },
                "path": {
                    "type": "string",
                    "description": "Suchpfad relativ zum Repo-Root (Standard: '.')"
                },
                "file_pattern": {
                    "type": "string",
                    "description": "Dateinamenmuster (z.B. '*.txt' für alle Textdateien)"
                }
            },
            "required": ["pattern"]
        }
    },
    {
        "name": "finish",
        "description": "Beendet die aktuelle Aufgabe mit einer Zusammenfassung der durchgeführten Arbeiten.",
        "input_schema": {
            "type": "object",
            "properties": {
                "summary": {
                    "type": "string",
                    "description": "Kurze Zusammenfassung der abgeschlossenen Arbeit"
                }
            },
            "required": ["summary"]
        }
    }
]

# ── Tool-Implementierungen ────────────────────────────────────────────────────

def tool_list_directory(path: str, recursive: bool = False) -> str:
    target = REPO_ROOT / path
    if not target.exists():
        return f"Fehler: Pfad '{path}' existiert nicht"
    if not target.is_dir():
        return f"Fehler: '{path}' ist kein Verzeichnis"

    entries = []
    if recursive:
        for item in sorted(target.rglob("*")):
            rel = item.relative_to(REPO_ROOT)
            prefix = "📁 " if item.is_dir() else "📄 "
            entries.append(f"{prefix}{rel}")
    else:
        for item in sorted(target.iterdir()):
            rel = item.relative_to(REPO_ROOT)
            prefix = "📁 " if item.is_dir() else "📄 "
            entries.append(f"{prefix}{rel}")

    return "\n".join(entries) if entries else "(leer)"


def tool_read_file(path: str) -> str:
    target = REPO_ROOT / path
    if not target.exists():
        return f"Fehler: Datei '{path}' nicht gefunden"
    if not target.is_file():
        return f"Fehler: '{path}' ist keine Datei"
    try:
        return target.read_text(encoding="utf-8", errors="replace")
    except Exception as e:
        return f"Fehler beim Lesen: {e}"


def tool_write_file(path: str, content: str) -> str:
    target = REPO_ROOT / path

    # Sicherheitscheck: nur innerhalb des Repo-Roots schreiben
    try:
        target.resolve().relative_to(REPO_ROOT.resolve())
    except ValueError:
        return "Fehler: Schreiben außerhalb des Repositories nicht erlaubt"

    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_text(content, encoding="utf-8")
    return f"Datei '{path}' erfolgreich geschrieben ({len(content)} Zeichen)"


def tool_bash_command(command: str) -> str:
    # Gefährliche Befehle blockieren
    blocked = ["rm -rf", "sudo", "curl", "wget", "nc ", "netcat", "> /dev", "dd if"]
    for b in blocked:
        if b in command:
            return f"Fehler: Befehl enthält blockierten Ausdruck '{b}'"

    try:
        result = subprocess.run(
            command,
            shell=True,
            cwd=str(REPO_ROOT),
            capture_output=True,
            text=True,
            timeout=30
        )
        output = result.stdout
        if result.stderr:
            output += f"\n[stderr]: {result.stderr}"
        return output.strip() or "(kein Output)"
    except subprocess.TimeoutExpired:
        return "Fehler: Befehl-Timeout (>30s)"
    except Exception as e:
        return f"Fehler: {e}"


def tool_search_files(pattern: str, path: str = ".", file_pattern: str = "*") -> str:
    search_path = REPO_ROOT / path
    if not search_path.exists():
        return f"Fehler: Pfad '{path}' existiert nicht"

    cmd = f'grep -r --include="{file_pattern}" -l "{pattern}" "{search_path}" 2>/dev/null'
    try:
        result = subprocess.run(cmd, shell=True, capture_output=True, text=True, timeout=10)
        lines = result.stdout.strip().split("\n") if result.stdout.strip() else []

        # Pfade relativ zum Repo-Root ausgeben
        rel_paths = []
        for line in lines:
            try:
                rel_paths.append(str(Path(line).relative_to(REPO_ROOT)))
            except ValueError:
                rel_paths.append(line)

        return "\n".join(rel_paths) if rel_paths else "Keine Treffer gefunden"
    except Exception as e:
        return f"Fehler: {e}"


def execute_tool(name: str, tool_input: dict) -> str:
    """Führt ein Tool aus und gibt das Ergebnis zurück."""
    if name == "list_directory":
        return tool_list_directory(
            tool_input["path"],
            tool_input.get("recursive", False)
        )
    elif name == "read_file":
        return tool_read_file(tool_input["path"])
    elif name == "write_file":
        return tool_write_file(tool_input["path"], tool_input["content"])
    elif name == "bash_command":
        return tool_bash_command(tool_input["command"])
    elif name == "search_files":
        return tool_search_files(
            tool_input["pattern"],
            tool_input.get("path", "."),
            tool_input.get("file_pattern", "*")
        )
    elif name == "finish":
        return f"FINISH: {tool_input['summary']}"
    else:
        return f"Fehler: Unbekanntes Tool '{name}'"

# ── Hauptagent ────────────────────────────────────────────────────────────────

def run_agent(mission: str) -> str:
    """
    Startet den autonomen Agenten mit einer gegebenen Mission.
    Gibt eine Zusammenfassung der durchgeführten Arbeit zurück.
    """
    client = anthropic.Anthropic()

    print(f"\n{'='*60}")
    print(f"🤖 Autonomer Flipper Zero Agent")
    print(f"{'='*60}")
    print(f"📋 Mission: {mission}")
    print(f"{'='*60}\n")

    messages = [
        {
            "role": "user",
            "content": f"Deine Mission: {mission}\n\nBeginne mit der Arbeit. Analysiere zuerst die Repo-Struktur und entscheide dann selbstständig, was verbessert werden soll. Nutze das 'finish'-Tool, wenn du fertig bist."
        }
    ]

    iteration = 0
    finish_summary = None

    while iteration < MAX_ITERATIONS:
        iteration += 1
        print(f"── Iteration {iteration}/{MAX_ITERATIONS} ──")

        response = client.messages.create(
            model=MODEL,
            max_tokens=MAX_TOKENS,
            thinking={"type": "adaptive"},
            system=[{
                "type": "text",
                "text": SYSTEM_PROMPT,
                "cache_control": {"type": "ephemeral"}
            }],
            tools=TOOLS,
            messages=messages
        )

        # Verarbeite den Response
        tool_results = []
        has_text = False

        for block in response.content:
            if block.type == "thinking":
                print(f"  💭 Denke nach... ({len(block.thinking)} Zeichen)")

            elif block.type == "text":
                if block.text.strip():
                    print(f"  🗣  {block.text[:300]}{'...' if len(block.text) > 300 else ''}")
                    has_text = True

            elif block.type == "tool_use":
                tool_name = block.name
                tool_input = block.input
                print(f"  🔧 Tool: {tool_name}({json.dumps(tool_input, ensure_ascii=False)[:100]})")

                result = execute_tool(tool_name, tool_input)

                if tool_name == "finish":
                    finish_summary = tool_input["summary"]
                    print(f"\n✅ Agent fertig: {finish_summary}")
                    return finish_summary

                print(f"     → {result[:150]}{'...' if len(result) > 150 else ''}")

                tool_results.append({
                    "type": "tool_result",
                    "tool_use_id": block.id,
                    "content": result
                })

        # Füge den Assistenten-Response zur History hinzu
        messages.append({"role": "assistant", "content": response.content})

        # Wenn es Tool-Aufrufe gab, die Ergebnisse zurückgeben
        if tool_results:
            messages.append({"role": "user", "content": tool_results})

        # Prüfe Stop-Reason
        if response.stop_reason == "end_turn" and not tool_results:
            print("\n✅ Agent hat seine Arbeit beendet (end_turn ohne weitere Tools)")
            break

        print()

    if iteration >= MAX_ITERATIONS:
        print(f"\n⚠️  Maximale Iterationen ({MAX_ITERATIONS}) erreicht")

    return finish_summary or "Arbeit abgeschlossen (Zusammenfassung fehlt)"


# ── Einstiegspunkt ────────────────────────────────────────────────────────────

DEFAULT_MISSIONS = [
    "Analysiere alle DuckyScript-Dateien in der Sammlung auf Qualität, Kommentare und Timing. "
    "Verbessere mindestens 3 Scripts durch bessere Dokumentation und robustere Delays. "
    "Erstelle eine COLLECTION_OVERVIEW.md mit einer strukturierten Übersicht aller Scripts. "
    "Committe alle Änderungen mit aussagekräftigen git-Messages.",

    "Durchsuche die gesamte Sammlung nach Scripts ohne ausreichende REM-Kommentare. "
    "Füge fehlende Metadaten hinzu (Author-Platzhalter, Version, Kategorie, Beschreibung). "
    "Verbessere Timing-Werte, die zu kurz erscheinen (<500ms für kritische Operationen). "
    "Committe die Verbesserungen.",

    "Erstelle eine strukturierte README-Verbesserung mit Script-Übersicht und Kategorien. "
    "Analysiere ob alle Kategorien vollständig dokumentiert sind. "
    "Füge fehlende readme.md Dateien hinzu wo sie fehlen. "
    "Committe alle neuen Dateien."
]


def main():
    api_key = os.environ.get("ANTHROPIC_API_KEY")
    if not api_key:
        print("Fehler: ANTHROPIC_API_KEY Umgebungsvariable nicht gesetzt")
        print("Setze sie mit: export ANTHROPIC_API_KEY='sk-ant-...'")
        sys.exit(1)

    # Mission aus Kommandozeile oder Standard-Mission
    if len(sys.argv) > 2 and sys.argv[1] == "--mission":
        mission = " ".join(sys.argv[2:])
    elif len(sys.argv) > 1 and sys.argv[1] != "--mission":
        mission = " ".join(sys.argv[1:])
    else:
        import random
        mission = random.choice(DEFAULT_MISSIONS)
        print(f"Keine Mission angegeben. Verwende zufällige Standard-Mission.")

    summary = run_agent(mission)
    print(f"\n{'='*60}")
    print(f"📊 Abschlussbericht: {summary}")
    print(f"{'='*60}\n")


if __name__ == "__main__":
    main()
