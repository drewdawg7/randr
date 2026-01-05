#!/usr/bin/env python3
"""Edit a GitHub issue."""
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

def edit_issue(number, title=None, body=None, add_labels=None, remove_labels=None, assignee=None):
    """Edit an existing issue."""
    cmd = ["issue", "edit", str(number)]

    if title:
        cmd.extend(["--title", title])
    if body:
        cmd.extend(["--body", body])
    if add_labels:
        for label in add_labels:
            cmd.extend(["--add-label", label])
    if remove_labels:
        for label in remove_labels:
            cmd.extend(["--remove-label", label])
    if assignee:
        cmd.extend(["--add-assignee", assignee])

    result = run_gh(*cmd)

    if result.returncode != 0:
        return {"success": False, "error": result.stderr.strip()}

    return {
        "success": True,
        "number": number,
        "updated": {
            "title": title is not None,
            "body": body is not None,
            "labels_added": add_labels if add_labels else [],
            "labels_removed": remove_labels if remove_labels else [],
            "assignee": assignee is not None
        }
    }

def main():
    if len(sys.argv) < 2:
        print(json.dumps({
            "success": False,
            "error": "Usage: edit.py <issue_number> [--title <title>] [--body <body>] [--add-label <label>] [--remove-label <label>] [--assignee <user>]"
        }))
        sys.exit(1)

    try:
        number = int(sys.argv[1])
    except ValueError:
        print(json.dumps({"success": False, "error": "Issue number must be an integer"}))
        sys.exit(1)

    title = None
    body = None
    add_labels = []
    remove_labels = []
    assignee = None

    i = 2
    while i < len(sys.argv):
        arg = sys.argv[i]
        if arg == "--title" and i + 1 < len(sys.argv):
            title = sys.argv[i + 1]
            i += 2
        elif arg == "--body" and i + 1 < len(sys.argv):
            body = sys.argv[i + 1]
            i += 2
        elif arg == "--add-label" and i + 1 < len(sys.argv):
            add_labels.append(sys.argv[i + 1])
            i += 2
        elif arg == "--remove-label" and i + 1 < len(sys.argv):
            remove_labels.append(sys.argv[i + 1])
            i += 2
        elif arg == "--assignee" and i + 1 < len(sys.argv):
            assignee = sys.argv[i + 1]
            i += 2
        else:
            i += 1

    # Check if any edits were requested
    if not any([title, body, add_labels, remove_labels, assignee]):
        print(json.dumps({
            "success": False,
            "error": "No edits specified. Use --title, --body, --add-label, --remove-label, or --assignee"
        }))
        sys.exit(1)

    result = edit_issue(
        number,
        title=title,
        body=body,
        add_labels=add_labels if add_labels else None,
        remove_labels=remove_labels if remove_labels else None,
        assignee=assignee
    )
    print(json.dumps(result, indent=2))

if __name__ == "__main__":
    main()
