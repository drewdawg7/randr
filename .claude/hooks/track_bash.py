#!/usr/bin/env python3
"""
PostToolUse hook: Track Bash tool invocations for session state.

Records all Bash operations for feedback metrics.
"""
import json
import sys
from pathlib import Path

# Import session state from same directory
sys.path.insert(0, str(Path(__file__).parent))
from session_state import get_state


def main():
    try:
        hook_input = json.load(sys.stdin)
    except json.JSONDecodeError:
        return

    tool_name = hook_input.get("tool_name", "")

    # Only track Bash operations
    if tool_name != "Bash":
        return

    state = get_state()
    state.record_bash_call()


if __name__ == "__main__":
    main()
