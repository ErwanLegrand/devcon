# Plan: Security — Prevent Path Traversal in Workspace and Dockerfile Paths

## Phase 1: Path Validation Helper

- [x] Task: Create `src/devcontainers/paths.rs`
    - [x] Implement `fn validate_within_root(root: &Path, candidate: &Path) -> io::Result<PathBuf>`
    - [x] Normalise candidate (resolve `..` components without requiring path to exist)
    - [x] Return error if normalised path does not start with `root`
    - [x] `cargo build` must pass

## Phase 2: Wire Into Config Loading

- [x] Task: Validate `build.dockerfile` and `build.context`
    - [x] In provider build methods, call `validate_within_root` for dockerfile and context paths
    - [x] Abort with descriptive error if traversal detected
    - [x] `cargo build` must pass
- [x] Task: Validate `mounts[].source` for relative paths
    - [x] Relative `mounts` source paths must be validated within workspace root
    - [x] Absolute paths pass through
    - [x] `cargo build` must pass
    - [x] Commit: `fix(security): validate dockerfile/context/mounts paths against traversal`

## Phase 3: Tests

- [x] Task: Unit tests for `validate_within_root`
    - [x] Normal relative path within root: OK
    - [x] `../sibling` escaping root: error
    - [x] `../../etc/passwd`: error
    - [x] Absolute path inside root: OK
    - [x] Absolute path outside root: error
    - [x] `cargo test` must pass
    - [x] Commit: `test(security): path traversal prevention`

## Phase 4: Quality Gate

- [x] Task: Full quality gate
    - [x] `cargo test`
    - [x] `cargo clippy --all-targets -- -D warnings`
    - [x] `cargo fmt --check`
