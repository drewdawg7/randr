#!/usr/bin/env python3
"""Generate feedback file from session state and template.

Usage:
    python3 generate.py --issue 42
    python3 generate.py --issue 42 --title "Add inventory feature"
    python3 generate.py --branch feat/add-inventory
    python3 generate.py --branch feat/add-inventory --title "Add inventory feature"

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

# Import analysis modules
from token_usage import get_session_tokens
from session_analysis import analyze_session

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


def fill_template(template: str, identifier: str, title: str, issue_type: str,
                  complexity: str, state: dict, token_usage: dict, analysis: dict, is_branch: bool = False) -> str:
    """Fill in template with available data."""
    content = template

    # Basic info - handle both issue numbers and branch names
    if is_branch:
        # Replace issue-specific header with branch-based header
        content = content.replace("# Issue #[NUMBER]: [TITLE]", f"# Branch: {identifier}")
        content = content.replace("[NUMBER]", identifier)
    else:
        content = content.replace("[NUMBER]", str(identifier))
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

    # New tracked fields
    bash_calls = state.get("bash_calls", 0)
    grep_blocked = state.get("grep_blocked", 0)
    agent_delegations = state.get("agent_delegations", {})
    reverts_needed = state.get("reverts_needed", 0)
    compilation_errors = state.get("compilation_errors", 0)
    removals_attempted = state.get("removals_attempted", 0)
    removals_with_check = state.get("removals_with_check", 0)
    tests_run = state.get("tests_run", False)
    tests_passed = state.get("tests_passed", None)

    # Calculate metrics
    total_ops = edit_count + ast_grep_calls
    manual_ratio = (edit_count / total_ops * 100) if total_ops > 0 else 100
    find_refs_compliant = removals_attempted == 0 or removals_with_check == removals_attempted

    # Determine outcome based on metrics
    if reverts_needed > 0 or compilation_errors > 0:
        outcome = "Partial"
    elif tests_run and tests_passed is False:
        outcome = "Failed"
    else:
        outcome = "Completed"
    content = content.replace("[Completed/Failed/Partial]", outcome)

    # === P1: Stability ===
    content = content.replace("| Reverts needed | X |", f"| Reverts needed | {reverts_needed} |")
    content = content.replace("| Compilation errors from removals | X |", f"| Compilation errors from removals | {compilation_errors} |")
    find_refs_status = "Yes" if find_refs_compliant else "No"
    content = content.replace("| findReferences before removal | Yes/No |", f"| findReferences before removal | {find_refs_status} |")

    # === P2: Token & Cost Analysis ===
    # Edit stats
    content = content.replace("| Manual edit count | X |", f"| Manual edit count | {edit_count} |")
    content = content.replace("| ast-grep operations | X |", f"| ast-grep operations | {ast_grep_calls} |")
    content = content.replace("| Manual edit ratio | X% |", f"| Manual edit ratio | {manual_ratio:.0f}% |")

    # Fill in session analysis data
    if analysis.get("success"):
        # Summary metrics
        content = content.replace("| Session duration | DURATION |", f"| Session duration | {analysis.get('duration_formatted', 'N/A')} |")
        content = content.replace("| API calls | API_CALLS |", f"| API calls | {analysis.get('api_calls', 0):,} |")
        content = content.replace("| Estimated cost | $TOTAL_COST |", f"| Estimated cost | ${analysis.get('total_cost', 0):.2f} |")
        content = content.replace("| Avg cost/call | $AVG_COST |", f"| Avg cost/call | ${analysis.get('avg_cost_per_call', 0):.4f} |")

        tokens = analysis.get('tokens', {})
        content = content.replace("| Message tokens | MSG_TOKENS |", f"| Message tokens | {tokens.get('message', 0):,} |")
        content = content.replace("| Cache efficiency | CACHE_RATIO:1 |", f"| Cache efficiency | {analysis.get('cache_efficiency_ratio', 0)}:1 |")

        # Token breakdown
        content = content.replace("| Input | INPUT_TOKENS |", f"| Input | {tokens.get('input', 0):,} |")
        content = content.replace("| Output | OUTPUT_TOKENS |", f"| Output | {tokens.get('output', 0):,} |")
        content = content.replace("| Cache read | CACHE_READ |", f"| Cache read | {tokens.get('cache_read', 0):,} |")
        content = content.replace("| Cache write | CACHE_WRITE |", f"| Cache write | {tokens.get('cache_write', 0):,} |")

        # Top tools by cost
        top_tools = analysis.get('top_tools_by_cost', [])
        for i, tool in enumerate(top_tools[:3], 1):
            content = content.replace(f"| TOOL{i}_NAME | TOOL{i}_CALLS | $TOOL{i}_COST | $TOOL{i}_AVG |",
                f"| {tool['name']} | {tool['calls']} | ${tool['cost']:.2f} | ${tool['avg_cost']:.4f} |")

    # === P3: Speed ===
    content = content.replace("| LSP operations | X |", f"| LSP operations | {lsp_calls} |", 1)
    content = content.replace("| Grep on .rs (blocked) | X |", f"| Grep on .rs (blocked) | {grep_blocked} |")
    # Parallel read batches - tracked in operations if available
    parallel_reads = operations.get("parallel_read", 0)
    content = content.replace("| Parallel read batches | X |", f"| Parallel read batches | {parallel_reads} |")

    # === Tool Stats ===
    content = content.replace("| Bash invocations | X |", f"| Bash invocations | {bash_calls} |")
    read_ops = operations.get("read", 0)
    content = content.replace("| Read operations | X |", f"| Read operations | {read_ops} |")
    content = content.replace("| Edit operations | X |", f"| Edit operations | {edit_count} |")
    content = content.replace("| LSP operations | X |", f"| LSP operations | {lsp_calls} |")
    content = content.replace("| Grep attempts blocked | X |", f"| Grep attempts blocked | {grep_blocked} |")

    # === Workflow Compliance (auto-check based on metrics) ===
    # LSP used for Rust navigation
    lsp_compliant = lsp_calls > 0 or grep_blocked == 0
    if lsp_compliant:
        content = content.replace("- [ ] LSP used for Rust navigation (not grep)", "- [x] LSP used for Rust navigation (not grep)")

    # Batch operations used where applicable
    rs_edits = edit_patterns.get(".rs", 0)
    batch_compliant = rs_edits <= 5 or ast_grep_calls > 0
    if batch_compliant:
        content = content.replace("- [ ] Batch operations used where applicable (>5 similar changes)", "- [x] Batch operations used where applicable (>5 similar changes)")

    # findReferences before removal
    if find_refs_compliant:
        content = content.replace("- [ ] `findReferences` run before any code removal", "- [x] `findReferences` run before any code removal")

    # Agent delegation
    total_delegations = sum(agent_delegations.values()) if agent_delegations else 0
    if delegation_used or total_delegations > 0 or edit_count == 0:
        content = content.replace("- [ ] Agent delegation followed (orchestrator doesn't write code)", "- [x] Agent delegation followed (orchestrator doesn't write code)")

    # === Agent Usage ===
    coder_calls = agent_delegations.get("coder", 0)
    reviewer_calls = agent_delegations.get("reviewer", 0)
    test_writer_calls = agent_delegations.get("test-writer", 0)
    content = content.replace("| Coder (Opus) | X |", f"| Coder (Opus) | {coder_calls} |")
    content = content.replace("| Reviewer (Sonnet) | X |", f"| Reviewer (Sonnet) | {reviewer_calls} |")
    content = content.replace("| Test-writer (Sonnet) | X |", f"| Test-writer (Sonnet) | {test_writer_calls} |")

    # Direct edits
    direct_edits = edit_count if not delegation_used else 0
    content = content.replace("| Direct edits (should be 0) | X |", f"| Direct edits (should be 0) | {direct_edits} |")

    # === Quality Metrics ===
    # Compilation warnings - we track errors, start is unknown
    content = content.replace("| Compilation warnings | X | 0 |", f"| Compilation warnings | - | {compilation_errors} |")

    # Tests passing
    if tests_run:
        test_status = "Yes" if tests_passed else "No"
        content = content.replace("| Tests passing | X | X |", f"| Tests passing | - | {test_status} |")
    else:
        content = content.replace("| Tests passing | X | X |", "| Tests passing | - | Not run |")

    # Reverts
    content = content.replace("| Reverts needed | - | X (target: 0) |", f"| Reverts needed | - | {reverts_needed} (target: 0) |")

    return content


def generate_feedback(issue: int = None, branch: str = None, title: str = "",
                      issue_type: str = "feat", complexity: str = "Medium") -> dict:
    """Generate feedback file for an issue or branch."""
    # Must have either issue or branch
    if not issue and not branch:
        return {
            "success": False,
            "error": "Either --issue or --branch must be provided"
        }

    # Load data
    state = load_session_state()
    template = load_template()
    token_usage = get_session_tokens()
    analysis = analyze_session()

    # Determine identifier and filename
    is_branch = branch is not None
    if is_branch:
        identifier = branch
        # Clean branch name for filename (replace / with -)
        safe_name = branch.replace("/", "-")
        output_path = FEEDBACK_DIR / f"branch-{safe_name}-stats.md"
        if not title:
            title = branch
    else:
        identifier = issue
        output_path = FEEDBACK_DIR / f"issue-{issue}-stats.md"
        if not title:
            title = f"Issue #{issue}"

    # Fill template
    content = fill_template(template, identifier, title, issue_type, complexity, state, token_usage, analysis, is_branch)

    # Check if file already exists
    if output_path.exists():
        return {
            "success": False,
            "error": f"Feedback file already exists: {output_path}",
            "hint": "Delete or rename existing file to regenerate"
        }

    output_path.write_text(content)

    label = f"branch {branch}" if is_branch else f"issue #{issue}"
    return {
        "success": True,
        "file": str(output_path),
        "message": f"Generated feedback for {label}",
        "session_metrics": {
            "edit_count": state.get("edit_count", 0),
            "lsp_calls": state.get("lsp_calls", 0),
            "ast_grep_calls": state.get("ast_grep_calls", 0),
            "files_edited": len(state.get("files_edited", []))
        },
        "cost_analysis": {
            "total_cost": analysis.get("total_cost", 0),
            "api_calls": analysis.get("api_calls", 0),
            "duration": analysis.get("duration_formatted", "N/A"),
            "cache_efficiency": analysis.get("cache_efficiency_ratio", 0),
            "top_tools": analysis.get("top_tools_by_cost", [])[:3]
        } if analysis.get("success") else None,
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
    group = parser.add_mutually_exclusive_group(required=True)
    group.add_argument("--issue", "-i", type=int,
                       help="Issue number")
    group.add_argument("--branch", "-b", type=str,
                       help="Branch name (e.g., feat/add-inventory)")
    parser.add_argument("--title", "-t", type=str, default="",
                        help="Title/description (optional)")
    parser.add_argument("--type", type=str, default="feat",
                        choices=["feat", "fix", "refactor", "docs", "test", "chore"],
                        help="Change type (default: feat)")
    parser.add_argument("--complexity", "-c", type=str, default="Medium",
                        choices=["Low", "Medium", "High"],
                        help="Complexity level (default: Medium)")
    return parser.parse_args()


def main():
    args = parse_args()
    result = generate_feedback(
        issue=args.issue,
        branch=args.branch,
        title=args.title,
        issue_type=args.type,
        complexity=args.complexity
    )
    print(json.dumps(result, indent=2))


if __name__ == "__main__":
    main()
