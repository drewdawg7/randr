#!/usr/bin/env python3
"""Create or switch git branches with naming convention enforcement."""
import json
import subprocess
import sys
import re

VALID_TYPES = ["feat", "fix", "refactor", "docs", "test", "chore"]

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

def branch_exists(name):
    """Check if a branch exists locally or remotely."""
    local = run_git("rev-parse", "--verify", name)
    if local.returncode == 0:
        return True
    remote = run_git("rev-parse", "--verify", f"origin/{name}")
    return remote.returncode == 0

def create_branch(name):
    """Create and checkout a new branch."""
    # Validate format
    pattern = r"^(feat|fix|refactor|docs|test|chore)/[a-z0-9-]+$"
    if not re.match(pattern, name):
        return {
            "success": False,
            "error": f"Invalid branch name: {name}",
            "hint": f"Format: <type>/<description> where type is one of: {', '.join(VALID_TYPES)}",
            "example": "feat/add-inventory"
        }

    if branch_exists(name):
        # Switch to existing branch
        result = run_git("checkout", name)
        if result.returncode != 0:
            return {"success": False, "error": result.stderr.strip()}
        return {
            "success": True,
            "action": "switched",
            "branch": name
        }

    # Create new branch from main
    run_git("fetch", "origin", "main")
    result = run_git("checkout", "-b", name, "origin/main")
    if result.returncode != 0:
        # Try without origin/main
        result = run_git("checkout", "-b", name)
        if result.returncode != 0:
            return {"success": False, "error": result.stderr.strip()}

    return {
        "success": True,
        "action": "created",
        "branch": name
    }

def list_branches():
    """List all local branches."""
    result = run_git("branch", "--list")
    branches = [b.strip().lstrip("* ") for b in result.stdout.splitlines()]
    current = get_current_branch()
    return {
        "success": True,
        "current": current,
        "branches": branches
    }

def main():
    if len(sys.argv) < 2:
        print(json.dumps(list_branches(), indent=2))
        return

    action = sys.argv[1]

    if action == "--list":
        print(json.dumps(list_branches(), indent=2))
    elif action == "--current":
        print(json.dumps({"branch": get_current_branch()}))
    else:
        # Treat as branch name to create/switch
        result = create_branch(action)
        print(json.dumps(result, indent=2))

if __name__ == "__main__":
    main()
