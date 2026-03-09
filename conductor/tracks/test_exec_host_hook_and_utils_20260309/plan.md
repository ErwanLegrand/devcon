# Plan: Tests — exec_host_hook, OneOrMany, safe_name, and Utilities

## Phase 1: exec_host_hook Tests

- [ ] Task: Write `exec_host_hook` unit tests
    - [ ] Command `"true"` exits 0 → `Ok(true)`
    - [ ] Command `"false"` exits 1 → `Ok(false)`
    - [ ] Non-existent binary → `Err`
    - [ ] Many form `["echo", "arg with space"]` → arg not split
    - [ ] `cargo test` must pass
    - [ ] Commit: `test(devcontainer): exec_host_hook coverage`

## Phase 2: OneOrMany Edge Cases

- [ ] Task: Extend `OneOrMany` deserialization tests
    - [ ] `null` input → document expected behaviour, test it
    - [ ] Integer input → `Err`
    - [ ] Nested array `[["a"]]` → `Err`
    - [ ] Empty string `""` → `Ok(One(""))`
    - [ ] `cargo test` must pass
    - [ ] Commit: `test(one_or_many): edge case deserialization coverage`

## Phase 3: safe_name and Config::parse Tests

- [ ] Task: Add `safe_name()` tests (coordinate with `fix_safe_name_validation` track)
    - [ ] Valid name, name with special chars, all-Unicode name
    - [ ] `cargo test` must pass
- [ ] Task: Add `Config::parse` error path tests
    - [ ] Invalid JSON5 → `Err`
    - [ ] Missing optional fields → uses defaults
    - [ ] `cargo test` must pass
    - [ ] Commit: `test(config): parse error paths and safe_name edge cases`

## Phase 4: Quality Gate

- [ ] Task: Full quality gate and coverage check
    - [ ] `cargo test`
    - [ ] `cargo llvm-cov --lib` — verify target functions ≥80%
    - [ ] `cargo clippy --all-targets -- -D warnings`
    - [ ] `cargo fmt --check`
