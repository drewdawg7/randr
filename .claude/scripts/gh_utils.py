#!/usr/bin/env python3
"""
gh_utils.py - Shared utilities for GitHub CLI operations

Common patterns extracted from skill scripts for DRY compliance.
All scripts in .claude/scripts/ should import from this module.

Usage:
    import sys
    from pathlib import Path
    sys.path.insert(0, str(Path(__file__).parent.parent))
    from gh_utils import run_cmd, get_issue_details
"""

import json
import re
import subprocess
from datetime import datetime, timezone
from typing import Any


# ==============================================================================
# COMMAND EXECUTION
# ==============================================================================

def run_cmd(cmd: list[str], check: bool = True) -> tuple[bool, str]:
    """
    Run a command and return (success, output).

    Args:
        cmd: Command as list of strings (e.g., ["gh", "issue", "list"])
        check: If True, treat non-zero exit as failure

    Returns:
        Tuple of (success: bool, output: str)
    """
    try:
        result = subprocess.run(cmd, capture_output=True, text=True, check=check)
        return True, result.stdout.strip()
    except subprocess.CalledProcessError as e:
        return False, e.stderr.strip()


def run_gh(args: list[str]) -> str:
    """
    Run gh CLI command, exit on failure.

    For scripts that want fail-fast behavior rather than error handling.
    """
    result = subprocess.run(["gh"] + args, capture_output=True, text=True)
    if result.returncode != 0:
        import sys
        print(f"Error: {result.stderr}", file=sys.stderr)
        sys.exit(1)
    return result.stdout.strip()


# ==============================================================================
# ISSUE FETCHING
# ==============================================================================

def get_issue_details(issue_number: int,
                      fields: str = "number,title,body,labels,createdAt,state"
                      ) -> dict[str, Any] | None:
    """
    Fetch issue details from GitHub.

    Args:
        issue_number: GitHub issue number
        fields: Comma-separated JSON fields to fetch

    Returns:
        Issue data dict or None if not found
    """
    success, output = run_cmd([
        "gh", "issue", "view", str(issue_number), "--json", fields
    ])
    if success and output:
        return json.loads(output)
    return None


def get_issue_comments(issue_number: int) -> list[dict[str, str]]:
    """
    Fetch all comments on an issue via GitHub API.

    Returns:
        List of {author, body, created_at} dicts
    """
    success, output = run_cmd([
        "gh", "api",
        f"repos/:owner/:repo/issues/{issue_number}/comments",
        "--jq", '.[] | {author: .user.login, body: .body, created_at: .created_at}'
    ])

    if not success or not output:
        return []

    comments = []
    for line in output.strip().split('\n'):
        if line:
            try:
                comments.append(json.loads(line))
            except json.JSONDecodeError:
                continue
    return comments


def get_issue_comments_full(issue_number: int) -> list[dict[str, Any]]:
    """
    Fetch all comments on an issue via GitHub API (full JSON).

    Returns:
        List of comment dicts with author, body, created_at
    """
    success, output = run_cmd([
        "gh", "api", f"repos/:owner/:repo/issues/{issue_number}/comments"
    ])

    if not success or not output:
        return []

    comments = json.loads(output)
    return [
        {
            "author": c.get("user", {}).get("login", "unknown"),
            "body": c.get("body", ""),
            "created_at": c.get("created_at", ""),
        }
        for c in comments
    ]


def list_issues(label: str = None, state: str = "open",
                fields: str = "number,title,labels,createdAt",
                limit: int = 100) -> list[dict[str, Any]]:
    """
    List issues with optional filters.

    Args:
        label: Filter by label (optional)
        state: open, closed, or all
        fields: JSON fields to return
        limit: Maximum issues to fetch

    Returns:
        List of issue dicts
    """
    cmd = ["gh", "issue", "list", "--state", state, "--json", fields, "--limit", str(limit)]
    if label:
        cmd.extend(["--label", label])

    success, output = run_cmd(cmd)
    if not success or not output:
        return []
    return json.loads(output)


def issue_exists(number: int) -> tuple[bool, str]:
    """Check if issue exists and return its title."""
    success, output = run_cmd([
        "gh", "issue", "view", str(number),
        "--json", "number,title,state"
    ], check=False)

    if success:
        data = json.loads(output)
        return True, data.get("title", "")
    return False, ""


