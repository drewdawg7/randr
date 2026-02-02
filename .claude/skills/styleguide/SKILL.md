---
name: styleguide
description: Use when code has been changed in order to ensure the code aligns with best practices. This should run before any commits.
---
## Notes
- Items that appear in this list are not optional, they are fully required.

## Things To Not Do (MANDATORY)
1. **Unwrap**: Unwrap should not be used anywhere outside of test code.
2. **Magic Numbers**: Unless it is very clear what a number refers to, put it in a constant/variable/enum/etc
3. **Hardcoded Values**: Wherever possible, derive values instead of hardcoding them.
4. **Race Conditions**: Make sure new race conditions are not introduced. If things must run in a certain order than run them in that order. Do not rely upon async code where you need to introduce guard clauses.
5. **Comments**: Comments are unnecessary and should not be used outside of test code, unless they explain a complex piece of code or code where the meaning / reason is not immediately obvious.
6. **Couple UI and Game Logic**: Game and UI logic must be kept totally separate. UI should focus almost entirely on rendering.

## Things To Do (MANDATORY)
1. **Declarative Style**: Wherever possible favor a declarative style over an imperative one.
2. **Abstract**: If it is possible abstract code to make it more readable, modular, and composable.
3. **Pure Functions**: Functions should be kept pure with minimal side effects wherever possible.
4. **ECS**: Use an ECS wherever possible. Do not use global state outside of the ECS system.
5. **UPDATE OUT OF SCOPE**: If you notice a styleguide issue, even if its not in scope of your ticket, fix it regardless. Commit the changes so they are easy to find.

