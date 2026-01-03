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
import subprocess
from datetime import datetime, timezone
from typing import Any


def run_cmd(cmd: list[str]) -> tuple[bool, str]:
    """Run a command and return (success, output)."""
    try:
        result = subprocess.run(cmd, capture_output=True, text=True, check=True)
        return True, result.stdout.strip()
    except subprocess.CalledProcessError as e:
        return False, e.stderr.strip()


def get_fresh_issues() -> list[dict[str, Any]]:
    """Fetch all issues with 'fresh' label."""
    success, output = run_cmd([
        "gh", "issue", "list",
        "--label", "fresh",
        "--state", "open",
        "--json", "number,title,labels,createdAt,body",
        "--limit", "100"
    ])

    if not success or not output:
        return []

    return json.loads(output)


def get_priority(labels: list[str]) -> tuple[int, str]:
    """
    Determine priority from labels.
    Returns (priority_rank, priority_name).
    Lower rank = higher priority.
    """
    labels_lower = [label.lower() for label in labels]

    for label in labels_lower:
        if "critical" in label:
            return (0, "critical")
        if "high" in label:
            return (1, "high")
        if "medium" in label:
            return (2, "medium")
        if "low" in label:
            return (3, "low")

    return (4, "none")


def calculate_age_days(created_at: str) -> int:
    """Calculate age in days from ISO timestamp."""
    try:
        created = datetime.fromisoformat(created_at.replace("Z", "+00:00"))
        now = datetime.now(timezone.utc)
        return (now - created).days
    except (ValueError, TypeError):
        return 0


def main():
    issues = get_fresh_issues()

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
