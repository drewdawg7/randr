---
name: research-issues
description: Pull down issues from the github 
---

## Overview
This skill is aimed to help provide context to issues in the github repo. This skill should always be run async in the background. No user intervention.

## Steps
1. Pull down the current github issues with the label 'fresh'
2. Pick an issue to research
3. Look through the codebase to provide more context to the issue. Add file names, function names, struct names, etc. Attempt to explain why the issue is happening
4. Once context has been added to the issue, remove the 'fresh' label and add the 'researched' label
5. Do not ask for user intervention.