# ==============================================================================
# LABEL OPERATIONS
# ==============================================================================

def add_label(issue_number: int, label: str, create_if_missing: bool = True) -> bool:
    """
    Add a label to an issue.

    Args:
        issue_number: GitHub issue number
        label: Label name to add
        create_if_missing: If True, create label if it doesn't exist

    Returns:
        True if successful
    """
    if create_if_missing:
        run_cmd(["gh", "label", "create", label, "--force"], check=False)

    success, _ = run_cmd([
        "gh", "issue", "edit", str(issue_number), "--add-label", label
    ], check=False)
    return success


def remove_label(issue_number: int, label: str) -> bool:
    """Remove a label from an issue."""
    success, _ = run_cmd([
        "gh", "issue", "edit", str(issue_number), "--remove-label", label
    ], check=False)
    return success


def ensure_label_exists(label: str, color: str = "ededed",
                        description: str = "") -> bool:
    """Create label if it doesn't exist."""
    cmd = ["gh", "label", "create", label, "--color", color, "--force"]
    if description:
        cmd.extend(["--description", description])
    success, _ = run_cmd(cmd, check=False)
    return success


def get_existing_labels() -> set[str]:
    """Fetch all existing labels in the repository."""
    success, output = run_cmd([
        "gh", "label", "list", "--json", "name", "--limit", "200"
    ])
    if not success or not output:
        return set()

    labels = json.loads(output)
    return {label.get("name", "") for label in labels}


def get_all_labels() -> list[dict[str, str]]:
    """Fetch all labels from the repository with details."""
    success, output = run_cmd([
        "gh", "label", "list", "--json", "name,description,color", "--limit", "200"
    ])
    if not success or not output:
        return []
    return json.loads(output)


def get_issue_labels(issue_number: int) -> list[str]:
    """Fetch current labels for an issue."""
    success, output = run_cmd([
        "gh", "issue", "view", str(issue_number), "--json", "labels"
    ])

    if not success or not output:
        return []

    data = json.loads(output)
    return [label.get("name", "") for label in data.get("labels", [])]


# ==============================================================================
# TEXT EXTRACTION
# ==============================================================================

def extract_file_references(text: str) -> list[str]:
    """
    Extract file paths mentioned in text.

    Looks for patterns like:
    - `src/path/file.rs` (in backticks)
    - src/path/file.rs (without backticks)
    - `filename.{rs,py,md,toml}` (any code file in backticks)

    Returns:
        Sorted list of unique file paths
    """
    if not text:
        return []

    patterns = [
        r'`(src/[^`]+\.(rs|py|md|toml))`',      # Files in backticks
        r'(?:^|\s)(src/\S+\.(rs|py|md|toml))',  # Files without backticks
        r'`([^`]+\.(rs|py|md|toml))`',          # Any file in backticks
    ]

    files = set()
    for pattern in patterns:
        matches = re.findall(pattern, text, re.MULTILINE)
        for match in matches:
            if isinstance(match, tuple):
                files.add(match[0])
            else:
                files.add(match)

    return sorted(files)


def extract_keywords(title: str, body: str) -> list[str]:
    """Extract potential search keywords from title and body."""
    text = f"{title} {body}".lower()

    keywords = set()
    patterns = [
        r'\b(player|mob|combat|item|inventory|store|blacksmith|mine|stat|ui)\b',
        r'\b(attack|defense|health|damage|gold|xp|level)\b',
        r'\b(weapon|shield|armor|equipment)\b',
        r'\b(spawn|drop|loot|reward)\b',
    ]

    for pattern in patterns:
        matches = re.findall(pattern, text)
        keywords.update(matches)

    return sorted(keywords)


# ==============================================================================
# PRIORITY & AGE
# ==============================================================================

PRIORITY_RANKS = {
    "critical": 0,
    "high": 1,
    "medium": 2,
    "low": 3,
}


