#!/usr/bin/env python3
"""
PreToolUse hook to enforce checking helper scripts before running CLI commands.

For certain command categories, this hook blocks execution and reminds Claude
to check .claude/scripts/SCRIPTS.index.md for available helper scripts first.
"""
import json
import sys
import re

# Command categories that likely have helper scripts
# Pattern -> category name for the reminder message
SCRIPT_CHECK_PATTERNS = [
    (r'^gh\s+issue\b', "GitHub issue operations"),
    (r'^gh\s+label\b', "GitHub label operations"),
    (r'^gh\s+api\b.*issues', "GitHub API issue operations"),
]

def main():
    try:
        input_data = json.load(sys.stdin)
    except json.JSONDecodeError:
        sys.exit(0)

    tool_name = input_data.get("tool_name", "")
    tool_input = input_data.get("tool_input", {})
    command = tool_input.get("command", "")

    # Only check Bash commands
    if tool_name != "Bash":
        sys.exit(0)

    for pattern, category in SCRIPT_CHECK_PATTERNS:
        if re.match(pattern, command):
            output = {
                "hookSpecificOutput": {
                    "hookEventName": "PreToolUse",
                    "permissionDecision": "deny",
                    "permissionDecisionReason": (
                        f"STOP: Before running {category} commands, you MUST check for helper scripts.\n\n"
                        f"1. Read .claude/scripts/SCRIPTS.index.md\n"
                        f"2. Find the appropriate script for your task\n"
                        f"3. Use the helper script instead of raw commands\n\n"
                        f"Helper scripts handle auth, error handling, and project-specific workflows."
                    )
                }
            }
            print(json.dumps(output))
            sys.exit(0)

    # Allow all other commands
    sys.exit(0)

if __name__ == "__main__":
    main()
