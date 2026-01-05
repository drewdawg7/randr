#!/usr/bin/env python3
"""Merge current branch to main and cleanup."""
import json
import subprocess
import sys

def run_git(*args):
    """Run a git command and return result."""
    result = subprocess.run(
        ["git"] + list(args),
        capture_output=True,
        text=True
    )
    return result

def get_current_branch():
    """Get the current branch name."""
    result = run_git("branch", "--show-current")
    return result.stdout.strip()

def has_uncommitted_changes():
    """Check for uncommitted changes."""
    result = run_git("status", "--porcelain")
    return bool(result.stdout.strip())

def merge_to_main(delete_branch=True, push=True):
    """Merge current branch to main."""
    current = get_current_branch()

    if current in ("main", "master"):
        return {
            "success": False,
            "error": "Already on main branch, nothing to merge"
        }

    if has_uncommitted_changes():
        return {
            "success": False,
            "error": "Uncommitted changes exist",
            "hint": "Commit or stash changes before merging"
        }

    # Fetch latest main
    fetch_result = run_git("fetch", "origin", "main")

    # Switch to main
    checkout_result = run_git("checkout", "main")
    if checkout_result.returncode != 0:
        return {"success": False, "error": f"Failed to checkout main: {checkout_result.stderr}"}

    # Pull latest
    pull_result = run_git("pull", "origin", "main")
    if pull_result.returncode != 0:
        # Try to recover
        run_git("checkout", current)
        return {"success": False, "error": f"Failed to pull main: {pull_result.stderr}"}

    # Merge feature branch
    merge_result = run_git("merge", current, "--no-ff", "-m", f"Merge branch '{current}'")
    if merge_result.returncode != 0:
        run_git("merge", "--abort")
        run_git("checkout", current)
        return {
            "success": False,
            "error": "Merge conflict",
            "hint": "Resolve conflicts manually or rebase first"
        }

    result = {
        "success": True,
        "merged": current,
        "into": "main"
    }

    # Push to remote
    if push:
        push_result = run_git("push", "origin", "main")
        if push_result.returncode != 0:
            result["push_error"] = push_result.stderr.strip()
        else:
            result["pushed"] = True

    # Delete feature branch
    if delete_branch:
        run_git("branch", "-d", current)
        run_git("push", "origin", "--delete", current)
        result["deleted_branch"] = current

    return result

def main():
    delete = "--keep" not in sys.argv
    push = "--no-push" not in sys.argv

    result = merge_to_main(delete_branch=delete, push=push)
    print(json.dumps(result, indent=2))

if __name__ == "__main__":
    main()
