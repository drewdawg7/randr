#!/usr/bin/env python3
"""Close a GitHub issue."""
import json
import subprocess
import sys

def run_gh(*args):
    """Run a gh command and return result."""
    result = subprocess.run(
        ["gh"] + list(args),
        capture_output=True,
        text=True
    )
    return result

def close_issue(number, comment=None, reason="completed"):
    """Close an issue with optional comment."""
    # Add comment if provided
    if comment:
        comment_result = run_gh("issue", "comment", str(number), "--body", comment)
        if comment_result.returncode != 0:
            return {"success": False, "error": f"Failed to add comment: {comment_result.stderr}"}

    # Close the issue
    cmd = ["issue", "close", str(number)]
    if reason == "not_planned":
        cmd.append("--reason")
        cmd.append("not planned")

    result = run_gh(*cmd)

    if result.returncode != 0:
        return {"success": False, "error": result.stderr.strip()}

    return {
        "success": True,
        "number": number,
        "reason": reason,
        "commented": comment is not None
    }

def main():
    if len(sys.argv) < 2:
        print(json.dumps({
            "success": False,
            "error": "Usage: close.py <issue_number> [--comment <text>] [--reason not_planned]"
        }))
        sys.exit(1)

    try:
        number = int(sys.argv[1])
    except ValueError:
        print(json.dumps({"success": False, "error": "Issue number must be an integer"}))
        sys.exit(1)

    comment = None
    reason = "completed"

    i = 2
    while i < len(sys.argv):
        arg = sys.argv[i]
        if arg == "--comment" and i + 1 < len(sys.argv):
            comment = sys.argv[i + 1]
            i += 2
        elif arg == "--reason" and i + 1 < len(sys.argv):
            reason = sys.argv[i + 1]
            i += 2
        else:
            i += 1

    result = close_issue(number, comment, reason)
    print(json.dumps(result, indent=2))

if __name__ == "__main__":
    main()
