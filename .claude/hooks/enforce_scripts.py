#!/usr/bin/env python3
"""PreToolUse hook: Block raw gh commands, suggest scripts instead."""
import json
import sys
import re

SCRIPT_MAP = {
    "gh issue list": ".claude/scripts/issue/list.py",
    "gh issue view": ".claude/scripts/issue/view.py",
    "gh issue create": ".claude/scripts/issue/create.py",
    "gh issue close": ".claude/scripts/issue/close.py",
    "gh issue edit": ".claude/scripts/issue/edit.py",
    "gh issue comment": ".claude/scripts/issue/comment.py",
    "gh label": ".claude/scripts/issue/labels.py",
    "gh api": ".claude/scripts/issue/api.py",
}

def main():
    hook_input = json.load(sys.stdin)
    tool_name = hook_input.get("tool_name", "")
    tool_input = hook_input.get("tool_input", {})

    if tool_name != "Bash":
        print(json.dumps({"decision": "allow"}))
        return

    command = tool_input.get("command", "")

    # Check if it's a gh command
    if not command.strip().startswith("gh "):
        print(json.dumps({"decision": "allow"}))
        return

    # Find the matching script
    suggested_script = None
    for gh_cmd, script in SCRIPT_MAP.items():
        if command.strip().startswith(gh_cmd):
            suggested_script = script
            break

    if suggested_script:
        print(json.dumps({
            "decision": "block",
            "reason": f"""Use helper scripts instead of raw gh commands.

Blocked command: {command}

Use instead: python3 {suggested_script}

All scripts output JSON for consistent parsing.
See .claude/CLAUDE.md for full script reference."""
        }))
    else:
        # Unknown gh command - allow but warn
        print(json.dumps({"decision": "allow"}))

if __name__ == "__main__":
    main()
