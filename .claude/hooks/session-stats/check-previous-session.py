#!/usr/bin/env python3
"""
Check if previous session needs stats aggregation.
Outputs a prompt for Claude if aggregation is needed.

This hook runs on SessionStart and checks if the previous session
has unaggregated stats. If so, it outputs a prompt for Claude to ask
the user if they want to generate a report.
"""
import json
from pathlib import Path

CLAUDE_DIR = Path.home() / ".claude"
STATS_DIR = Path(".claude/session_stats")


def get_previous_session_id():
    """
    Find the most recent session that isn't the current one.

    Returns the second-most-recent session ID by timestamp.
    """
    history_file = CLAUDE_DIR / "history.jsonl"
    if not history_file.exists():
        return None

    sessions = {}
    with open(history_file) as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            try:
                entry = json.loads(line)
                sid = entry.get("sessionId")
                ts = entry.get("timestamp", 0)
                if sid:
                    if sid not in sessions or ts > sessions[sid]:
                        sessions[sid] = ts
            except json.JSONDecodeError:
                continue

    # Sort by timestamp descending
    sorted_sessions = sorted(sessions.items(), key=lambda x: x[1], reverse=True)

    # Return the second session (previous, not current)
    if len(sorted_sessions) < 2:
        return None

    return sorted_sessions[1][0]


def stats_exist(session_id: str) -> bool:
    """Check if stats already generated for this session."""
    STATS_DIR.mkdir(parents=True, exist_ok=True)
    return (STATS_DIR / f"{session_id}.json").exists()


def main():
    """Main entry point."""
    session_id = get_previous_session_id()

    if not session_id:
        # No previous session, no output needed
        return

    if stats_exist(session_id):
        # Already aggregated, no output needed
        return

    # Previous session needs aggregation - prompt user
    short_id = session_id[:8]
    prompt = (
        f"The previous session (ID: {short_id}...) does not have aggregated stats. "
        "Ask the user if they would like to generate a session stats report "
        "using the session-stats skill."
    )
    print(prompt)


if __name__ == "__main__":
    main()
