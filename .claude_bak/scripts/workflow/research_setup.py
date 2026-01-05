#!/usr/bin/env python3
"""
research_setup.py - Start research on a GitHub issue

Usage: python research_setup.py <issue_number>

Actions:
1. Fetch issue details (title, body, comments, labels)
2. Transition labels: remove 'fresh', add 'under research'
3. Extract context hints (domain, keywords, mentioned files)

Output: JSON with issue context and transition status
"""

import json
import sys
from pathlib import Path
from typing import Any

sys.path.insert(0, str(Path(__file__).parent.parent))
from gh_utils import (
    get_issue_details, get_issue_comments_full, add_label, remove_label,
    extract_file_references, extract_keywords
)


def get_domain_hints(labels: list[str]) -> list[str]:
    """Extract domain hints from labels."""
    meta_labels = {"fresh", "under research", "researched", "fix-attempted"}
    priority_prefix = "priority:"

    domains = []
    for label in labels:
        label_lower = label.lower()
        if label_lower not in meta_labels and not label_lower.startswith(priority_prefix):
            domains.append(label)

    return domains


def main():
    if len(sys.argv) < 2:
        print(json.dumps({"error": "Usage: research_setup.py <issue_number>"}))
        sys.exit(1)

    try:
        issue_number = int(sys.argv[1])
    except ValueError:
        print(json.dumps({"error": "Issue number must be an integer"}))
        sys.exit(1)

    # Fetch issue details
    issue = get_issue_details(issue_number)
    if not issue:
        print(json.dumps({"error": f"Could not fetch issue #{issue_number}"}))
        sys.exit(1)

    # Check if issue has 'fresh' label
    labels = [label.get("name", "") for label in issue.get("labels", [])]
    if "fresh" not in [l.lower() for l in labels]:
        print(json.dumps({
            "error": f"Issue #{issue_number} does not have 'fresh' label",
            "current_labels": labels
        }))
        sys.exit(1)

    # Fetch comments
    comments = get_issue_comments_full(issue_number)

    # Transition labels
    fresh_removed = remove_label(issue_number, "fresh")
    research_added = add_label(issue_number, "under research")

    # Update labels list
    new_labels = [l for l in labels if l.lower() != "fresh"]
    new_labels.append("under research")

    # Extract context
    body = issue.get("body", "") or ""
    title = issue.get("title", "") or ""

    all_text = body + " " + " ".join(c.get("body", "") for c in comments)
    files_mentioned = extract_file_references(all_text)
    keywords = extract_keywords(title, body)
    domain_hints = get_domain_hints(labels)

    result = {
        "issue_number": issue_number,
        "title": title,
        "body": body,
        "labels": new_labels,
        "comments": comments,
        "transition": {
            "from": "fresh",
            "to": "under research",
            "fresh_removed": fresh_removed,
            "research_added": research_added,
            "success": fresh_removed and research_added,
        },
        "context": {
            "domain_hints": domain_hints,
            "files_mentioned": files_mentioned,
            "keywords": keywords,
        },
    }

    print(json.dumps(result, indent=2))


if __name__ == "__main__":
    main()
