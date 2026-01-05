#!/usr/bin/env python3
"""PreToolUse hook: Warn if editing src/ files on main branch."""
import json
import sys
import subprocess

def get_current_branch():
    """Get the current git branch name."""
    try:
        result = subprocess.run(
            ["git", "branch", "--show-current"],
            capture_output=True,
            text=True,
            timeout=5
        )
        return result.stdout.strip()
    except Exception:
        return None

def main():
    hook_input = json.load(sys.stdin)
    tool_name = hook_input.get("tool_name", "")
    tool_input = hook_input.get("tool_input", {})

    # Only check Edit and Write tools
    if tool_name not in ("Edit", "Write"):
        print(json.dumps({"decision": "allow"}))
        return

    file_path = tool_input.get("file_path", "")

    # Only warn for src/ files (actual code)
    if "/src/" not in file_path and not file_path.startswith("src/"):
        print(json.dumps({"decision": "allow"}))
        return

    branch = get_current_branch()

    if branch in ("main", "master"):
        print(json.dumps({
            "decision": "block",
            "reason": f"""Cannot edit source files on {branch} branch.

Create a feature branch first:
  python3 .claude/scripts/git/branch.py <branch-type>/<description>

Branch types: feat, fix, refactor, docs, test, chore

Example:
  python3 .claude/scripts/git/branch.py feat/add-inventory"""
        }))
    else:
        print(json.dumps({"decision": "allow"}))

if __name__ == "__main__":
    main()
