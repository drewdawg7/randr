---
name: updating-code
description: Guidance for making any code changes. This provides direction on git workflow, how to navigate the code base, and more.
---

## Workflow
1. **Branch**: Create a new github branch with a descriptive name.
2. **Analyze**: Analyze the code base using ast-grep and rust lsp. There is also an agent called rust-codebase-researcher skilled at doing this.
3. **Ask**: If there is any ambiguity, ask the user questions for clarification. This step can be repeated as many times as necessary.
4. **Compare**: Compare your plan to the code of similar functionalities in the codebase, if they exist.
5. **Make Changes**: Execute your plan
6. **Test**: Run the tests only relevant to code you've changed.
7. **Clean-Up**: Clean up any compiler warnings that relate to your changes
8. **Merge**: Commit, Merge, and Push your changes. No PR is necessary.
9. **Close**: If working on a github issue, close it out.
