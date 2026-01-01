---
name: updating-code
description: Outlines the necessary steps to make code changes. All steps must be followed. Use when planning or making code changes.
---

When refactoring or updating code always do the following:

1. **Checkout a new branch**: Checkout a new branch from main for the changes. Make sure it's named decsriptively.
2. **Reference Docs**: Check for any documention in .claude/skills/updating-code that are relevant to the changes you're being asked to make. This needs to be done before checking the codebase.
3. **Activate Skills**: Be sure to also activate any necessary skills. i.e. if you need to make ascii art, use the ascii-art skill.
4. **Ask Questions**: Once you've reviewed relevant documentation, make sure to ask questions to clarify any amibiguity
5. **Make changes**: Proceed with the changes. For any large, logical chunk of changes, add a commit to easily rollback if needed.
6. **Test**: Check if a tests.rs file exists in the relevant modules and run them. If a test fails, do not update the test itself unless the failure was due to a structural change. (i.e., a new field is added to a struct and so now the test errors)
7. **Cargo Check**: Run cargo check to verify compilation
8. **User Check**: Ask the user to review the changes and verify they work.
9. **Update Documentation**: In .claude/skills/updating-code create .md files to place documentation. These files should be broken down/named by relevant area, and can be broken down into subdirectories to group related information. In these documents, place information related to the coding/architecture style of that area, justifications for decisions, explanations of that area of code, and any information that would make future changes faster and easier.
10. **Merge**: Once all of the above is done, commit any outstanding changes, push, merge the branch into main, and then delete. Once merged, push the main branch.
11. **Additional Documentation**: Create additional documentation within .claude/skills/updating-code that will allow you to use this skill more efficiently if needed.


## Documentation
* The main purpose of the documentation is to make it so that you can more easily find the areas you need to update for future requests, so that you don't have to search through the whole code base, and can instead scan the documents. Therefore, you should be descriptive. Include file names, module names,/ function names, concepts, etc.
* Documentation around UI decisions and patterns is important.
* Documentation should be written such that you can efficiently scan it for information to minimize token waste and time waste.
* If necessary, you may re-organize the documentation within .claude/skills/updating-code, including deleting, renaming, moving, or editing files. 
* If several files are repeating similar things, extract that information into higher level documentation.
* Documentation should only ever include examples from the codebase or closely related.
