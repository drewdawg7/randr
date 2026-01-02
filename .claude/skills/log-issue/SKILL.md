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
