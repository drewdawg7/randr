---
name: log-issue
description: Logs an issue against the github repo to be looked at later.
---

## User Directed
### Steps
1. Ask the user if there is more context they can provide
2. Log an issue into the repo with a clear title and context
3. Once the issue is logged, tell the user the issue-id

## Claude Directed
### Steps
1. If an issue is noticed while updating code, but is not directly relevant to the current change, log an issue against the github repo
2. Provide a clear title and organized context at what the issue is and where it was noticed
3. After finishing a code change, inform the user of any new issues.


## Priority
* Give issues a priority based on percieved severity

## Labels
* Tag issues with relevant labels. If none exist, create one yourself to add. 
* Tag any fresh issues with a label called 'fresh'
* Labels should not just be about the status of the ticket, but areas the bug is relevant to. i.e., 'dungeon', 'ui', etc.
* An issue can have mutliple labels.

## Helper Scripts

Python scripts in `scripts/` directory automate common tasks. All output JSON.

### create_issue.py - Create New Issue
```bash
python .claude/skills/log-issue/scripts/create_issue.py \
    --title "Issue title" \
    --body "Issue description" \
    --domain "ui" \
    --priority "medium"
```
Creates issue with 'fresh' label, domain label, and priority label.

Options:
- `--title` (required): Issue title
- `--body` (required): Issue body/description
- `--domain` (optional): Domain label (ui, combat, store, etc.)
- `--priority` (optional): Priority level (critical, high, medium, low)
- `--labels` (optional): Comma-separated additional labels

### label_manager.py - Manage Labels
```bash
# List all labels by category
python .claude/skills/log-issue/scripts/label_manager.py --list

# Create new label
python .claude/skills/log-issue/scripts/label_manager.py --create "mining" --description "Mining system"
```
