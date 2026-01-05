#!/usr/bin/env python3
"""Get token usage for current or specified session."""

import json
import os
import sys
from pathlib import Path

def get_current_session_id():
    """Get the current session ID from history."""
    history_file = Path.home() / ".claude" / "history.jsonl"
    if not history_file.exists():
        return None

    # Read last entry to get current session
    with open(history_file, 'r') as f:
        lines = f.readlines()
        if lines:
            last_entry = json.loads(lines[-1])
            return last_entry.get('sessionId')
    return None

def get_project_path():
    """Get the Claude project path for current directory."""
    cwd = os.getcwd()
    # Convert path to Claude's format: /Users/foo/bar -> -Users-foo-bar
    project_name = cwd.replace('/', '-')  # Replace / with -
    return Path.home() / ".claude" / "projects" / project_name

def get_session_tokens(session_id=None, project_path=None):
    """Calculate token usage for a session."""
    if session_id is None:
        session_id = get_current_session_id()

    if project_path is None:
        project_path = get_project_path()

    session_file = project_path / f"{session_id}.jsonl"

    if not session_file.exists():
        return {"error": f"Session file not found: {session_file}"}

    totals = {
        "input_tokens": 0,
        "output_tokens": 0,
        "cache_read_input_tokens": 0,
        "cache_creation_input_tokens": 0
    }

    message_count = 0

    with open(session_file, 'r') as f:
        for line in f:
            try:
                entry = json.loads(line)
                if 'message' in entry and 'usage' in entry['message']:
                    usage = entry['message']['usage']
                    totals['input_tokens'] += usage.get('input_tokens', 0)
                    totals['output_tokens'] += usage.get('output_tokens', 0)
                    totals['cache_read_input_tokens'] += usage.get('cache_read_input_tokens', 0)
                    totals['cache_creation_input_tokens'] += usage.get('cache_creation_input_tokens', 0)
                    message_count += 1
            except json.JSONDecodeError:
                continue

    # Message tokens = input + output (matches /context "Messages" display)
    message_tokens = totals['input_tokens'] + totals['output_tokens']

    return {
        "success": True,
        "session_id": session_id,
        "message_tokens": message_tokens,  # This matches /context "Messages"
        "input_tokens": totals['input_tokens'],
        "output_tokens": totals['output_tokens'],
        "cache_read_tokens": totals['cache_read_input_tokens'],
        "cache_creation_tokens": totals['cache_creation_input_tokens'],
        "message_count": message_count
    }

if __name__ == "__main__":
    session_id = sys.argv[1] if len(sys.argv) > 1 else None
    result = get_session_tokens(session_id)
    print(json.dumps(result, indent=2))
