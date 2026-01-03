#!/usr/bin/env python3
"""
create_issue.py - Create GitHub issues with proper labels

Usage: python create_issue.py --title "Title" --body "Description" [options]

Options:
    --title     (required) Issue title
    --body      (required) Issue body/description
    --domain    (optional) Domain label (ui, combat, store, item, etc.)
    --priority  (optional) Priority level (critical, high, medium, low)
    --labels    (optional) Comma-separated additional labels

The issue will always be created with the 'fresh' label.
Priority labels are formatted as 'priority:level' (e.g., priority:high).

Output: JSON with issue number, URL, and applied labels
"""

import argparse
import json
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


def get_existing_labels() -> set[str]:
    """Fetch all existing labels in the repository."""
    success, output = run_cmd([
        "gh", "label", "list", "--json", "name", "--limit", "200"
    ])
    if not success or not output:
        return set()

    labels = json.loads(output)
    return {label.get("name", "") for label in labels}


def ensure_label_exists(label: str, existing_labels: set[str]) -> tuple[bool, bool]:
    """
    Ensure a label exists, creating it if necessary.
    Returns (success, was_created).
    """
    if label in existing_labels:
        return True, False

    # Default colors for different label types
    color = "ededed"  # Default gray
    if label.startswith("priority:"):
        priority = label.split(":")[1]
        colors = {"critical": "b60205", "high": "d93f0b", "medium": "fbca04", "low": "0e8a16"}
        color = colors.get(priority, "ededed")
    elif label == "fresh":
        color = "1d76db"  # Blue

    success, _ = run_cmd([
        "gh", "label", "create", label, "--color", color, "--force"
    ], check=False)

    return success, True


def create_issue(title: str, body: str, labels: list[str]) -> dict[str, Any] | None:
    """Create a GitHub issue with the given labels."""
    cmd = ["gh", "issue", "create", "--title", title, "--body", body]

    for label in labels:
        cmd.extend(["--label", label])

    success, output = run_cmd(cmd)
    if not success:
        return None

    # Output is the issue URL
    issue_url = output.strip()

    # Extract issue number from URL
    try:
        issue_number = int(issue_url.rstrip("/").split("/")[-1])
    except (ValueError, IndexError):
        issue_number = None

    return {
        "issue_number": issue_number,
        "issue_url": issue_url,
    }


def main():
    parser = argparse.ArgumentParser(description="Create a GitHub issue with proper labels")
    parser.add_argument("--title", required=True, help="Issue title")
    parser.add_argument("--body", required=True, help="Issue body/description")
    parser.add_argument("--domain", help="Domain label (ui, combat, store, etc.)")
    parser.add_argument("--priority", choices=["critical", "high", "medium", "low"],
                        help="Priority level")
    parser.add_argument("--labels", help="Comma-separated additional labels")

    args = parser.parse_args()

    # Build label list
    labels = ["fresh"]  # Always add fresh label

    if args.domain:
        labels.append(args.domain)

    if args.priority:
        labels.append(f"priority:{args.priority}")

    if args.labels:
        extra_labels = [l.strip() for l in args.labels.split(",") if l.strip()]
        labels.extend(extra_labels)

    # Remove duplicates while preserving order
    seen = set()
    unique_labels = []
    for label in labels:
        if label not in seen:
            seen.add(label)
            unique_labels.append(label)
    labels = unique_labels

    # Get existing labels
    existing_labels = get_existing_labels()

    # Ensure all labels exist
    labels_created = []
    for label in labels:
        success, was_created = ensure_label_exists(label, existing_labels)
        if was_created and success:
            labels_created.append(label)
            existing_labels.add(label)

    # Create the issue
    result = create_issue(args.title, args.body, labels)

    if result is None:
        print(json.dumps({"error": "Failed to create issue"}))
        sys.exit(1)

    output = {
        "success": True,
        "issue_number": result["issue_number"],
        "issue_url": result["issue_url"],
        "title": args.title,
        "labels": labels,
        "labels_created": labels_created,
    }

    print(json.dumps(output, indent=2))


if __name__ == "__main__":
    main()
