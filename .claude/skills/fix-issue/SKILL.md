---
name: fix-issue
description: Identifies researched issues in the github repo and fixes them based on releative severity
---

**IMPORTANT**: Use subagents to parallelize fixing issues.

## Overview
1. Pull down a list of issues that have the label 'researched' and are not marked as complete
2. Pick an issue based on percieved severity
3. Given the context from the ticket, work to resolve the issue. Use updating-code.
4. Once the issue is resolved, add the label 'fix-attempted'

