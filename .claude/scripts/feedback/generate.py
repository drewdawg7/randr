#!/usr/bin/env python3
"""Generate feedback file from session state and template.

Usage:
    python3 generate.py --issue 42
    python3 generate.py --issue 42 --title "Add inventory feature"
    python3 generate.py --issue 42 --type feat --complexity medium

This script:
1. Reads session state from .session_state.json
2. Reads the feedback template
3. Generates a feedback file with metrics filled in
4. Leaves placeholders for manual review
"""
import argparse
import json
import sys
from datetime import datetime
from pathlib import Path

# Paths
SCRIPT_DIR = Path(__file__).parent
CLAUDE_DIR = SCRIPT_DIR.parent.parent
TEMPLATE_PATH = CLAUDE_DIR / "feedback" / "TEMPLATE.md"
FEEDBACK_DIR = CLAUDE_DIR / "feedback"
SESSION_STATE_PATH = CLAUDE_DIR / ".session_state.json"


def load_session_state() -> dict:
    """Load session state if it exists."""
    if SESSION_STATE_PATH.exists():
        try:
            with open(SESSION_STATE_PATH, 'r') as f:
                return json.load(f)
        except (json.JSONDecodeError, IOError):
            pass
    return {}


def load_template() -> str:
    """Load the feedback template."""
    if TEMPLATE_PATH.exists():
        return TEMPLATE_PATH.read_text()
    return get_default_template()


def get_default_template() -> str:
    """Return default template if file not found."""
    return """# Issue #[NUMBER]: [TITLE]

## Summary
- **Type:** [feat/fix/refactor]
- **Complexity:** [Low/Medium/High] ([X] files modified)
- **Outcome:** [Completed/Failed/Partial]

## Goal Metrics (see workflow-goals.md)

### P1: Stability
| Metric | Value | Target |
|--------|-------|--------|
| Reverts needed | X | 0 |
| Compilation errors from removals | X | 0 |
| findReferences before removal | Yes/No | Yes |

### P2: Token Usage
| Metric | Value | Target |
|--------|-------|--------|
| Manual edit count | X | - |
| ast-grep operations | X | - |
| Manual edit ratio | X% | <20% |
| Similar edits without ast-grep | X | 0 |

### P3: Speed
| Metric | Value |
|--------|-------|
| LSP operations | X |
| Grep on .rs (blocked) | X |
| Parallel read batches | X |

## Tool Stats
| Metric | Count |
|--------|-------|
| Bash invocations | X |
| Read operations | X |
| Edit operations | X |
| LSP operations | X |
| Grep attempts blocked | X |

## Workflow Compliance
- [ ] LSP used for Rust navigation (not grep)
- [ ] Batch operations used where applicable (>5 similar changes)
- [ ] `findReferences` run before any code removal
- [ ] Agent delegation followed (orchestrator doesn't write code)

## Agent Usage
| Agent | Invocations |
|-------|-------------|
| Coder (Opus) | X |
| Reviewer (Sonnet) | X |
| Test-writer (Sonnet) | X |
| Direct edits (should be 0) | X |

## Quality Metrics
| Metric | Start | End |
|--------|-------|-----|
| Compilation warnings | X | 0 |
| Tests passing | X | X |
| Reverts needed | - | X (target: 0) |

## Lessons Learned

### What Went Well
- [List successes]

### What Could Improve
- [List areas for improvement]

### Specific Recommendations
- [Actionable next steps]
"""


