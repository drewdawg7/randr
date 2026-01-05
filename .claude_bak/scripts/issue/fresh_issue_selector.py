#!/usr/bin/env python3
"""
fresh_issue_selector.py - List fresh issues for research

Usage: python fresh_issue_selector.py

Lists all issues with 'fresh' label, sorted by priority then age.

Priority ranking:
1. Labels containing "critical"
2. Labels containing "high"
3. Labels containing "medium"
4. Labels containing "low"
5. By age (oldest first) if no priority label

Output: JSON with prioritized issue list
"""

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent.parent))
from gh_utils import list_issues, get_priority, calculate_age_days


def main():
    issues = list_issues(
        label="fresh",
        state="open",
        fields="number,title,labels,createdAt,body",
        limit=100
    )

    if not issues:
        print(json.dumps({"issues": [], "count": 0}))
        return

    # Enrich issues with priority and age
    enriched = []
    for issue in issues:
        labels = [label.get("name", "") for label in issue.get("labels", [])]
        priority_rank, priority_name = get_priority(labels)
        age_days = calculate_age_days(issue.get("createdAt", ""))

        enriched.append({
            "number": issue.get("number"),
            "title": issue.get("title"),
            "labels": labels,
            "priority": priority_name,
            "priority_rank": priority_rank,
            "age_days": age_days,
            "created_at": issue.get("createdAt"),
        })

    # Sort by priority rank, then by age (oldest first)
    enriched.sort(key=lambda x: (x["priority_rank"], -x["age_days"]))

    # Remove internal priority_rank from output
    for item in enriched:
        del item["priority_rank"]

    result = {
        "issues": enriched,
        "count": len(enriched),
    }

    print(json.dumps(result, indent=2))


if __name__ == "__main__":
    main()
