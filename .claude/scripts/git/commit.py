#!/usr/bin/env python3
"""Create conventional commits with validation."""
import json
import subprocess
import sys
import re

VALID_TYPES = ["feat", "fix", "refactor", "docs", "test", "chore", "style", "perf"]

def run_git(*args):
    """Run a git command and return result."""
    result = subprocess.run(
        ["git"] + list(args),
        capture_output=True,
        text=True
    )
    return result

def get_staged_files():
    """Get list of staged files."""
    result = run_git("diff", "--cached", "--name-only")
    return result.stdout.strip().splitlines() if result.stdout.strip() else []

def get_unstaged_files():
    """Get list of modified but unstaged files."""
    result = run_git("diff", "--name-only")
    return result.stdout.strip().splitlines() if result.stdout.strip() else []

def get_untracked_files():
    """Get list of untracked files."""
    result = run_git("ls-files", "--others", "--exclude-standard")
    return result.stdout.strip().splitlines() if result.stdout.strip() else []

def validate_message(message):
    """Validate conventional commit message format."""
    pattern = r"^(feat|fix|refactor|docs|test|chore|style|perf)(\(.+\))?: .+"
    if not re.match(pattern, message):
        return {
            "valid": False,
            "error": "Invalid commit message format",
            "hint": f"Format: <type>[(scope)]: <description>",
            "types": VALID_TYPES,
            "example": "feat(inventory): add item stacking"
        }
    return {"valid": True}

def commit(message, files=None):
    """Create a commit with optional file staging."""
    # Validate message
    validation = validate_message(message)
    if not validation["valid"]:
        return {"success": False, **validation}

    # Stage files if provided
    if files:
        for f in files:
            result = run_git("add", f)
            if result.returncode != 0:
                return {"success": False, "error": f"Failed to stage {f}: {result.stderr}"}

    staged = get_staged_files()
    if not staged:
        return {
            "success": False,
            "error": "No files staged for commit",
            "unstaged": get_unstaged_files(),
            "untracked": get_untracked_files()
        }

    # Create commit
    result = run_git("commit", "-m", message)
    if result.returncode != 0:
        return {"success": False, "error": result.stderr.strip()}

    # Get commit hash
    hash_result = run_git("rev-parse", "--short", "HEAD")

    return {
        "success": True,
        "hash": hash_result.stdout.strip(),
        "message": message,
        "files": staged
    }

def status():
    """Get current git status."""
    return {
        "staged": get_staged_files(),
        "unstaged": get_unstaged_files(),
        "untracked": get_untracked_files()
    }

def main():
    if len(sys.argv) < 2:
        print(json.dumps(status(), indent=2))
        return

    message = sys.argv[1]
    files = sys.argv[2:] if len(sys.argv) > 2 else None

    result = commit(message, files)
    print(json.dumps(result, indent=2))

if __name__ == "__main__":
    main()
