# Plan: Fix — safe_name() Silently Truncates Unicode — Validate Output

## Phase 1: Fix safe_name()

- [ ] Task: Update `safe_name()` to validate its output
    - [ ] After stripping unsafe characters, check for empty result → return `Err`
    - [ ] Check if result starts with non-alphanumeric → prepend `"dev-"` with notice
    - [ ] `cargo build` must pass
    - [ ] Commit: `fix(devcontainer): safe_name() validates non-empty and valid-start output`

## Phase 2: Tests

- [ ] Task: Unit tests for `safe_name()` edge cases
    - [ ] All-ASCII valid name → returned as-is (after lowercasing if applicable)
    - [ ] Name with spaces → spaces replaced with `-`
    - [ ] All-Unicode name → `Err` with helpful message
    - [ ] Name starting with `-` → prepended with `"dev"`
    - [ ] Name starting with digit → valid (Docker allows it)
    - [ ] Mixed ASCII+Unicode → Unicode stripped, ASCII retained
    - [ ] `cargo test` must pass
    - [ ] Commit: `test(devcontainer): safe_name() Unicode and edge case coverage`

## Phase 3: Quality Gate

- [ ] Task: Full quality gate
    - [ ] `cargo test`
    - [ ] `cargo clippy --all-targets -- -D warnings`
    - [ ] `cargo fmt --check`
