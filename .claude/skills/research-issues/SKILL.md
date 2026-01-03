---
name: research-issues
description: Pull down issues from the github 
---

**IMPORTANT**: DO NOT ASK FOR USER INTERVENTION.
**IMPORTANT**: DO NOT PULL DOWN ISSUES NOT MARKED WITH 'fresh'
**IMPORTANT**: **Use Rust LSP (rust-analyzer) instead of grep for navigating Rust code.** LSP provides semantic understanding - use `goToDefinition`, `findReferences`, `goToImplementation`, `hover`, and `workspaceSymbol` for accurate code navigation. Only use grep for documentation files or when LSP is unavailable.

## Overview
This skill is aimed to help provide context to issues in the github repo. This skill should always be run async in the background. No user intervention.

## Steps
1. Pull down the current github issues with the label 'fresh'
2. Pick an issue to research
3. Remove the 'fresh' label and add a new 'under research' label
4. Look through the codebase to provide more context to the issue. Add file names, function names, struct names, etc. Attempt to explain why the issue is happening
5. Once context has been added to the issue, remove the 'fresh' label and add the 'researched' label
6. Do not ask for user intervention.

## Helper Scripts

Python scripts in `scripts/` directory automate common tasks. All output JSON.

### fresh_issue_selector.py - List Fresh Issues
```bash
python .claude/skills/research-issues/scripts/fresh_issue_selector.py
```
Lists all 'fresh' issues sorted by priority (critical > high > medium > low) then age (oldest first).

### research_setup.py - Start Research
```bash
python .claude/skills/research-issues/scripts/research_setup.py <issue_number>
```
Transitions labels (fresh → under research), outputs full issue context including:
- Issue title, body, comments
- Domain hints from labels
- Keywords extracted from text
- File references mentioned

### research_complete.py - Complete Research
```bash
python .claude/skills/research-issues/scripts/research_complete.py <issue_number> "<findings_markdown>"
```
Posts formatted findings comment, transitions labels (under research → researched).
