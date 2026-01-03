#!/usr/bin/env python3
"""
list_issues.py - Fetch all GitHub issues for duplicate analysis

Usage: python list_issues.py [--state open|closed|all]

Output: JSON with issue list including title, body, labels
"""

import argparse
import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent.parent))
from gh_utils import list_issues


def main():
    parser = argparse.ArgumentParser(description="Fetch GitHub issues for duplicate analysis")
    parser.add_argument("--state", default="open", choices=["open", "closed", "all"],
                        help="Issue state to fetch (default: open)")
    args = parser.parse_args()

    issues = list_issues(
        state=args.state,
        fields="number,title,body,labels,createdAt,comments",
        limit=500
    )

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
