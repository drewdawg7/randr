#!/usr/bin/env python3
"""
fix_setup.py - Branch & workspace setup for fixing GitHub issues

Usage: python fix_setup.py <issue_number>

Actions:
1. Fetch issue title
2. Checkout main, pull latest
3. Create branch: fix/issue-{number}-{slug}
4. Output issue context

Output: JSON with branch info and full issue context
"""

import json
import re
import subprocess
import sys
from typing import Any


def run_cmd(cmd: list[str], check: bool = True) -> tuple[bool, str]:
    """Run a command and return (success, output)."""
    try:
        result = subprocess.run(cmd, capture_output=True, text=True, check=check)
        return True, result.stdout.strip()
    except subprocess.CalledProcessError as e:
        return False, e.stderr.strip()


def slugify(text: str, max_length: int = 40) -> str:
    """Convert text to URL-friendly slug."""
    # Lowercase
    slug = text.lower()
    # Replace non-alphanumeric with hyphens
    slug = re.sub(r'[^a-z0-9]+', '-', slug)
    # Remove leading/trailing hyphens
    slug = slug.strip('-')
    # Truncate
    if len(slug) > max_length:
        slug = slug[:max_length].rstrip('-')
    return slug


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
    for line in output.strip().split('\n'):
        if line:
            try:
                comments.append(json.loads(line))
            except json.JSONDecodeError:
                continue

    return comments


def extract_file_references(text: str) -> list[str]:
    """Extract file paths mentioned in text."""
    patterns = [
        r'`(src/[^`]+\.rs)`',
        r'\b(src/[\w/]+\.rs)\b',
        r'`([^`]+\.(rs|py|md|toml))`',
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


def checkout_main_and_pull() -> bool:
    """Checkout main and pull latest."""
    success, _ = run_cmd(["git", "checkout", "main"])
    if not success:
        return False

    success, _ = run_cmd(["git", "pull", "origin", "main"])
    return success


def create_branch(branch_name: str) -> bool:
    """Create and checkout a new branch."""
    success, _ = run_cmd(["git", "checkout", "-b", branch_name])
    return success


def main():
    if len(sys.argv) < 2:
        print(json.dumps({"error": "Usage: fix_setup.py <issue_number>"}))
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

    title = issue.get("title", "")
    slug = slugify(title)
    branch_name = f"fix/issue-{issue_number}-{slug}"

    result: dict[str, Any] = {
        "issue": issue_number,
        "branch": branch_name,
        "created": False,
        "context": None,
    }

    # Checkout main and pull
    if not checkout_main_and_pull():
        result["error"] = "Failed to checkout/pull main"
        print(json.dumps(result, indent=2))
        sys.exit(1)

    # Create branch
    if not create_branch(branch_name):
        result["error"] = f"Failed to create branch {branch_name}"
        print(json.dumps(result, indent=2))
        sys.exit(1)

    result["created"] = True

    # Build context
    comments = get_issue_comments(issue_number)

    all_text = issue.get("body", "") or ""
    for comment in comments:
        all_text += "\n" + (comment.get("body", "") or "")

    files_mentioned = extract_file_references(all_text)

    result["context"] = {
        "number": issue.get("number"),
        "title": title,
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
