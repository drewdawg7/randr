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
import sys
from pathlib import Path
from typing import Any

sys.path.insert(0, str(Path(__file__).parent.parent))
from gh_utils import (
    get_issue_details, get_issue_comments, extract_file_references,
    checkout_main_and_pull, create_branch, slugify
)


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
