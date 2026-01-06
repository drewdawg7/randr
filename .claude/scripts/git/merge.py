#!/usr/bin/env python3
"""Merge current branch to main and cleanup."""
import json
import subprocess
import sys
from pathlib import Path

# Import feedback generator and session state
sys.path.insert(0, str(Path(__file__).parent.parent / "feedback"))
sys.path.insert(0, str(Path(__file__).parent.parent.parent / "hooks"))
from generate import generate_feedback
from session_state import get_state


def validate_workflow_compliance() -> dict:
    """Validate that workflow requirements are met before merge.

    Returns dict with:
    - compliant: bool - whether all requirements are met
    - violations: list - specific violations found
    - warnings: list - non-blocking issues
    """
    state = get_state()
    summary = state.get_summary()

    violations = []
    warnings = []

    # P1: Stability checks (BLOCKING)
    if summary.get("reverts_needed", 0) > 0:
        violations.append(f"Reverts were needed ({summary['reverts_needed']}x) - review for stability issues")

    if summary.get("compilation_errors", 0) > 0:
        violations.append(f"Compilation errors occurred ({summary['compilation_errors']}x)")

    if not summary.get("find_references_compliant", True):
        removals = summary.get("removals_attempted", 0)
        checked = summary.get("removals_with_check", 0)
        violations.append(f"Code removals without findReferences check ({checked}/{removals} checked)")

    # P2: Token efficiency checks (WARNINGS)
    edit_count = summary.get("edit_count", 0)
    ast_grep_calls = summary.get("ast_grep_calls", 0)
    if edit_count > 5 and ast_grep_calls == 0:
        warnings.append(f"High manual edit count ({edit_count}) without ast-grep - consider batch operations")

    # P3: Tool usage checks (WARNINGS)
    grep_blocked = summary.get("grep_blocked", 0)
    if grep_blocked > 0:
        warnings.append(f"Grep was blocked {grep_blocked}x - LSP should be used for Rust navigation")

    # Agent delegation check (WARNING for now, could be violation)
    delegation_used = summary.get("delegation_used", False)
    if edit_count > 0 and not delegation_used:
        agent_delegations = summary.get("agent_delegations", {})
        total_delegations = sum(agent_delegations.values())
        if total_delegations == 0:
            warnings.append(f"Direct edits made ({edit_count}) without agent delegation")

    # Test check (WARNING)
    tests_run = summary.get("tests_run", False)
    tests_passed = summary.get("tests_passed", None)
    if not tests_run:
        warnings.append("Tests were not run during this session")
    elif tests_passed is False:
        violations.append("Tests are failing - fix before merge")

    return {
        "compliant": len(violations) == 0,
        "violations": violations,
        "warnings": warnings,
        "summary": summary
    }

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
    """Check for uncommitted changes (tracked files only).

    Ignores untracked files since they don't affect merge operations.
    """
    result = run_git("status", "--porcelain", "--untracked-files=no")
    return bool(result.stdout.strip())

def merge_to_main(delete_branch=True, push=True, skip_feedback=False, force=False):
    """Merge current branch to main.

    Args:
        delete_branch: Delete feature branch after merge
        push: Push to remote after merge
        skip_feedback: Skip feedback generation
        force: Force merge even with workflow violations (not recommended)
    """
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

    # Validate workflow compliance BEFORE allowing merge
    validation = validate_workflow_compliance()
    if not validation["compliant"] and not force:
        return {
            "success": False,
            "error": "Workflow violations detected - merge blocked",
            "violations": validation["violations"],
            "warnings": validation["warnings"],
            "hint": "Fix violations before merging, or use --force to override (not recommended)",
            "session_summary": validation["summary"]
        }

    # Generate feedback before merge (unless skipped)
    feedback_result = None
    if not skip_feedback:
        feedback_result = generate_feedback(branch=current)
        if not feedback_result.get("success"):
            # Feedback file may already exist, that's OK
            if "already exists" not in feedback_result.get("error", ""):
                return {
                    "success": False,
                    "error": f"Failed to generate feedback: {feedback_result.get('error')}",
                    "hint": "Use --skip-feedback to bypass"
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
        "into": "main",
        "feedback": feedback_result if feedback_result else None,
        "validation": validation,
        "forced": force and not validation["compliant"],
        "reminder": "Run /context to capture token usage in feedback file" if feedback_result and feedback_result.get("success") else None
    }

    # Push to remote
    if push:
        push_result = run_git("push", "origin", "main")
        if push_result.returncode != 0:
            result["push_error"] = push_result.stderr.strip()
            result["message"] = f"Merged {current} to main but push failed"
        else:
            result["pushed"] = True
            result["message"] = f"Merged {current} to main and pushed to origin"

    # Delete feature branch
    if delete_branch:
        run_git("branch", "-d", current)
        run_git("push", "origin", "--delete", current)
        result["deleted_branch"] = current

    return result

def main():
    delete = "--keep" not in sys.argv
    push = "--no-push" not in sys.argv
    skip_feedback = "--skip-feedback" in sys.argv
    force = "--force" in sys.argv

    result = merge_to_main(delete_branch=delete, push=push, skip_feedback=skip_feedback, force=force)
    print(json.dumps(result, indent=2))

    # Exit with error code if merge failed
    if not result.get("success"):
        sys.exit(1)

if __name__ == "__main__":
    main()
