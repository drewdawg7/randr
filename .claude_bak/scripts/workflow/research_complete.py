#!/usr/bin/env python3
"""
research_complete.py - Complete research on a GitHub issue

Usage: python research_complete.py <issue_number> "<findings_markdown>"

Actions:
1. Post formatted research findings as a comment
2. Transition labels: remove 'under research', add 'researched'

Output: JSON with status of each action
"""

import json
import sys
from pathlib import Path
from typing import Any

sys.path.insert(0, str(Path(__file__).parent.parent))
from gh_utils import (
    get_issue_labels, add_label, remove_label, post_comment
)


def format_research_comment(findings: str) -> str:
    """Format the research findings into a structured comment."""
    return f"""## Research Findings

{findings}

---
*Automated via research_complete.py*"""


def main():
    if len(sys.argv) < 3:
        print(json.dumps({
            "error": "Usage: research_complete.py <issue_number> <findings_markdown>"
        }))
        sys.exit(1)

    try:
        issue_number = int(sys.argv[1])
    except ValueError:
        print(json.dumps({"error": "Issue number must be an integer"}))
        sys.exit(1)

    findings = sys.argv[2]

    # Check current labels
    labels = get_issue_labels(issue_number)
    labels_lower = [l.lower() for l in labels]

    if "under research" not in labels_lower:
        print(json.dumps({
            "warning": f"Issue #{issue_number} does not have 'under research' label",
            "current_labels": labels,
            "continuing": True
        }))

    result: dict[str, Any] = {
        "issue_number": issue_number,
        "comment_posted": False,
        "transition": {
            "from": "under research",
            "to": "researched",
            "success": False,
        },
        "ready_for_fix": False,
    }

    # Post research findings comment
    comment = format_research_comment(findings)
    result["comment_posted"] = post_comment(issue_number, comment)

    # Transition labels
    research_removed = remove_label(issue_number, "under research")
    researched_added = add_label(issue_number, "researched")

    result["transition"]["research_removed"] = research_removed
    result["transition"]["researched_added"] = researched_added
    result["transition"]["success"] = researched_added  # Main success criteria

    # Ready for fix if comment posted and label transitioned
    result["ready_for_fix"] = result["comment_posted"] and result["transition"]["success"]

    print(json.dumps(result, indent=2))


if __name__ == "__main__":
    main()
