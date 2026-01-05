# Scripts Index

All scripts output JSON. Use `python3 .claude/scripts/<path>` to run.

## Git Scripts (`scripts/git/`)

| Script | Purpose | Usage |
|--------|---------|-------|
| `branch.py` | Create/switch branches | `branch.py feat/description` |
| `commit.py` | Conventional commits | `commit.py "type: message"` |
| `merge.py` | Merge to main + cleanup | `merge.py [--keep] [--no-push]` |

## Check Scripts (`scripts/check/`)

| Script | Purpose | Usage |
|--------|---------|-------|
| `cargo_check.py` | Type check with JSON | `cargo_check.py [package]` |
| `run_tests.py` | Run tests with parsing | `run_tests.py [filter] [-p package]` |
| `integration_test.py` | Workflow verification | `integration_test.py` |

## Issue Scripts (`scripts/issue/`)

| Script | Purpose | Usage |
|--------|---------|-------|
| `list.py` | List issues | `list.py [--state open] [--label x]` |
| `view.py` | View issue details | `view.py <number>` |
| `create.py` | Create issue | `create.py "title" [--body x]` |
| `close.py` | Close issue | `close.py <number> [--comment x]` |

## Code Scripts (`scripts/code/`)

| Script | Purpose | Usage |
|--------|---------|-------|
| `find_symbol.py` | Find symbols via ast-grep | `find_symbol.py <type> <name>` |

Symbol types: `function`, `struct`, `impl`, `trait`, `pattern`
