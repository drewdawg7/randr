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
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent.parent))
from gh_utils import (
    get_issue_details, get_issue_comments,
    extract_file_references
)


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
