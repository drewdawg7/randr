#!/usr/bin/env python3
"""
Process selected option in GitHub issue body.

After user checks an option checkbox, this script:
1. Identifies the selected option
2. Rewrites issue body with selected visible, others in <details>
3. Removes needs-decision label

Usage: python3 option_selector.py <issue_number>
"""

import json
import re
import subprocess
import sys


def run_gh(args: list[str]) -> str:
    """Run gh CLI command and return output."""
    result = subprocess.run(["gh"] + args, capture_output=True, text=True)
    if result.returncode != 0:
        print(f"Error: {result.stderr}", file=sys.stderr)
        sys.exit(1)
    return result.stdout.strip()


def get_issue_body(issue_number: str) -> str:
    """Fetch issue body."""
    return run_gh(["issue", "view", issue_number, "--json", "body", "-q", ".body"])


def get_issue_labels(issue_number: str) -> list[str]:
    """Fetch issue labels."""
    output = run_gh(["issue", "view", issue_number, "--json", "labels", "-q", ".labels[].name"])
    return output.split("\n") if output else []


def update_issue_body(issue_number: str, new_body: str) -> None:
    """Update issue body."""
    run_gh(["issue", "edit", issue_number, "--body", new_body])


def remove_label(issue_number: str, label: str) -> None:
    """Remove label from issue."""
    run_gh(["issue", "edit", issue_number, "--remove-label", label])


def parse_options(body: str) -> tuple[str | None, list[dict], str, str]:
    """
    Parse options from issue body.

    Returns:
        - section_header: The header before options (e.g., "## Suggested Options")
        - options: List of {checked: bool, text: str}
        - before: Text before options section
        - after: Text after options section
    """
    # Pattern to match options section
    # Looks for header followed by checkbox items
    options_pattern = r'(## (?:Suggested )?Options?\s*\n)((?:- \[[ x]\] .+\n?)+)'

    match = re.search(options_pattern, body, re.IGNORECASE)
    if not match:
        return None, [], body, ""

    header = match.group(1)
    options_block = match.group(2)
    before = body[:match.start()]
    after = body[match.end():]

    # Parse individual options
    option_pattern = r'- \[([ x])\] (.+?)(?:\n|$)'
    options = []
    for m in re.finditer(option_pattern, options_block):
        options.append({
            "checked": m.group(1) == "x",
            "text": m.group(2).strip()
        })

    return header, options, before, after


def format_selected_output(selected: dict, others: list[dict]) -> str:
    """Format the output with selected option visible and others collapsed."""
    output = "## Selected Approach\n"
    output += f"{selected['text']}\n\n"

    if others:
        output += "<details>\n"
        output += "<summary>Other options considered</summary>\n\n"
        for opt in others:
            output += f"- {opt['text']}\n"
        output += "\n</details>\n"

    return output


def main():
    if len(sys.argv) != 2:
        print("Usage: python3 option_selector.py <issue_number>", file=sys.stderr)
        sys.exit(1)

    issue_number = sys.argv[1]

    # Fetch issue body
    body = get_issue_body(issue_number)

    # Parse options
    header, options, before, after = parse_options(body)

    if not options:
        print(json.dumps({
            "success": False,
            "error": "No options section found in issue body"
        }))
        sys.exit(0)

    # Find selected option
    selected = [o for o in options if o["checked"]]

    if len(selected) == 0:
        print(json.dumps({
            "success": False,
            "error": "No option selected (none checked)",
            "options": [o["text"] for o in options]
        }))
        sys.exit(0)

    if len(selected) > 1:
        print(json.dumps({
            "success": False,
            "error": "Multiple options selected (only one allowed)",
            "selected": [o["text"] for o in selected]
        }))
        sys.exit(0)

    selected_option = selected[0]
    other_options = [o for o in options if not o["checked"]]

    # Build new body
    formatted = format_selected_output(selected_option, other_options)
    new_body = before.rstrip() + "\n\n" + formatted + "\n" + after.lstrip()

    # Update issue
    update_issue_body(issue_number, new_body)

    # Remove needs-decision label if present
    labels = get_issue_labels(issue_number)
    if "needs-decision" in labels:
        remove_label(issue_number, "needs-decision")

    print(json.dumps({
        "success": True,
        "selected": selected_option["text"],
        "collapsed": [o["text"] for o in other_options],
        "label_removed": "needs-decision" in labels
    }))


if __name__ == "__main__":
    main()
