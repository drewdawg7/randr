#!/usr/bin/env python3
"""
list_issues.py - Fetch all GitHub issues for duplicate analysis

Usage: python list_issues.py [--state open|closed|all]

Output: JSON with issue list including title, body, labels
"""

import argparse
import json
import subprocess
from typing import Any


def run_cmd(cmd: list[str]) -> tuple[bool, str]:
    """Run a command and return (success, output)."""
    try:
        result = subprocess.run(cmd, capture_output=True, text=True, check=True)
        return True, result.stdout.strip()
    except subprocess.CalledProcessError as e:
        return False, e.stderr.strip()


def get_issues(state: str = "open") -> list[dict[str, Any]]:
    """Fetch issues with specified state."""
    success, output = run_cmd([
        "gh", "issue", "list",
        "--state", state,
        "--json", "number,title,body,labels,createdAt,comments",
        "--limit", "500"
    ])

    if not success or not output:
        return []

    return json.loads(output)


def main():
    parser = argparse.ArgumentParser(description="Fetch GitHub issues for duplicate analysis")
    parser.add_argument("--state", default="open", choices=["open", "closed", "all"],
                        help="Issue state to fetch (default: open)")
    args = parser.parse_args()

    issues = get_issues(args.state)

    # Simplify output
    simplified = []
    for issue in issues:
        labels = [l.get("name", "") for l in issue.get("labels", [])]
        simplified.append({
            "number": issue.get("number"),
            "title": issue.get("title"),
            "body": (issue.get("body") or "")[:500],  # Truncate for readability
            "labels": labels,
            "created_at": issue.get("createdAt"),
            "comment_count": len(issue.get("comments", []))
        })

    print(json.dumps({"issues": simplified, "count": len(simplified)}, indent=2))


if __name__ == "__main__":
    main()
