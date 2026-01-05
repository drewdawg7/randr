#!/usr/bin/env python3
"""
PostToolUse hook: Track edit patterns and suggest batch operations.

Monitors edit counts and suggests ast-grep when patterns are detected:
- After 5+ similar edits (same file type), suggest ast-grep
- After 10+ total manual edits, remind about batch operations

This hook runs AFTER Edit operations to track patterns.
"""
import json
import sys
from pathlib import Path

# Import session state from same directory
sys.path.insert(0, str(Path(__file__).parent))
from session_state import get_state


def count_lines_changed(old_string: str, new_string: str) -> int:
    """Estimate lines changed in an edit."""
    old_lines = old_string.count('\n') + 1
    new_lines = new_string.count('\n') + 1
    return max(old_lines, new_lines)


def get_suggestion_message(edit_count: int, rs_edit_count: int, files_edited: int) -> str:
    """Generate appropriate suggestion based on edit patterns."""
    messages = []

    if rs_edit_count >= 5:
        messages.append(
            f"You've made {rs_edit_count} edits to .rs files in this session. "
            "Consider using ast-grep for batch operations:\n"
            "  python3 .claude/scripts/code/find_symbol.py pattern \"your_pattern\"\n"
            "  ast-grep --pattern 'old' --rewrite 'new' src/\n\n"
            "See .claude/docs/batch-operations.md for examples."
        )

    if edit_count >= 10 and not messages:
        messages.append(
            f"High edit count ({edit_count} edits across {files_edited} files). "
            "Review if ast-grep batch operations could help.\n"
            "See .claude/docs/batch-operations.md"
        )

    return "\n\n".join(messages)


def main():
    try:
        hook_input = json.load(sys.stdin)
    except json.JSONDecodeError:
        return

    tool_name = hook_input.get("tool_name", "")
    tool_input = hook_input.get("tool_input", {})

    # Only track Edit operations
    if tool_name != "Edit":
        return

    file_path = tool_input.get("file_path", "")
    old_string = tool_input.get("old_string", "")
    new_string = tool_input.get("new_string", "")

    # Get session state
    state = get_state()

    # Record this edit
    lines_changed = count_lines_changed(old_string, new_string)
    state.record_edit(file_path, lines_changed)

    # Get current counts
    edit_count = state.get("edit_count", 0)
    files_edited = state.get("files_edited", [])
    edit_patterns = state.get("edit_patterns", {})
    rs_edit_count = edit_patterns.get(".rs", 0)

    # Check if we should suggest batch operations
    # Only suggest at specific thresholds to avoid spam
    suggest_thresholds = [5, 10, 15, 20]

    should_suggest = (
        (rs_edit_count in suggest_thresholds) or
        (edit_count in [10, 20, 30] and rs_edit_count < 5)
    )

    if should_suggest:
        message = get_suggestion_message(edit_count, rs_edit_count, len(files_edited))
        if message:
            # Output as stderr info (not blocking)
            print(f"\n📊 Edit Pattern Alert:\n{message}\n", file=sys.stderr)


if __name__ == "__main__":
    main()
