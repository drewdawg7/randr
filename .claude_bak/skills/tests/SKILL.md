---
name: tests
description: Guidelines for creating, updating, and using tests.
---

## Guidelines
* File should be named tests.rs
* Do not create a mod inside the file.
* Use #[cfg(test)] to guard imports, functions, etc.
* If a test fails you must not edit to make it pass unless the failure is due to a structural change i.e., a new field was added to a struct or a new param to a function
* Try and test edge cases as well as happy path
* If possible, create tests that mock how a user would take multiple steps in a game.