def get_priority(labels: list[str]) -> tuple[int, str]:
    """
    Determine priority rank from labels.

    Searches for labels containing priority keywords.

    Returns:
        (priority_rank, priority_name) where lower rank = higher priority
    """
    labels_lower = [label.lower() for label in labels]

    for label in labels_lower:
        for keyword, rank in PRIORITY_RANKS.items():
            if keyword in label:
                return (rank, keyword)

    return (4, "none")


def calculate_age_days(created_at: str) -> int:
    """
    Calculate age in days from ISO timestamp.

    Args:
        created_at: ISO 8601 timestamp (e.g., "2024-01-15T10:30:00Z")

    Returns:
        Age in days (0 if parse fails)
    """
    try:
        created = datetime.fromisoformat(created_at.replace("Z", "+00:00"))
        now = datetime.now(timezone.utc)
        return (now - created).days
    except (ValueError, TypeError):
        return 0


# ==============================================================================
# COMMENT & ISSUE OPERATIONS
# ==============================================================================

def post_comment(issue_number: int, comment: str) -> bool:
    """Post a comment on an issue."""
    success, _ = run_cmd([
        "gh", "issue", "comment", str(issue_number), "--body", comment
    ])
    return success


def close_issue(issue_number: int, reason: str = "completed") -> bool:
    """Close an issue with optional reason."""
    cmd = ["gh", "issue", "close", str(issue_number)]
    if reason:
        cmd.extend(["--reason", reason])
    success, _ = run_cmd(cmd)
    return success


def update_issue_body(issue_number: int, new_body: str) -> bool:
    """Update issue body."""
    success, _ = run_cmd([
        "gh", "issue", "edit", str(issue_number), "--body", new_body
    ])
    return success


# ==============================================================================
# GIT OPERATIONS
# ==============================================================================

def get_current_branch() -> str:
    """Get current git branch name."""
    success, output = run_cmd(["git", "branch", "--show-current"])
    return output if success else ""


def checkout_main_and_pull() -> bool:
    """Checkout main and pull latest."""
    success, _ = run_cmd(["git", "checkout", "main"])
    if not success:
        return False

    success, _ = run_cmd(["git", "pull", "origin", "main"])
    return success


def create_branch(branch_name: str) -> bool:
    """Create and checkout a new branch."""
    success, _ = run_cmd(["git", "checkout", "-b", branch_name])
    return success


def push_branch(branch: str) -> bool:
    """Push branch to origin."""
    success, _ = run_cmd(["git", "push", "-u", "origin", branch])
    return success


def merge_to_main(branch: str) -> bool:
    """Checkout main, merge branch, push."""
    success, _ = run_cmd(["git", "checkout", "main"])
    if not success:
        return False

    success, _ = run_cmd(["git", "merge", branch])
    if not success:
        return False

    success, _ = run_cmd(["git", "push", "origin", "main"])
    return success


def delete_branch(branch: str) -> bool:
    """Delete branch locally and remotely."""
    run_cmd(["git", "branch", "-d", branch], check=False)
    success, _ = run_cmd(["git", "push", "origin", "--delete", branch], check=False)
    return success


# ==============================================================================
# TEXT UTILITIES
# ==============================================================================

def slugify(text: str, max_length: int = 40) -> str:
    """Convert text to URL-friendly slug."""
    slug = text.lower()
    slug = re.sub(r'[^a-z0-9]+', '-', slug)
    slug = slug.strip('-')
    if len(slug) > max_length:
        slug = slug[:max_length].rstrip('-')
    return slug


def normalize_text(text: str) -> str:
    """Lowercase, remove punctuation, normalize whitespace."""
    if not text:
        return ""
    text = text.lower()
    text = re.sub(r'[^\w\s]', ' ', text)
    text = re.sub(r'\s+', ' ', text).strip()
    return text


def get_keywords_from_text(text: str) -> set[str]:
    """Extract meaningful keywords (3+ chars) from text."""
    words = normalize_text(text).split()
    stop_words = {
        'the', 'and', 'for', 'that', 'this', 'with', 'from', 'are', 'was',
        'will', 'can', 'has', 'have', 'been', 'being', 'would', 'could',
        'should', 'into', 'also', 'when', 'where', 'which', 'while', 'about'
    }
    return {w for w in words if len(w) >= 3 and w not in stop_words}
