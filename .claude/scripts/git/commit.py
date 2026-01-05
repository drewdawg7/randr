#!/usr/bin/env python3
"""Create conventional commits with validation.

Usage:
    commit.py <message>                    # Commit staged files
    commit.py <message> <file1> <file2>    # Stage and commit specific files
    commit.py <message> --all              # Stage all modified files and commit
    commit.py <message> --issue 42         # Append "Closes #42" to commit message

Examples:
    python3 commit.py "feat: add inventory"
    python3 commit.py "fix: resolve crash" --issue 15
    python3 commit.py "refactor: clean up" --all
    python3 commit.py "feat: new feature" src/main.rs --issue 42
"""
import json
import subprocess
import sys
import re
import argparse

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

def commit(message, files=None, stage_all=False, issue=None):
    """Create a commit with optional file staging.

    Args:
        message: Commit message (conventional commit format)
        files: Optional list of specific files to stage
        stage_all: If True, stage all modified tracked files
        issue: Optional issue number to close (appends "Closes #N")
    """
    # Validate message
    validation = validate_message(message)
    if not validation["valid"]:
        return {"success": False, **validation}

    # Stage all modified files if requested
    if stage_all:
        result = run_git("add", "-u")  # Stage all modified tracked files
        if result.returncode != 0:
            return {"success": False, "error": f"Failed to stage files: {result.stderr}"}

    # Stage specific files if provided
    if files:
        for f in files:
            result = run_git("add", f)
            if result.returncode != 0:
                return {"success": False, "error": f"Failed to stage {f}: {result.stderr}"}

    staged = get_staged_files()
    if not staged:
        unstaged = get_unstaged_files()
        untracked = get_untracked_files()
        hint = ""
        if unstaged:
            hint = "Tip: Use --all to stage modified files, or pass file paths as arguments"
        elif untracked:
            hint = "Tip: Pass file paths as arguments to stage untracked files"
        return {
            "success": False,
            "error": "No files staged for commit",
            "hint": hint,
            "unstaged": unstaged,
            "untracked": untracked
        }

    # Append issue closer if provided
    final_message = message
    if issue:
        final_message = f"{message}\n\nCloses #{issue}"

    # Create commit
    result = run_git("commit", "-m", final_message)
    if result.returncode != 0:
        return {"success": False, "error": result.stderr.strip()}

    # Get commit hash
    hash_result = run_git("rev-parse", "--short", "HEAD")

    response = {
        "success": True,
        "hash": hash_result.stdout.strip(),
        "message": message,
        "files": staged
    }
    if issue:
        response["closes_issue"] = issue
    return response

def status():
    """Get current git status."""
    return {
        "staged": get_staged_files(),
        "unstaged": get_unstaged_files(),
        "untracked": get_untracked_files()
    }

def parse_args():
    """Parse command line arguments."""
    parser = argparse.ArgumentParser(
        description="Create conventional commits with validation",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
    %(prog)s "feat: add inventory"
    %(prog)s "fix: resolve crash" --issue 15
    %(prog)s "refactor: clean up" --all
    %(prog)s "feat: new feature" src/main.rs --issue 42
        """
    )
    parser.add_argument("message", nargs="?", help="Commit message (conventional format)")
    parser.add_argument("files", nargs="*", help="Files to stage before committing")
    parser.add_argument("--all", "-a", action="store_true", dest="stage_all",
                        help="Stage all modified tracked files before committing")
    parser.add_argument("--issue", "-i", type=int, metavar="N",
                        help="Issue number to close (appends 'Closes #N' to message)")
    return parser.parse_args()


def main():
    args = parse_args()

    if not args.message:
        print(json.dumps(status(), indent=2))
        return

    result = commit(
        message=args.message,
        files=args.files if args.files else None,
        stage_all=args.stage_all,
        issue=args.issue
    )
    print(json.dumps(result, indent=2))


if __name__ == "__main__":
    main()
