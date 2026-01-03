#!/usr/bin/env python3
"""
issue_context.py - Full context extractor for GitHub issues

Usage: python issue_context.py <issue_number>

Extracts:
- Issue title, body, labels
- All comments with authors
- File references mentioned in issue/comments
- Creation date

Output: JSON with full issue context
"""

import json
import re
import subprocess
import sys
from typing import Any


def run_cmd(cmd: list[str]) -> tuple[bool, str]:
    """Run a command and return (success, output)."""
    try:
        result = subprocess.run(cmd, capture_output=True, text=True, check=True)
        return True, result.stdout.strip()
    except subprocess.CalledProcessError as e:
        return False, e.stderr.strip()


def extract_file_references(text: str) -> list[str]:
    """Extract file paths mentioned in text."""
    patterns = [
        r'`(src/[^`]+\.rs)`',           # src/path/file.rs in backticks
        r'\b(src/[\w/]+\.rs)\b',        # src/path/file.rs without backticks
        r'`([^`]+\.(rs|py|md|toml))`',  # Any code file in backticks
    ]

    files = set()
    for pattern in patterns:
        matches = re.findall(pattern, text)
        for match in matches:
            if isinstance(match, tuple):
                files.add(match[0])
            else:
                files.add(match)

    return sorted(files)


def get_issue_details(issue_number: int) -> dict[str, Any] | None:
    """Fetch issue details from GitHub."""
    fields = "number,title,body,labels,createdAt,state"
    success, output = run_cmd([
        "gh", "issue", "view", str(issue_number),
        "--json", fields
    ])

    if success:
        return json.loads(output)
    return None


def get_issue_comments(issue_number: int) -> list[dict[str, str]]:
    """Fetch all comments on an issue."""
    success, output = run_cmd([
        "gh", "api",
        f"repos/:owner/:repo/issues/{issue_number}/comments",
        "--jq", '.[] | {author: .user.login, body: .body, created_at: .created_at}'
    ])

    if not success or not output:
        return []

    comments = []
    # Each line is a JSON object
    for line in output.strip().split('\n'):
        if line:
            try:
                comments.append(json.loads(line))
            except json.JSONDecodeError:
                continue

    return comments


def main():
    if len(sys.argv) < 2:
        print(json.dumps({"error": "Usage: issue_context.py <issue_number>"}))
        sys.exit(1)

    try:
        issue_number = int(sys.argv[1])
    except ValueError:
        print(json.dumps({"error": "Issue number must be an integer"}))
        sys.exit(1)

    # Get issue details
    issue = get_issue_details(issue_number)
    if not issue:
        print(json.dumps({"error": f"Could not fetch issue #{issue_number}"}))
        sys.exit(1)

    # Get comments
    comments = get_issue_comments(issue_number)

    # Extract file references from body and comments
    all_text = issue.get("body", "") or ""
    for comment in comments:
        all_text += "\n" + (comment.get("body", "") or "")

    files_mentioned = extract_file_references(all_text)

    # Build result
    result = {
        "number": issue.get("number"),
        "title": issue.get("title"),
        "body": issue.get("body"),
        "state": issue.get("state"),
        "labels": [label.get("name") for label in issue.get("labels", [])],
        "created_at": issue.get("createdAt"),
        "comments": comments,
        "files_mentioned": files_mentioned,
    }

    print(json.dumps(result, indent=2))


if __name__ == "__main__":
    main()
