---
name: research-issues
description: Pull down issues from the github
---

**IMPORTANT**: DO NOT ASK FOR USER INTERVENTION.
**IMPORTANT**: DO NOT PULL DOWN ISSUES NOT MARKED WITH 'fresh'
**IMPORTANT**: ALWAYS use helper scripts first (they auto-detect repo). Only explore alternatives if scripts fail.
**IMPORTANT**: **Use Rust LSP (rust-analyzer) instead of grep for navigating Rust code.** LSP provides semantic understanding - use `goToDefinition`, `findReferences`, `goToImplementation`, `hover`, and `workspaceSymbol` for accurate code navigation. Only use grep for documentation files or when LSP is unavailable.

## Overview
This skill is aimed to help provide context to issues in the github repo. This skill should always be run async in the background. No user intervention.

## Steps
1. Pull down the current github issues with the label 'fresh'
2. Pick an issue to research
3. Remove the 'fresh' label and add a new 'under research' label
4. Look through the codebase to provide more context to the issue. Add file names, function names, struct names, etc. Attempt to explain why the issue is happening
5. **If multiple solution options exist**, format them in the issue body as checkboxes and add `needs-decision` label (see Options Format below)
6. Once context has been added to the issue, remove the 'fresh' label and add the 'researched' label
7. Do not ask for user intervention.

## Options Format

When multiple valid solutions exist, add them to the **issue body** (not comments) using this format:

```markdown
## Suggested Options
- [ ] **Option A**: Brief description of first approach
- [ ] **Option B**: Brief description of second approach
```

Then add the `needs-decision` label. The user will check one option, then run `option_selector.py` to process the selection.

## Helper Scripts

Python scripts in `.claude/scripts/` directory automate common tasks. All output JSON.
See [SCRIPTS.index.md](../../scripts/SCRIPTS.index.md) for complete reference.

### fresh_issue_selector.py - List Fresh Issues
```bash
python3 .claude/scripts/issue/fresh_issue_selector.py
```
Lists all 'fresh' issues sorted by priority (critical > high > medium > low) then age (oldest first).

### research_setup.py - Start Research
```bash
python3 .claude/scripts/workflow/research_setup.py <issue_number>
```
Transitions labels (fresh -> under research), outputs full issue context including:
- Issue title, body, comments
- Domain hints from labels
- Keywords extracted from text
- File references mentioned

### research_complete.py - Complete Research
```bash
python3 .claude/scripts/workflow/research_complete.py <issue_number> "<findings_markdown>"
```
Posts formatted findings comment, transitions labels (under research -> researched).

### option_selector.py - Process Selected Option
```bash
python3 .claude/scripts/issue/option_selector.py <issue_number>
```
After user checks an option checkbox in the issue body:
1. Identifies the selected option (checked `- [x]`)
2. Rewrites issue body: selected stays visible, others collapse into `<details>`
3. Removes `needs-decision` label

Output JSON includes: `success`, `selected`, `collapsed`, `label_removed`
