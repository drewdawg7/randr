---
name: fix-issue
description: Identifies researched issues in the github repo and fixes them based on releative severity
---

**IMPORTANT**: You do not need to ask permission to make edits.
**IMPORTANT**: Reference `parallel-dev` skill when handling multiple issues - use git worktrees if issues touch different files.

## Overview
1. Pull down a list of issues that have the label 'researched' and are not marked as complete
2. **Evaluate for parallel work** (see `parallel-dev` skill):
   - If multiple issues touch different files → use git worktrees
   - If issues overlap or single issue → work one at a time
3. Pick issue(s) based on perceived severity
4. **Check for `needs-decision` label** - if present, a solution option must be selected before fixing:
   - Review the options in the issue body
   - Check one option (`- [x]`)
   - Run `option_selector.py` to process the selection (collapses other options)
5. Given the context from the ticket, work to resolve the issue. Use updating-code.
6. Once the issue is resolved, add the label 'fix-attempted' and merge the branch into main
7. Add a follow up to the issue stating how the issue was resolved.

## Helper Scripts

Python scripts in `scripts/` directory automate common tasks. All output JSON.

### issue_selector.py - List & Prioritize Issues
```bash
python3 .claude/skills/fix-issue/scripts/issue_selector.py
```
Lists all 'researched' issues sorted by priority (critical > high > medium > low > age).

### issue_context.py - Get Full Issue Context
```bash
python3 .claude/skills/fix-issue/scripts/issue_context.py <issue_number>
```
Extracts issue body, comments, labels, and file references mentioned.

### fix_setup.py - Setup Branch for Issue
```bash
python3 .claude/skills/fix-issue/scripts/fix_setup.py <issue_number>
```
Creates branch `fix/issue-{number}-{slug}`, outputs full issue context.

### fix_complete.py - Complete Fix & Merge
```bash
python3 .claude/skills/fix-issue/scripts/fix_complete.py <issue_number> "<resolution_summary>"
```
Adds label, posts comment, commits, merges to main, cleans up branch.
