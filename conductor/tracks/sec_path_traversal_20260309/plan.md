# Plan: Security — Prevent Path Traversal in Workspace and Dockerfile Paths

## Phase 1: Path Validation Helper

- [ ] Task: Create `src/devcontainers/paths.rs`
    - [ ] Implement `fn validate_within_root(root: &Path, candidate: &Path) -> io::Result<PathBuf>`
    - [ ] Normalise candidate (resolve `..` components without requiring path to exist)
    - [ ] Return error if normalised path does not start with `root`
    - [ ] `cargo build` must pass

## Phase 2: Wire Into Config Loading

- [ ] Task: Validate `build.dockerfile` and `build.context`
    - [ ] In `Devcontainer::load()` or provider build methods, call `validate_within_root`
          for dockerfile path and context path
    - [ ] Abort with descriptive error if traversal detected
    - [ ] `cargo build` must pass
- [ ] Task: Validate `mounts[].source` for relative paths
    - [ ] Relative `mounts` source paths must be validated within workspace root
    - [ ] Absolute paths emit a one-line info log and pass through
    - [ ] `cargo build` must pass
    - [ ] Commit: `fix(security): validate dockerfile/context/mounts paths against traversal`

## Phase 3: Tests

- [ ] Task: Unit tests for `validate_within_root`
    - [ ] Normal relative path within root: OK
    - [ ] `../sibling` escaping root: error
    - [ ] `../../etc/passwd`: error
    - [ ] Absolute path inside root: OK
    - [ ] Symlink resolving outside root: error (if symlink exists at test time)
    - [ ] `cargo test` must pass
    - [ ] Commit: `test(security): path traversal prevention`

## Phase 4: Quality Gate

- [ ] Task: Full quality gate
    - [ ] `cargo test`
    - [ ] `cargo clippy --all-targets -- -D warnings`
    - [ ] `cargo fmt --check`
