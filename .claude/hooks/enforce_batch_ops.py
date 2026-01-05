#!/usr/bin/env python3
"""
PostToolUse hook: Advisory for batch operations.

Provides prominent reminders when edit patterns suggest ast-grep would be more efficient.
This is advisory (non-blocking) per workflow preference.

Triggers at:
- 5+ edits to .rs files (suggest ast-grep)
- 10+ total edits (general batch reminder)
"""
import json
import sys
from pathlib import Path

# Import session state from same directory
sys.path.insert(0, str(Path(__file__).parent))
from session_state import get_state


def main():
    try:
        hook_input = json.load(sys.stdin)
    except json.JSONDecodeError:
        return

    tool_name = hook_input.get("tool_name", "")

    # Only check after Edit operations
    if tool_name != "Edit":
        return

    # Get session state
    state = get_state()
    edit_patterns = state.get("edit_patterns", {})
    rs_edit_count = edit_patterns.get(".rs", 0)
    edit_count = state.get("edit_count", 0)
    files_edited = state.get("files_edited", [])

    # Determine if we should show advisory
    message = None

    if rs_edit_count == 5:
        message = """⚡ Batch Operation Recommended

You've made 5 edits to .rs files. Consider using ast-grep for remaining similar changes:

  # Find similar patterns
  ast-grep --pattern 'YOUR_PATTERN' src/

  # Batch replace
  ast-grep --pattern 'old_code' --rewrite 'new_code' src/

See .claude/docs/batch-operations.md for common patterns.

This is advisory - continue with manual edits if appropriate."""

    elif rs_edit_count == 10:
        message = f"""⚠️ High Edit Count Warning

{rs_edit_count} edits to .rs files across {len(files_edited)} files.
Manual edit ratio is likely exceeding the <20% target.

Strongly recommend switching to ast-grep for remaining changes.
See .claude/docs/batch-operations.md"""

    elif edit_count == 10 and rs_edit_count < 5:
        message = f"""📊 Edit Count Check

{edit_count} total edits this session.
If you're making similar changes, consider batch operations:
  - ast-grep for code patterns
  - Parallel Read for file scanning

See .claude/docs/batch-operations.md"""

    if message:
        # Return advisory message (non-blocking)
        print(json.dumps({
            "decision": "allow",
            "message": message
        }))
    # No output = silent pass


if __name__ == "__main__":
    main()
