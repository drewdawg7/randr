# Scripts Index

Central Python scripts for GitHub issue management workflows.
All scripts output JSON and use `gh_utils.py` for common operations.

## Quick Task Lookup

### I need to... list/query issues
| Script | Location | Description |
|--------|----------|-------------|
| `list_issues.py` | `issue/` | Fetch issues with state/label filters |
| `issue_selector.py` | `issue/` | List 'researched' issues sorted by priority |
| `fresh_issue_selector.py` | `issue/` | List 'fresh' issues sorted by priority |
| `find_duplicates.py` | `issue/` | Find potential duplicate issues by similarity |

### I need to... get issue details
| Script | Location | Description |
|--------|----------|-------------|
| `issue_context.py` | `issue/` | Full issue context (body, comments, file refs) |

### I need to... modify issue state
| Script | Location | Description |
|--------|----------|-------------|
| `close_duplicate.py` | `issue/` | Mark and close issue as duplicate |
| `option_selector.py` | `issue/` | Process selected option checkbox in issue body |

### I need to... manage labels
| Script | Location | Description |
|--------|----------|-------------|
| `label_manager.py` | `workflow/` | List labels by category, create new labels |

### I need to... create issues
| Script | Location | Description |
|--------|----------|-------------|
| `create_issue.py` | `workflow/` | Create issue with 'fresh' label + domain/priority |

### I need to... work on fixes
| Script | Location | Description |
|--------|----------|-------------|
| `fix_setup.py` | `workflow/` | Create branch `fix/issue-{n}-{slug}`, get context |
| `fix_complete.py` | `workflow/` | Commit, merge to main, cleanup branch |

### I need to... research issues
| Script | Location | Description |
|--------|----------|-------------|
| `research_setup.py` | `workflow/` | Start research (fresh -> under research) |
| `research_complete.py` | `workflow/` | Complete research (-> researched) |

---

## Shared Library: gh_utils.py

Import pattern for scripts in subdirectories:
```python
import sys
from pathlib import Path
sys.path.insert(0, str(Path(__file__).parent.parent))
from gh_utils import run_cmd, get_issue_details
```

### Command Execution
| Function | Description |
|----------|-------------|
| `run_cmd(cmd, check=True)` | Run command, return `(success, output)` |
| `run_gh(args)` | Run `gh` command, exit on failure |

### Issue Fetching
| Function | Description |
|----------|-------------|
| `get_issue_details(issue_number, fields)` | Fetch issue JSON |
| `get_issue_comments(issue_number)` | Fetch comments via jq |
| `get_issue_comments_full(issue_number)` | Fetch full comment JSON |
| `list_issues(label, state, fields, limit)` | List issues with filters |
| `issue_exists(number)` | Check if issue exists, return `(exists, title)` |

### Label Operations
| Function | Description |
|----------|-------------|
| `add_label(issue_number, label, create_if_missing)` | Add label to issue |
| `remove_label(issue_number, label)` | Remove label from issue |
| `ensure_label_exists(label, color, description)` | Create label if missing |
| `get_existing_labels()` | Get all label names as set |
| `get_all_labels()` | Get labels with name/description/color |
| `get_issue_labels(issue_number)` | Get labels for specific issue |

### Text Extraction
| Function | Description |
|----------|-------------|
| `extract_file_references(text)` | Extract `.rs/.py/.md/.toml` file paths |
| `extract_keywords(title, body)` | Extract game-related keywords |

### Priority & Age
| Function | Description |
|----------|-------------|
| `get_priority(labels)` | Get `(rank, name)` from priority labels |
| `calculate_age_days(created_at)` | Days since ISO timestamp |

### Issue Operations
| Function | Description |
|----------|-------------|
| `post_comment(issue_number, comment)` | Post comment on issue |
| `close_issue(issue_number, reason)` | Close issue |
| `update_issue_body(issue_number, body)` | Update issue body |

### Git Operations
| Function | Description |
|----------|-------------|
| `get_current_branch()` | Get current branch name |
| `checkout_main_and_pull()` | Checkout main, pull latest |
| `create_branch(branch_name)` | Create and checkout branch |
| `push_branch(branch)` | Push branch to origin |
| `merge_to_main(branch)` | Merge branch to main, push |
| `delete_branch(branch)` | Delete local and remote branch |

### Text Utilities
| Function | Description |
|----------|-------------|
| `slugify(text, max_length)` | Convert to URL-friendly slug |
| `normalize_text(text)` | Lowercase, remove punctuation |
| `get_keywords_from_text(text)` | Extract meaningful keywords |

---

## Script Details

### issue/list_issues.py
```bash
python3 .claude/scripts/issue/list_issues.py [--state open|closed|all]
```
Fetches issues with optional state filter. Returns truncated body (500 chars).

### issue/issue_selector.py
```bash
python3 .claude/scripts/issue/issue_selector.py
```
Lists 'researched' issues (excluding 'fix-attempted'), sorted by priority then age.

### issue/fresh_issue_selector.py
```bash
python3 .claude/scripts/issue/fresh_issue_selector.py
```
Lists 'fresh' issues, sorted by priority then age.

### issue/issue_context.py
```bash
python3 .claude/scripts/issue/issue_context.py <issue_number>
```
Extracts full context: title, body, comments, labels, file references.

### issue/find_duplicates.py
```bash
python3 .claude/scripts/issue/find_duplicates.py [--threshold 0.4]
```
Compares open issues using title (50%), body (35%), label (15%) similarity.

### issue/close_duplicate.py
```bash
python3 .claude/scripts/issue/close_duplicate.py <duplicate_number> <original_number>
```
Adds 'duplicate' label, posts linking comment, closes as "not planned".

### issue/option_selector.py
```bash
python3 .claude/scripts/issue/option_selector.py <issue_number>
```
Processes checked option in issue body, collapses others into `<details>`.

### workflow/create_issue.py
```bash
python3 .claude/scripts/workflow/create_issue.py --title "Title" --body "Body" \
    [--domain ui] [--priority high] [--labels "extra,labels"]
```
Creates issue with 'fresh' label, optional domain and priority labels.

### workflow/label_manager.py
```bash
python3 .claude/scripts/workflow/label_manager.py --list
python3 .claude/scripts/workflow/label_manager.py --create "name" [--color hex] [--description "desc"]
```
List labels by category or create new labels.

### workflow/fix_setup.py
```bash
python3 .claude/scripts/workflow/fix_setup.py <issue_number>
```
Checks out main, creates `fix/issue-{n}-{slug}` branch, outputs full context.

### workflow/fix_complete.py
```bash
python3 .claude/scripts/workflow/fix_complete.py <issue_number> "<resolution_summary>"
```
Adds 'fix-attempted' label, posts comment, commits, merges to main, deletes branch.

### workflow/research_setup.py
```bash
python3 .claude/scripts/workflow/research_setup.py <issue_number>
```
Transitions fresh -> under research, extracts context hints (domain, files, keywords).

### workflow/research_complete.py
```bash
python3 .claude/scripts/workflow/research_complete.py <issue_number> "<findings_markdown>"
```
Posts research findings, transitions under research -> researched.
