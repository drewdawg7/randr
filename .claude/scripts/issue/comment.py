#!/usr/bin/env python3
"""Add a comment to a GitHub issue."""
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

def add_comment(issue_number, body):
    """Add a comment to an issue."""
    cmd = ["issue", "comment", str(issue_number), "--body", body]
    result = run_gh(*cmd)

    if result.returncode != 0:
        return {"success": False, "error": result.stderr.strip()}

    return {
        "success": True,
        "issue": issue_number,
        "message": "Comment added successfully"
    }

def add_label(issue_number, label):
    """Add a label to an issue."""
    cmd = ["issue", "edit", str(issue_number), "--add-label", label]
    result = run_gh(*cmd)

    if result.returncode != 0:
        return {"success": False, "error": result.stderr.strip()}

    return {
        "success": True,
        "issue": issue_number,
        "label": label,
        "message": "Label added successfully"
    }

def main():
    if len(sys.argv) < 3:
        print(json.dumps({
            "success": False,
            "error": "Usage: comment.py <issue_number> <body> [--add-label <label>]"
        }))
        sys.exit(1)

    issue_number = sys.argv[1]
    body = sys.argv[2]
    label = None

    i = 3
    while i < len(sys.argv):
        arg = sys.argv[i]
        if arg == "--add-label" and i + 1 < len(sys.argv):
            label = sys.argv[i + 1]
            i += 2
        else:
            i += 1

    # Add the comment
    result = add_comment(issue_number, body)
    if not result["success"]:
        print(json.dumps(result, indent=2))
        sys.exit(1)

    # Add label if specified
    if label:
        label_result = add_label(issue_number, label)
        result["label_added"] = label_result["success"]
        if not label_result["success"]:
            result["label_error"] = label_result.get("error")

    print(json.dumps(result, indent=2))

if __name__ == "__main__":
    main()
