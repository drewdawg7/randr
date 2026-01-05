#!/usr/bin/env python3
"""View a single GitHub issue with full details."""
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

def view_issue(number):
    """Get full details of an issue."""
    result = run_gh(
        "issue", "view", str(number),
        "--json", "number,title,body,state,labels,assignees,comments,createdAt,updatedAt,author"
    )

    if result.returncode != 0:
        return {"success": False, "error": result.stderr.strip()}

    try:
        issue = json.loads(result.stdout)
        return {
            "success": True,
            "number": issue["number"],
            "title": issue["title"],
            "body": issue.get("body", ""),
            "state": issue["state"],
            "author": issue.get("author", {}).get("login", ""),
            "labels": [l["name"] for l in issue.get("labels", [])],
            "assignees": [a["login"] for a in issue.get("assignees", [])],
            "created": issue.get("createdAt", ""),
            "updated": issue.get("updatedAt", ""),
            "comments": [
                {
                    "author": c.get("author", {}).get("login", ""),
                    "body": c.get("body", ""),
                    "created": c.get("createdAt", "")
                }
                for c in issue.get("comments", [])
            ]
        }
    except json.JSONDecodeError as e:
        return {"success": False, "error": f"Failed to parse response: {e}"}

def main():
    if len(sys.argv) < 2:
        print(json.dumps({"success": False, "error": "Usage: view.py <issue_number>"}))
        sys.exit(1)

    try:
        number = int(sys.argv[1])
    except ValueError:
        print(json.dumps({"success": False, "error": "Issue number must be an integer"}))
        sys.exit(1)

    result = view_issue(number)
    print(json.dumps(result, indent=2))

if __name__ == "__main__":
    main()
