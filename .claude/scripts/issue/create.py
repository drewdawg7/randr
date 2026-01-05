#!/usr/bin/env python3
"""Create a new GitHub issue."""
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

def create_issue(title, body="", labels=None, assignee=None):
    """Create a new issue."""
    cmd = ["issue", "create", "--title", title]

    if body:
        cmd.extend(["--body", body])
    if labels:
        for label in labels:
            cmd.extend(["--label", label])
    if assignee:
        cmd.extend(["--assignee", assignee])

    result = run_gh(*cmd)

    if result.returncode != 0:
        return {"success": False, "error": result.stderr.strip()}

    # Parse the issue URL from output
    url = result.stdout.strip()
    number = None
    if url:
        try:
            number = int(url.split("/")[-1])
        except (ValueError, IndexError):
            pass

    return {
        "success": True,
        "number": number,
        "url": url,
        "title": title
    }

def main():
    if len(sys.argv) < 2:
        print(json.dumps({
            "success": False,
            "error": "Usage: create.py <title> [--title <title>] [--body <body>] [--label <label>] [--assignee <user>]"
        }))
        sys.exit(1)

    # Check if first arg is a flag or a positional title
    title = None
    if not sys.argv[1].startswith("--"):
        title = sys.argv[1]
        start_idx = 2
    else:
        start_idx = 1

    body = ""
    labels = []
    assignee = None

    i = start_idx
    while i < len(sys.argv):
        arg = sys.argv[i]
        if arg == "--title" and i + 1 < len(sys.argv):
            title = sys.argv[i + 1]
            i += 2
        elif arg == "--body" and i + 1 < len(sys.argv):
            body = sys.argv[i + 1]
            i += 2
        elif arg == "--label" and i + 1 < len(sys.argv):
            labels.append(sys.argv[i + 1])
            i += 2
        elif arg == "--assignee" and i + 1 < len(sys.argv):
            assignee = sys.argv[i + 1]
            i += 2
        else:
            i += 1

    if not title:
        print(json.dumps({
            "success": False,
            "error": "Title is required. Use: create.py <title> or create.py --title <title>"
        }))
        sys.exit(1)

    result = create_issue(title, body, labels if labels else None, assignee)
    print(json.dumps(result, indent=2))

if __name__ == "__main__":
    main()
