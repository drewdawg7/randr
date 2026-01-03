#!/usr/bin/env python3
"""
label_manager.py - List and manage GitHub labels

Usage:
    python label_manager.py --list              # List all labels by category
    python label_manager.py --create "name"     # Create a new label
    python label_manager.py --create "name" --description "desc" --color "hex"

Options:
    --list          List all labels, categorized by type
    --create NAME   Create a new label with the given name
    --description   Description for new label (optional)
    --color         Hex color for new label without # (optional)

Output: JSON with label information
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


def get_all_labels() -> list[dict[str, str]]:
    """Fetch all labels from the repository."""
    success, output = run_cmd([
        "gh", "label", "list", "--json", "name,description,color", "--limit", "200"
    ])
    if not success or not output:
        return []

    return json.loads(output)


def categorize_labels(labels: list[dict[str, str]]) -> dict[str, list[str]]:
    """Categorize labels by type."""
    # Known status labels
    status_labels = {"fresh", "under research", "researched", "fix-attempted"}

    # Priority labels start with 'priority:'
    priority_labels = []
    domain_labels = []
    other_labels = []
    found_status = []

    for label in labels:
        name = label.get("name", "")
        name_lower = name.lower()

        if name_lower in status_labels:
            found_status.append(name)
        elif name_lower.startswith("priority:"):
            priority_labels.append(name)
        elif name_lower in {"bug", "enhancement", "documentation", "duplicate",
                           "good first issue", "help wanted", "invalid",
                           "question", "wontfix"}:
            # GitHub default labels
            other_labels.append(name)
        else:
            # Assume it's a domain label
            domain_labels.append(name)

    return {
        "all_labels": sorted([l.get("name", "") for l in labels]),
        "domain_labels": sorted(domain_labels),
        "priority_labels": sorted(priority_labels),
        "status_labels": sorted(found_status),
        "other_labels": sorted(other_labels),
    }


def create_label(name: str, description: str = "", color: str = "") -> dict[str, Any]:
    """Create a new label."""
    cmd = ["gh", "label", "create", name]

    if description:
        cmd.extend(["--description", description])

    if color:
        # Remove # if present
        color = color.lstrip("#")
        cmd.extend(["--color", color])

    cmd.append("--force")  # Update if exists

    success, output = run_cmd(cmd, check=False)

    if success:
        return {
            "success": True,
            "label": name,
            "description": description,
            "color": color or "default",
        }
    else:
        return {
            "success": False,
            "error": output,
            "label": name,
        }


def main():
    parser = argparse.ArgumentParser(description="List and manage GitHub labels")
    parser.add_argument("--list", action="store_true", help="List all labels by category")
    parser.add_argument("--create", metavar="NAME", help="Create a new label")
    parser.add_argument("--description", default="", help="Description for new label")
    parser.add_argument("--color", default="", help="Hex color for new label (without #)")

    args = parser.parse_args()

    if not args.list and not args.create:
        parser.print_help()
        sys.exit(1)

    if args.list:
        labels = get_all_labels()
        categorized = categorize_labels(labels)
        print(json.dumps(categorized, indent=2))

    elif args.create:
        result = create_label(args.create, args.description, args.color)
        print(json.dumps(result, indent=2))


if __name__ == "__main__":
    main()
