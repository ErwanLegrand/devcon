# Development Workflow

## Test-Driven Development (TDD)

All feature work follows strict TDD:

1. **RED** — Write a failing test that captures the requirement
2. **GREEN** — Write the minimal implementation to make the test pass
3. **IMPROVE** — Refactor while keeping tests green

Never write implementation code without a corresponding test first.

## Code Coverage

- **Target:** >80% line coverage on all modules
- Measured with `cargo-llvm-cov`
- Coverage is checked in CI and blocks merge if below threshold

## Task Workflow

For each task in a plan:

1. Mark the task `[~]` (in progress) in `plan.md`
2. Write tests first (RED)
3. Implement (GREEN)
4. Refactor (IMPROVE)
5. Run `cargo test` and `cargo clippy -- -D warnings` — both must pass
6. Mark the task `[x]` (complete) in `plan.md`
7. Commit with a conventional commit message
8. Add a Git Note summarizing what was done

## Commit Convention

```
<type>: <short description>

<optional body>
```

Types: `feat`, `fix`, `refactor`, `docs`, `test`, `chore`, `perf`, `ci`

Commit after **every task** (not after phases).

## Git Notes

After each commit, record a task summary:

```bash
git notes add -m "<task summary>"
```

The summary should describe: what was implemented, what tests were added, and any notable decisions.

## Phase Completion Verification and Checkpointing Protocol

At the end of each phase:

1. Run the full test suite: `cargo test`
2. Run coverage: `cargo llvm-cov --summary-only`
3. Run linter: `cargo clippy -- -D warnings`
4. Run formatter check: `cargo fmt --check`
5. Review the phase's tasks — all must be `[x]`
6. **Manual user verification:** The user reviews the phase output before the next phase begins
7. Only proceed to the next phase after explicit user approval

## Quality Gates (CI)

All of the following must pass before a branch can merge:

- `cargo build`
- `cargo test`
- `cargo clippy -- -D warnings`
- `cargo fmt --check`
- `cargo deny check`
- `cargo llvm-cov --summary-only` (>80% coverage)

## Security Checklist (before each commit)

- [ ] No `unwrap()` or `expect()` on user-facing paths
- [ ] No user input interpolated into shell strings
- [ ] All file paths validated and canonicalized
- [ ] No hardcoded secrets or credentials
- [ ] Dependencies audited with `cargo deny`
