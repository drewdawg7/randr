#!/usr/bin/env python3
"""
PostToolUse hook: Suggest agent delegation for large changes.

Monitors cumulative code changes and suggests delegation to specialized agents
when thresholds are exceeded:
- >50 lines edited: suggest coder agent delegation
- After feature completion: suggest reviewer agent
- New test files: suggest test-writer agent

This is a soft suggestion, not a hard block, to allow tuning.
"""
import json
import sys
from pathlib import Path

# Import session state from same directory
sys.path.insert(0, str(Path(__file__).parent))
from session_state import get_state

# Thresholds for suggestions (advisory, not blocking)
# See .claude/docs/delegation.md for full guide
LINES_THRESHOLD = 30  # Lowered from 50 based on feedback analysis
FILES_THRESHOLD = 5


def get_delegation_suggestion(state) -> str:
    """Generate delegation suggestion based on session state."""
    lines_edited = state.get("lines_edited", 0)
    files_edited = state.get("files_edited", [])
    delegation_used = state.get("delegation_used", False)

    if delegation_used:
        return ""  # Already using delegation

    messages = []

    # Large changes without delegation
    if lines_edited >= LINES_THRESHOLD:
        messages.append(
            f"Large change detected: {lines_edited} lines across {len(files_edited)} files.\n"
            "Consider delegating to the coder agent for better code quality.\n\n"
            "Agent architecture (see .claude/agents/):\n"
            "  - code-change (Sonnet): Orchestrates workflow\n"
            "  - coder (Opus): Writes code with goal constraints\n"
            "  - reviewer (Sonnet): Reviews for issues\n"
            "  - test-writer (Sonnet): Writes focused tests"
        )

    # Many files touched
    elif len(files_edited) >= FILES_THRESHOLD:
        messages.append(
            f"Multi-file change: {len(files_edited)} files edited.\n"
            "Consider using the reviewer agent before committing.\n"
            "See: .claude/agents/reviewer.md"
        )

    return "\n\n".join(messages)


def main():
    try:
        hook_input = json.load(sys.stdin)
    except json.JSONDecodeError:
        return

    tool_name = hook_input.get("tool_name", "")
    tool_input = hook_input.get("tool_input", {})

    # Track Task tool usage as delegation
    if tool_name == "Task":
        state = get_state()
        prompt = tool_input.get("prompt", "").lower()
        # Check if delegating to a known agent
        if any(agent in prompt for agent in ["coder", "reviewer", "test-writer"]):
            state.record_delegation()
        return

    # Only check after Edit operations
    if tool_name != "Edit":
        return

    file_path = tool_input.get("file_path", "")
    if not file_path.endswith(".rs"):
        return

    # Get session state
    state = get_state()

    # Check thresholds at specific points to avoid spam
    lines_edited = state.get("lines_edited", 0)
    check_thresholds = [30, 60, 100, 150]  # First reminder at 30 lines

    # Only suggest when we first cross a threshold
    for threshold in check_thresholds:
        if lines_edited >= threshold and lines_edited < threshold + 10:
            suggestion = get_delegation_suggestion(state)
            if suggestion:
                print(f"\n🤖 Delegation Suggestion:\n{suggestion}\n", file=sys.stderr)
            break


if __name__ == "__main__":
    main()
