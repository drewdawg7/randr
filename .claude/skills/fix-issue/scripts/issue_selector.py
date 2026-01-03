#!/usr/bin/env python3
"""
issue_selector.py - Issue prioritization for GitHub issues

Usage: python issue_selector.py

Lists all issues with 'researched' label (excluding 'fix-attempted'),
sorted by priority.

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
import sys
from datetime import datetime, timezone
from typing import Any


def run_cmd(cmd: list[str]) -> tuple[bool, str]:
    """Run a command and return (success, output)."""
    try:
        result = subprocess.run(cmd, capture_output=True, text=True, check=True)
        return True, result.stdout.strip()
    except subprocess.CalledProcessError as e:
        return False, e.stderr.strip()


def get_researched_issues() -> list[dict[str, Any]]:
    """Fetch all issues with 'researched' label, excluding 'fix-attempted'."""
    success, output = run_cmd([
        "gh", "issue", "list",
        "--label", "researched",
        "--state", "open",
        "--json", "number,title,labels,createdAt",
        "--limit", "100"
    ])

    if not success or not output:
        return []

    issues = json.loads(output)

    # Filter out issues with 'fix-attempted' label
    filtered = []
    for issue in issues:
        labels = [label.get("name", "").lower() for label in issue.get("labels", [])]
        if "fix-attempted" not in labels:
            filtered.append(issue)

    return filtered


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
    issues = get_researched_issues()

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
