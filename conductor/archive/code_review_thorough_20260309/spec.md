# Spec: Thorough Code Review

## Objective

Perform a systematic, file-by-file code review of the entire `devcont` source tree. Produce a
prioritised findings report covering correctness, safety, maintainability, and idiomatic Rust.
This is an analytical track — it produces a report and follow-up track stubs, not code changes.

## Scope

Review every file under `src/`. For each file, check:

### Correctness
- Logic errors, off-by-one conditions, wrong branch taken
- Incorrect command argument ordering (e.g. `docker exec` flag positions)
- Silently ignored errors (dropping `Result`, unused `bool` return values)
- Race conditions or TOCTOU issues

### Safety & Security
- `unwrap()` / `expect()` on user-facing paths (should use `?` or explicit error handling)
- Shell-string construction from untrusted config values (injection risk)
- Paths derived from config not canonicalised
- Temp file creation in world-readable locations
- Sensitive data (SSH socket path, env vars) logged or exposed unintentionally

### Idiomatic Rust
- Unnecessary `.clone()` calls
- Owned `String` parameters where `&str` suffices
- Missing `#[must_use]` on functions returning meaningful `bool`/`Result`
- Use of deprecated or suboptimal stdlib patterns

### Maintainability
- Functions over 50 lines
- Files over 400 lines
- Duplicated logic across providers
- Missing or misleading doc comments on public items
- Magic strings / hardcoded values that should be constants

### API Design
- Public items that should be `pub(crate)` or private
- Struct fields that are `pub` but should use accessor methods
- Trait methods with surprising signatures

## Files to Review

- `src/main.rs`
- `src/lib.rs` (if present)
- `src/settings.rs`
- `src/error.rs`
- `src/devcontainers/mod.rs`
- `src/devcontainers/config.rs`
- `src/devcontainers/one_or_many.rs`
- `src/provider/mod.rs`
- `src/provider/docker.rs`
- `src/provider/podman.rs`
- `src/provider/docker_compose.rs`
- `src/provider/podman_compose.rs`
- `src/provider/utils.rs`
- `tests/integration.rs`

## Deliverable

A markdown report saved to `conductor/tracks/code_review_thorough_20260309/report.md`:

1. **Per-file findings** — each finding with: file, line range, severity
   (Critical/High/Medium/Low/Info), category (from scope above), description, and suggested fix.
2. **Summary table** — counts by severity and category.
3. **Follow-up track stubs** — for each cluster of related findings, a suggested track title
   and one-line description to be created as a conductor track.

## Output Artefact

`conductor/tracks/code_review_thorough_20260309/report.md`
