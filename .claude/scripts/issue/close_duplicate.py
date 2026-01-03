#!/usr/bin/env python3
"""
close_duplicate.py - Mark and close an issue as a duplicate

Usage: python close_duplicate.py <duplicate_number> <original_number>

Actions:
1. Verify both issues exist
2. Add 'duplicate' label to the duplicate issue
3. Post comment linking to original issue
4. Close duplicate with "not planned" reason

Output: JSON with status of each action
"""

import json
import sys
from pathlib import Path
from typing import Any

sys.path.insert(0, str(Path(__file__).parent.parent))
from gh_utils import (
    issue_exists, add_label, post_comment, close_issue,
    ensure_label_exists
)


def post_duplicate_comment(duplicate_number: int, original_number: int,
                           original_title: str) -> bool:
    """Post a comment explaining the duplicate."""
    comment = f"""## Duplicate Issue

This issue has been identified as a duplicate of #{original_number}.

**Original issue:** #{original_number} - {original_title}

Please follow the original issue for updates. Any additional context from this issue has been considered.

---
*Closed via clean-duplicates skill*"""

    return post_comment(duplicate_number, comment)


def main():
    if len(sys.argv) < 3:
        print(json.dumps({
            "error": "Usage: close_duplicate.py <duplicate_number> <original_number>"
        }))
        sys.exit(1)

    try:
        duplicate_number = int(sys.argv[1])
        original_number = int(sys.argv[2])
    except ValueError:
        print(json.dumps({"error": "Issue numbers must be integers"}))
        sys.exit(1)

    if duplicate_number == original_number:
        print(json.dumps({"error": "Duplicate and original cannot be the same issue"}))
        sys.exit(1)

    result: dict[str, Any] = {
        "duplicate_number": duplicate_number,
        "original_number": original_number,
        "duplicate_exists": False,
        "original_exists": False,
        "label_added": False,
        "comment_posted": False,
        "closed": False,
    }

    # Verify issues exist
    dup_exists, dup_title = issue_exists(duplicate_number)
    orig_exists, orig_title = issue_exists(original_number)

    result["duplicate_exists"] = dup_exists
    result["original_exists"] = orig_exists
    result["duplicate_title"] = dup_title
    result["original_title"] = orig_title

    if not dup_exists:
        result["error"] = f"Duplicate issue #{duplicate_number} not found"
        print(json.dumps(result, indent=2))
        sys.exit(1)

    if not orig_exists:
        result["error"] = f"Original issue #{original_number} not found"
        print(json.dumps(result, indent=2))
        sys.exit(1)

    # Ensure duplicate label exists with proper styling
    ensure_label_exists("duplicate", "cfd3d7", "Duplicate of another issue")

    # Add duplicate label
    result["label_added"] = add_label(duplicate_number, "duplicate", create_if_missing=False)

    # Post linking comment
    result["comment_posted"] = post_duplicate_comment(
        duplicate_number, original_number, orig_title
    )

    # Close the duplicate
    result["closed"] = close_issue(duplicate_number, "not planned")

    result["success"] = all([
        result["label_added"],
        result["comment_posted"],
        result["closed"]
    ])

    print(json.dumps(result, indent=2))


if __name__ == "__main__":
    main()
