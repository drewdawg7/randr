#!/usr/bin/env python3
"""Integration tests for Claude Code workflow verification (Issue #88/#89)."""
import json
import os
import subprocess
import sys
from pathlib import Path

# ANSI colors
GREEN = "\033[92m"
RED = "\033[91m"
YELLOW = "\033[93m"
RESET = "\033[0m"

CLAUDE_DIR = Path(__file__).parent.parent.parent
PROJECT_ROOT = CLAUDE_DIR.parent

class TestResults:
    def __init__(self):
        self.passed = 0
        self.failed = 0
        self.warnings = 0
        self.results = []

    def add(self, name, passed, message=""):
        status = "PASS" if passed else "FAIL"
        self.results.append({"test": name, "status": status, "message": message})
        if passed:
            self.passed += 1
        else:
            self.failed += 1

    def add_warning(self, name, message):
        self.results.append({"test": name, "status": "WARN", "message": message})
        self.warnings += 1

    def to_json(self):
        return {
            "success": self.failed == 0,
            "passed": self.passed,
            "failed": self.failed,
            "warnings": self.warnings,
            "results": self.results
        }

def test_hook(hook_path, test_input, expected_decision):
    """Test a hook with given input."""
    try:
        result = subprocess.run(
            ["python3", hook_path],
            input=json.dumps(test_input),
            capture_output=True,
            text=True,
            timeout=5
        )
        output = json.loads(result.stdout)
        return output.get("decision") == expected_decision, output
    except Exception as e:
        return False, {"error": str(e)}

def test_script_json_output(script_path, args=None):
    """Test that a script outputs valid JSON."""
    cmd = ["python3", script_path]
    if args:
        cmd.extend(args)
    try:
        result = subprocess.run(cmd, capture_output=True, text=True, timeout=30)
        output = json.loads(result.stdout)
        return True, output
    except json.JSONDecodeError as e:
        return False, {"error": f"Invalid JSON: {e}", "output": result.stdout[:500]}
    except Exception as e:
        return False, {"error": str(e)}

