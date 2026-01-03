#!/usr/bin/env python3
"""
fix_complete.py - Post-fix automation for GitHub issues

Usage: python fix_complete.py <issue_number> "<resolution_summary>"

Actions:
1. Add 'fix-attempted' label to issue
2. Post resolution comment with summary
3. Commit staged changes with proper message
4. Push branch
5. Merge to main (fast-forward)
6. Push main
7. Delete feature branch (local + remote)

Output: JSON with status of each action
"""

import json
import re
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


def get_issue_title(issue_number: int) -> str:
    """Fetch issue title from GitHub."""
    success, output = run_cmd(["gh", "issue", "view", str(issue_number), "--json", "title"])
    if success:
        data = json.loads(output)
        return data.get("title", "")
    return ""


def get_current_branch() -> str:
    """Get current git branch name."""
    success, output = run_cmd(["git", "branch", "--show-current"])
    return output if success else ""


def add_label(issue_number: int, label: str) -> bool:
    """Add a label to an issue."""
    success, _ = run_cmd(["gh", "issue", "edit", str(issue_number), "--add-label", label])
    return success


def post_comment(issue_number: int, comment: str) -> bool:
    """Post a comment on an issue."""
    success, _ = run_cmd(["gh", "issue", "comment", str(issue_number), "--body", comment])
    return success


def commit_changes(issue_number: int, title: str) -> tuple[bool, str]:
    """Commit staged changes with proper message format."""
    # Build commit message
    commit_msg = f"""fix: {title}

Closes #{issue_number}

Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude Opus 4.5 <noreply@anthropic.com>"""

    success, output = run_cmd(["git", "commit", "-m", commit_msg], check=False)
    if success:
        # Get commit SHA
        _, sha = run_cmd(["git", "rev-parse", "HEAD"])
        return True, sha[:7]
    return False, output


def push_branch(branch: str) -> bool:
    """Push branch to origin."""
    success, _ = run_cmd(["git", "push", "-u", "origin", branch])
    return success


def merge_to_main(branch: str) -> bool:
    """Checkout main, merge branch, push."""
    # Checkout main
    success, _ = run_cmd(["git", "checkout", "main"])
    if not success:
        return False

    # Merge branch
    success, _ = run_cmd(["git", "merge", branch])
    if not success:
        return False

    # Push main
    success, _ = run_cmd(["git", "push", "origin", "main"])
    return success


def delete_branch(branch: str) -> bool:
    """Delete branch locally and remotely."""
    # Delete local
    run_cmd(["git", "branch", "-d", branch], check=False)

    # Delete remote
    success, _ = run_cmd(["git", "push", "origin", "--delete", branch], check=False)
    return success


def format_resolution_comment(summary: str) -> str:
    """Format the resolution comment."""
    return f"""## Resolution

{summary}

---
*Automated via fix_complete.py*"""


def main():
    if len(sys.argv) < 3:
        print(json.dumps({"error": "Usage: fix_complete.py <issue_number> <resolution_summary>"}))
        sys.exit(1)

    try:
        issue_number = int(sys.argv[1])
    except ValueError:
        print(json.dumps({"error": "Issue number must be an integer"}))
        sys.exit(1)

    resolution_summary = sys.argv[2]

    result: dict[str, Any] = {
        "issue": issue_number,
        "label_added": False,
        "comment_posted": False,
        "commit_sha": None,
        "merged": False,
        "branch_deleted": False,
    }

    # Get current branch
    current_branch = get_current_branch()
    if current_branch == "main":
        print(json.dumps({"error": "Cannot run from main branch. Checkout your feature branch first."}))
        sys.exit(1)

    result["branch"] = current_branch

    # Get issue title
    title = get_issue_title(issue_number)
    if not title:
        print(json.dumps({"error": f"Could not fetch issue #{issue_number}"}))
        sys.exit(1)

    result["title"] = title

    # 1. Add label
    result["label_added"] = add_label(issue_number, "fix-attempted")

    # 2. Post comment
    comment = format_resolution_comment(resolution_summary)
    result["comment_posted"] = post_comment(issue_number, comment)

    # 3. Commit changes (if there are staged changes)
    success, output = run_cmd(["git", "diff", "--cached", "--quiet"], check=False)
    if not success:  # There are staged changes
        commit_success, sha = commit_changes(issue_number, title)
        result["commit_sha"] = sha if commit_success else None
    else:
        result["commit_sha"] = "no_changes"

    # 4. Push branch
    result["branch_pushed"] = push_branch(current_branch)

    # 5. Merge to main
    result["merged"] = merge_to_main(current_branch)

    # 6. Delete branch
    if result["merged"]:
        result["branch_deleted"] = delete_branch(current_branch)

    print(json.dumps(result, indent=2))


if __name__ == "__main__":
    main()