def fill_template(template: str, issue: int, title: str, issue_type: str,
                  complexity: str, state: dict) -> str:
    """Fill in template with available data."""
    content = template

    # Basic info
    content = content.replace("[NUMBER]", str(issue))
    content = content.replace("[TITLE]", title)
    content = content.replace("[feat/fix/refactor]", issue_type)

    # Complexity
    files_edited = state.get("files_edited", [])
    file_count = len(files_edited) if isinstance(files_edited, list) else 0
    content = content.replace("[Low/Medium/High]", complexity)
    content = content.replace("([X] files modified)", f"({file_count} files modified)")

    # Session state metrics
    edit_count = state.get("edit_count", 0)
    lsp_calls = state.get("lsp_calls", 0)
    ast_grep_calls = state.get("ast_grep_calls", 0)
    lines_edited = state.get("lines_edited", 0)
    delegation_used = state.get("delegation_used", False)
    edit_patterns = state.get("edit_patterns", {})
    operations = state.get("operations", {})

    # Calculate manual edit ratio
    total_ops = edit_count + ast_grep_calls
    manual_ratio = (edit_count / total_ops * 100) if total_ops > 0 else 100

    # Fill in P2: Token Usage
    content = content.replace("| Manual edit count | X |", f"| Manual edit count | {edit_count} |")
    content = content.replace("| ast-grep operations | X |", f"| ast-grep operations | {ast_grep_calls} |")
    content = content.replace("| Manual edit ratio | X% |", f"| Manual edit ratio | {manual_ratio:.0f}% |")

    # Fill in P3: Speed
    content = content.replace("| LSP operations | X |", f"| LSP operations | {lsp_calls} |", 1)

    # Fill in Tool Stats
    content = content.replace("| Edit operations | X |", f"| Edit operations | {edit_count} |")
    content = content.replace("| LSP operations | X |", f"| LSP operations | {lsp_calls} |")
    read_ops = operations.get("read", 0)
    content = content.replace("| Read operations | X |", f"| Read operations | {read_ops} |")

    # Agent usage - direct edits
    if delegation_used:
        content = content.replace("| Direct edits (should be 0) | X |", "| Direct edits (should be 0) | 0 |")
    else:
        content = content.replace("| Direct edits (should be 0) | X |", f"| Direct edits (should be 0) | {edit_count} |")

    return content


def generate_feedback(issue: int, title: str = "", issue_type: str = "feat",
                      complexity: str = "Medium") -> dict:
    """Generate feedback file for an issue."""
    # Load data
    state = load_session_state()
    template = load_template()

    # Default title if not provided
    if not title:
        title = f"Issue #{issue}"

    # Fill template
    content = fill_template(template, issue, title, issue_type, complexity, state)

    # Write feedback file
    output_path = FEEDBACK_DIR / f"issue-{issue}-stats.md"

    # Check if file already exists
    if output_path.exists():
        return {
            "success": False,
            "error": f"Feedback file already exists: {output_path}",
            "hint": "Delete or rename existing file to regenerate"
        }

    output_path.write_text(content)

    return {
        "success": True,
        "file": str(output_path),
        "message": f"Generated feedback for issue #{issue}",
        "session_metrics": {
            "edit_count": state.get("edit_count", 0),
            "lsp_calls": state.get("lsp_calls", 0),
            "ast_grep_calls": state.get("ast_grep_calls", 0),
            "files_edited": len(state.get("files_edited", []))
        },
        "next_steps": [
            "Review generated file and fill in remaining placeholders",
            "Add lessons learned based on session experience",
            "Update compliance checkboxes"
        ]
    }


def parse_args():
    """Parse command line arguments."""
    parser = argparse.ArgumentParser(
        description="Generate feedback file from session state",
        formatter_class=argparse.RawDescriptionHelpFormatter
    )
    parser.add_argument("--issue", "-i", type=int, required=True,
                        help="Issue number")
    parser.add_argument("--title", "-t", type=str, default="",
                        help="Issue title (optional)")
    parser.add_argument("--type", type=str, default="feat",
                        choices=["feat", "fix", "refactor", "docs", "test", "chore"],
                        help="Issue type (default: feat)")
    parser.add_argument("--complexity", "-c", type=str, default="Medium",
                        choices=["Low", "Medium", "High"],
                        help="Complexity level (default: Medium)")
    return parser.parse_args()


def main():
    args = parse_args()
    result = generate_feedback(
        issue=args.issue,
        title=args.title,
        issue_type=args.type,
        complexity=args.complexity
    )
    print(json.dumps(result, indent=2))


if __name__ == "__main__":
    main()