def run_tests():
    results = TestResults()

    print(f"\n{'='*60}")
    print("Claude Code Workflow Integration Tests")
    print(f"{'='*60}\n")

    # ==================== STRUCTURE TESTS ====================
    print(f"{YELLOW}[Structure Tests]{RESET}")

    # Test 1: CLAUDE.md under 100 lines
    claude_md = CLAUDE_DIR / "CLAUDE.md"
    if claude_md.exists():
        line_count = len(claude_md.read_text().splitlines())
        passed = line_count < 100
        results.add(
            "CLAUDE.md under 100 lines",
            passed,
            f"{line_count} lines" + ("" if passed else " - EXCEEDS LIMIT")
        )
        print(f"  {'✓' if passed else '✗'} CLAUDE.md: {line_count} lines")
    else:
        results.add("CLAUDE.md exists", False, "File not found")
        print(f"  ✗ CLAUDE.md: not found")

    # Test 2: Required directories exist
    required_dirs = ["hooks", "scripts", "agents", "skills", "docs"]
    for dir_name in required_dirs:
        dir_path = CLAUDE_DIR / dir_name
        exists = dir_path.is_dir()
        results.add(f"Directory: {dir_name}/", exists)
        print(f"  {'✓' if exists else '✗'} {dir_name}/")

    # Test 3: Required hooks exist
    print(f"\n{YELLOW}[Hook Files]{RESET}")
    required_hooks = ["enforce_lsp.py", "enforce_scripts.py", "check_branch.py", "auto_check.py"]
    for hook in required_hooks:
        hook_path = CLAUDE_DIR / "hooks" / hook
        exists = hook_path.exists()
        results.add(f"Hook: {hook}", exists)
        print(f"  {'✓' if exists else '✗'} {hook}")

    # Test 4: Required agents exist
    print(f"\n{YELLOW}[Agent Files]{RESET}")
    required_agents = ["code-change.md", "coder.md", "reviewer.md", "test-writer.md"]
    for agent in required_agents:
        agent_path = CLAUDE_DIR / "agents" / agent
        exists = agent_path.exists()
        results.add(f"Agent: {agent}", exists)
        print(f"  {'✓' if exists else '✗'} {agent}")

    # Test 5: Required skills exist
    print(f"\n{YELLOW}[Skill Directories]{RESET}")
    required_skills = ["git-workflow", "code-nav", "rust-patterns", "testing"]
    for skill in required_skills:
        skill_path = CLAUDE_DIR / "skills" / skill
        exists = skill_path.is_dir()
        results.add(f"Skill: {skill}/", exists)
        print(f"  {'✓' if exists else '✗'} {skill}/")

    # ==================== HOOK ENFORCEMENT TESTS ====================
    print(f"\n{YELLOW}[Hook Enforcement Tests]{RESET}")

    # Test: enforce_lsp.py blocks Grep on .rs files
    hook_path = CLAUDE_DIR / "hooks" / "enforce_lsp.py"
    if hook_path.exists():
        test_input = {
            "tool_name": "Grep",
            "tool_input": {"pattern": "fn main", "glob": "*.rs"}
        }
        passed, output = test_hook(hook_path, test_input, "block")
        results.add("LSP hook blocks Grep on *.rs", passed, str(output.get("decision", output)))
        print(f"  {'✓' if passed else '✗'} Grep on *.rs → {'blocked' if passed else 'NOT blocked'}")

        # Should allow Grep on .md files
        test_input = {
            "tool_name": "Grep",
            "tool_input": {"pattern": "README", "glob": "*.md"}
        }
        passed, output = test_hook(hook_path, test_input, "allow")
        results.add("LSP hook allows Grep on *.md", passed, str(output.get("decision", output)))
        print(f"  {'✓' if passed else '✗'} Grep on *.md → {'allowed' if passed else 'NOT allowed'}")

    # Test: enforce_scripts.py blocks raw gh commands
    hook_path = CLAUDE_DIR / "hooks" / "enforce_scripts.py"
    if hook_path.exists():
        test_input = {
            "tool_name": "Bash",
            "tool_input": {"command": "gh issue list"}
        }
        passed, output = test_hook(hook_path, test_input, "block")
        results.add("Scripts hook blocks gh commands", passed, str(output.get("decision", output)))
        print(f"  {'✓' if passed else '✗'} gh issue list → {'blocked' if passed else 'NOT blocked'}")

        # Should allow non-gh commands
        test_input = {
            "tool_name": "Bash",
            "tool_input": {"command": "ls -la"}
        }
        passed, output = test_hook(hook_path, test_input, "allow")
        results.add("Scripts hook allows ls command", passed, str(output.get("decision", output)))
        print(f"  {'✓' if passed else '✗'} ls -la → {'allowed' if passed else 'NOT allowed'}")

    # ==================== SCRIPT JSON OUTPUT TESTS ====================
    print(f"\n{YELLOW}[Script JSON Output Tests]{RESET}")

    scripts_to_test = [
        ("scripts/check/cargo_check.py", None, "cargo_check.py"),
        ("scripts/git/branch.py", ["--help"], "branch.py --help"),
        ("scripts/issue/list.py", None, "list.py"),
    ]

    for script_rel, args, name in scripts_to_test:
        script_path = CLAUDE_DIR / script_rel
        if script_path.exists():
            passed, output = test_script_json_output(script_path, args)
            results.add(f"JSON output: {name}", passed)
            print(f"  {'✓' if passed else '✗'} {name}: {'valid JSON' if passed else 'INVALID'}")
        else:
            results.add_warning(f"Script: {name}", "not found")
            print(f"  ? {name}: not found")

    # ==================== SUMMARY ====================
    print(f"\n{'='*60}")
    print(f"Results: {GREEN}{results.passed} passed{RESET}, ", end="")
    if results.failed > 0:
        print(f"{RED}{results.failed} failed{RESET}, ", end="")
    else:
        print(f"{results.failed} failed, ", end="")
    print(f"{results.warnings} warnings")
    print(f"{'='*60}\n")

    return results

def main():
    os.chdir(PROJECT_ROOT)
    results = run_tests()
    print(json.dumps(results.to_json(), indent=2))
    sys.exit(0 if results.failed == 0 else 1)

if __name__ == "__main__":
    main()
