#!/usr/bin/env python3
"""
PreToolUse hook to enforce use of helper scripts instead of raw gh commands.

Blocks raw `gh issue` and `gh api` commands and directs Claude to use
the helper scripts documented in .claude/scripts/SCRIPTS.index.md
"""
import json
import sys
import re

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

    # Patterns for raw gh commands that should use scripts instead
    blocked_patterns = [
        (r'^gh\s+issue\s+view\b', "Use `python3 .claude/scripts/issue/issue_context.py <issue_number>` instead"),
        (r'^gh\s+issue\s+list\b', "Use `python3 .claude/scripts/issue/list_issues.py` or `issue_selector.py` instead"),
        (r'^gh\s+issue\s+create\b', "Use `python3 .claude/scripts/workflow/create_issue.py` instead"),
        (r'^gh\s+issue\s+close\b', "Use `python3 .claude/scripts/issue/close_duplicate.py` or handle via workflow scripts"),
        (r'^gh\s+issue\s+edit\b', "Use the appropriate workflow script from .claude/scripts/"),
        (r'^gh\s+issue\s+comment\b', "Use workflow scripts that handle comments (fix_complete.py, research_complete.py)"),
    ]

    for pattern, suggestion in blocked_patterns:
        if re.match(pattern, command):
            output = {
                "hookSpecificOutput": {
                    "hookEventName": "PreToolUse",
                    "permissionDecision": "deny",
                    "permissionDecisionReason": f"Raw gh commands are not allowed. {suggestion}\n\nSee .claude/scripts/SCRIPTS.index.md for all available scripts."
                }
            }
            print(json.dumps(output))
            sys.exit(0)

    # Allow all other commands
    sys.exit(0)

if __name__ == "__main__":
    main()
