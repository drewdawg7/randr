#!/usr/bin/env python3
"""
Session state tracking for Claude Code hooks.

Provides shared state that persists across tool invocations within a session.
State is stored in a JSON file and automatically expires after inactivity.

Usage:
    from session_state import SessionState

    state = SessionState()
    state.increment("edit_count")
    state.add_to_set("files_edited", "src/main.rs")
    state.record_symbol_check("MyStruct")

    count = state.get("edit_count", 0)
    symbols = state.get_checked_symbols()
"""
import json
import os
import time
from pathlib import Path
from typing import Any, Optional, Set

# State file location - in project .claude directory
STATE_FILE = Path(__file__).parent.parent / ".session_state.json"

# Session timeout in seconds (30 minutes of inactivity = new session)
SESSION_TIMEOUT = 1800


class SessionState:
    """Manages session state for hooks."""

    def __init__(self):
        self._state = self._load_or_create()

    def _load_or_create(self) -> dict:
        """Load existing state or create new session."""
        if STATE_FILE.exists():
            try:
                with open(STATE_FILE, 'r') as f:
                    state = json.load(f)

                # Check if session expired
                last_activity = state.get("_last_activity", 0)
                if time.time() - last_activity > SESSION_TIMEOUT:
                    return self._new_session()

                return state
            except (json.JSONDecodeError, IOError):
                return self._new_session()
        return self._new_session()

    def _new_session(self) -> dict:
        """Create a fresh session state."""
        return {
            "_session_start": time.time(),
            "_last_activity": time.time(),
            "edit_count": 0,
            "files_edited": [],
            "lsp_calls": 0,
            "ast_grep_calls": 0,
            "symbols_checked": [],
            "edit_patterns": {},
            "lines_edited": 0,
            "delegation_used": False,
            "operations": {
                "edit": 0,
                "read": 0,
                "search": 0,
                "lsp": 0,
                "bash": 0
            },
            # New tracking fields
            "bash_calls": 0,
            "grep_blocked": 0,
            "agent_delegations": {
                "coder": 0,
                "reviewer": 0,
                "test-writer": 0,
                "explore": 0,
                "other": 0
            },
            "removals_attempted": 0,
            "removals_with_check": 0,
            "compilation_errors": 0,
            "tests_run": False,
            "tests_passed": None,
            "reverts_needed": 0
        }

    def _save(self):
        """Persist state to file."""
        self._state["_last_activity"] = time.time()
        try:
            with open(STATE_FILE, 'w') as f:
                json.dump(self._state, f, indent=2)
        except IOError:
            pass  # Fail silently - state is best-effort

    def get(self, key: str, default: Any = None) -> Any:
        """Get a state value."""
        return self._state.get(key, default)

    def set(self, key: str, value: Any):
        """Set a state value and persist."""
        self._state[key] = value
        self._save()

    def increment(self, key: str, amount: int = 1) -> int:
        """Increment a counter and return new value."""
        current = self._state.get(key, 0)
        new_value = current + amount
        self._state[key] = new_value
        self._save()
        return new_value

    def add_to_set(self, key: str, value: str):
        """Add a value to a list (used as set - no duplicates)."""
        items = self._state.get(key, [])
        if value not in items:
            items.append(value)
            self._state[key] = items
            self._save()

    def record_symbol_check(self, symbol: str):
        """Record that findReferences was called on a symbol."""
        self.add_to_set("symbols_checked", symbol)
        self.increment("lsp_calls")
        self._increment_operation("lsp")

    def was_symbol_checked(self, symbol: str) -> bool:
        """Check if findReferences was called on a symbol."""
        return symbol in self._state.get("symbols_checked", [])

    def get_checked_symbols(self) -> list:
        """Get all symbols that have been checked with findReferences."""
        return self._state.get("symbols_checked", [])

    def record_edit(self, file_path: str, lines_changed: int = 1):
        """Record an edit operation."""
        self.increment("edit_count")
        self.increment("lines_edited", lines_changed)
        self.add_to_set("files_edited", file_path)
        self._increment_operation("edit")

        # Track edit patterns by file extension
        ext = Path(file_path).suffix
        patterns = self._state.get("edit_patterns", {})
        patterns[ext] = patterns.get(ext, 0) + 1
        self._state["edit_patterns"] = patterns
        self._save()

    def record_ast_grep(self):
        """Record an ast-grep operation."""
        self.increment("ast_grep_calls")

    def record_delegation(self, agent_type: str = "other"):
        """Record that agent delegation was used."""
        self.set("delegation_used", True)
        delegations = self._state.get("agent_delegations", {})
        agent_key = agent_type.lower()
        if agent_key not in delegations:
            agent_key = "other"
        delegations[agent_key] = delegations.get(agent_key, 0) + 1
        self._state["agent_delegations"] = delegations
        self._save()

    def record_bash_call(self):
        """Record a Bash tool invocation."""
        self.increment("bash_calls")
        self._increment_operation("bash")

    def record_grep_blocked(self):
        """Record a blocked grep attempt on Rust files."""
        self.increment("grep_blocked")

    def record_removal_attempt(self, had_check: bool):
        """Record a code removal attempt."""
        self.increment("removals_attempted")
        if had_check:
            self.increment("removals_with_check")

    def record_compilation_error(self):
        """Record a compilation error."""
        self.increment("compilation_errors")

    def record_test_result(self, passed: bool):
        """Record test run result."""
        self.set("tests_run", True)
        self.set("tests_passed", passed)

    def record_revert(self):
        """Record that a revert was needed."""
        self.increment("reverts_needed")

    def _increment_operation(self, op_type: str):
        """Increment operation counter by type."""
        ops = self._state.get("operations", {})
        ops[op_type] = ops.get(op_type, 0) + 1
        self._state["operations"] = ops

    def get_summary(self) -> dict:
        """Get a summary of session state for feedback."""
        removals_attempted = self.get("removals_attempted", 0)
        removals_with_check = self.get("removals_with_check", 0)
        return {
            "edit_count": self.get("edit_count", 0),
            "files_edited": len(self.get("files_edited", [])),
            "lines_edited": self.get("lines_edited", 0),
            "lsp_calls": self.get("lsp_calls", 0),
            "ast_grep_calls": self.get("ast_grep_calls", 0),
            "symbols_checked": len(self.get("symbols_checked", [])),
            "delegation_used": self.get("delegation_used", False),
            "operations": self.get("operations", {}),
            # New fields
            "bash_calls": self.get("bash_calls", 0),
            "grep_blocked": self.get("grep_blocked", 0),
            "agent_delegations": self.get("agent_delegations", {}),
            "removals_attempted": removals_attempted,
            "removals_with_check": removals_with_check,
            "find_references_compliant": removals_attempted == 0 or removals_with_check == removals_attempted,
            "compilation_errors": self.get("compilation_errors", 0),
            "tests_run": self.get("tests_run", False),
            "tests_passed": self.get("tests_passed", None),
            "reverts_needed": self.get("reverts_needed", 0)
        }

    def reset(self):
        """Reset session state (for testing or manual reset)."""
        self._state = self._new_session()
        self._save()


# Convenience functions for simple usage
_state_instance: Optional[SessionState] = None

def get_state() -> SessionState:
    """Get or create the singleton session state instance."""
    global _state_instance
    if _state_instance is None:
        _state_instance = SessionState()
    return _state_instance


if __name__ == "__main__":
    # CLI for testing/debugging
    import sys

    state = get_state()

    if len(sys.argv) > 1:
        cmd = sys.argv[1]
        if cmd == "reset":
            state.reset()
            print(json.dumps({"success": True, "action": "reset"}))
        elif cmd == "summary":
            print(json.dumps(state.get_summary(), indent=2))
        elif cmd == "raw":
            print(json.dumps(state._state, indent=2))
    else:
        print(json.dumps(state.get_summary(), indent=2))
