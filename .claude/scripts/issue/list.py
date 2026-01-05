#!/usr/bin/env python3
"""List GitHub issues with filtering."""
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

def list_issues(state="open", labels=None, limit=30, assignee=None):
    """List issues with optional filters."""
    cmd = ["issue", "list", "--json", "number,title,state,labels,assignees,createdAt,updatedAt"]
    cmd.extend(["--state", state])
    cmd.extend(["--limit", str(limit)])

    if labels:
        cmd.extend(["--label", labels])
    if assignee:
        cmd.extend(["--assignee", assignee])

    result = run_gh(*cmd)
    if result.returncode != 0:
        return {"success": False, "error": result.stderr.strip()}

    try:
        issues = json.loads(result.stdout)
        return {
            "success": True,
            "count": len(issues),
            "issues": [
                {
                    "number": i["number"],
                    "title": i["title"],
                    "state": i["state"],
                    "labels": [l["name"] for l in i.get("labels", [])],
                    "assignees": [a["login"] for a in i.get("assignees", [])],
                    "created": i.get("createdAt", ""),
                    "updated": i.get("updatedAt", "")
                }
                for i in issues
            ]
        }
    except json.JSONDecodeError as e:
        return {"success": False, "error": f"Failed to parse response: {e}"}

def main():
    state = "open"
    labels = None
    limit = 30
    assignee = None

    i = 1
    while i < len(sys.argv):
        arg = sys.argv[i]
        if arg == "--state" and i + 1 < len(sys.argv):
            state = sys.argv[i + 1]
            i += 2
        elif arg == "--label" and i + 1 < len(sys.argv):
            labels = sys.argv[i + 1]
            i += 2
        elif arg == "--limit" and i + 1 < len(sys.argv):
            limit = int(sys.argv[i + 1])
            i += 2
        elif arg == "--assignee" and i + 1 < len(sys.argv):
            assignee = sys.argv[i + 1]
            i += 2
        else:
            i += 1

    result = list_issues(state, labels, limit, assignee)
    print(json.dumps(result, indent=2))

if __name__ == "__main__":
    main()
