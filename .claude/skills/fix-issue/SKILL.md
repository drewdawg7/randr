---
name: fix-issue
description: Identifies researched issues in the github repo and fixes them based on releative severity
---

**IMPORTANT**: You do not need to ask permission to make edits.
**IMPORTANT**: ONLY WORK ON ONE ISSUE AT A TIME

## Overview
1. Pull down a list of issues that have the label 'researched' and are not marked as complete
2. Pick an issue based on percieved severity
3. Given the context from the ticket, work to resolve the issue. Use updating-code.
4. Once the issue is resolved, add the label 'fix-attempted' and merge the branch into main
5. Add a follow up to the issue stating how the issue was resolved.

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
