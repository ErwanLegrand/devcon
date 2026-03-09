# Thorough Code Review Report
## date: 2026-03-09

---

## Per-File Findings

### `src/main.rs`

#### [Low] `no_cache` flag inverted at call-site
- **Lines**: L36
- **Category**: Correctness
- **Description**: `commands::rebuild::run(dir.as_deref(), !no_cache)` inverts the `no_cache` CLI flag before passing it. The `run` parameter is named `use_cache`, so the inversion is intentional, but the double negation (`no_cache` → `!no_cache` → `use_cache`) is a readability trap. A future developer editing either side of this boundary could easily flip the semantics.
- **Suggestion**: Name the CLI flag `--cache` (defaulting to true), or rename `run`'s parameter to `no_cache: bool` and pass the flag directly without inversion. At minimum, add a comment explaining the inversion.

#### [Info] `Commands` enum is private but `Cli` struct is also private
- **Lines**: L11–L26
- **Category**: API Design
- **Description**: Both `Cli` and `Commands` are private to `main.rs`, which is appropriate for a binary entry point. No issues here; noted for completeness.
- **Suggestion**: No action required.

---

### `src/settings.rs`

#### [Medium] TOCTOU race on config file existence check
- **Lines**: L55–L57
- **Category**: Safety & Security
- **Description**: `if !file.is_file()` followed by `std::fs::read_to_string(&file)` creates a classic check-then-act race. Between the `is_file()` probe and the read, another process could remove or replace the file. In this particular usage the consequence is simply falling back to defaults (benign), but it is still a race.
- **Suggestion**: Remove the `is_file()` guard. Instead, match on the `ErrorKind::NotFound` variant returned by `read_to_string`: if not found, return `Ok(Self::default())`; otherwise propagate.

#### [Low] `pub` visibility on `Settings` and `Provider` fields without protection
- **Lines**: L9–L30
- **Category**: API Design
- **Description**: `Settings::dotfiles` and `Settings::provider` are `pub`. Since `Settings` is constructed only by deserialization and used read-only downstream, making fields `pub(crate)` would be more appropriate and prevent accidental mutation from test code or future binary-crate consumers.
- **Suggestion**: Change `pub dotfiles` and `pub provider` to `pub(crate)` (or provide read-only accessors).

---

### `src/error.rs`

#### [Medium] `Error::Io` wraps `std::io::Error` with `#[from]` but the module also uses `std::io::Error` directly
- **Lines**: L7
- **Category**: Maintainability
- **Description**: `crate::error::Error` is only used internally within `settings.rs` and `config.rs`. The rest of the codebase (all of `src/devcontainers/mod.rs`, all providers) returns `std::io::Error` directly, creating two parallel error channels. This inconsistency means callers must handle both `crate::error::Error` and raw `std::io::Error`, and there is no conversion from `crate::error::Error` into `std::io::Error`, forcing `map_err` at every boundary.
- **Suggestion**: Either unify on `anyhow::Error` (already a dependency in `main.rs`), or define an application-level error that encompasses both and implement `From<crate::error::Error> for std::io::Error` (or vice versa) so the boundary is seamless.

#### [Info] `ConfigParse` and `SettingsLoad` variants carry a `String` rather than a typed error
- **Lines**: L10–L13
- **Category**: Idiomatic Rust
- **Description**: Erasing the parse error into a `String` loses structured error context. Downstream code cannot distinguish sub-causes.
- **Suggestion**: Consider holding the original error type (e.g., `toml::de::Error`, `json5::Error`) behind a `Box<dyn std::error::Error + Send + Sync>` or using `thiserror`'s `#[source]` attribute with concrete types.

---

### `src/devcontainers/mod.rs`

#### [High] `exec_hook` return value silently ignored at every call-site in `post_create` and `run`
- **Lines**: L119, L166, L169, L173
- **Category**: Correctness
- **Description**: `exec_hook` returns `std::io::Result<bool>`, where `false` indicates the command ran but exited non-zero. In `post_create` (L166, L169, L173) and `run` (L119), the return value is not inspected; the `?` operator propagates I/O errors but silently ignores a non-zero exit. This means lifecycle hooks such as `onCreateCommand`, `updateContentCommand`, `postCreateCommand`, and `postStartCommand` can fail without the overall operation aborting.
- **Suggestion**: Mirror the pattern used for `initializeCommand` (L105–L110): check the `bool` result and return an `Err` on `false`.

#### [High] TOCTOU race in `Devcontainer::load` — file existence check before parse
- **Lines**: L66–L72
- **Category**: Safety & Security
- **Description**: `file.is_file()` is called, then later `file.exists()` is called again (L72) before attempting to parse. Two consecutive filesystem probes on the same path allow a race between the check and the read. In a hostile environment (e.g., a project directory on a shared filesystem), the file could be swapped between probes.
- **Suggestion**: Collapse the two checks into a single `Config::parse` call; match on `ErrorKind::NotFound` to emit the "not found" error, and handle other parse failures directly.

#### [Medium] Unnecessary `.clone()` on `Config` after construction
- **Lines**: L83
- **Category**: Idiomatic Rust
- **Description**: `config.clone()` is called when storing the config into `Devcontainer`. The `config` local variable is only used to create `provider` (by value/reference) before being cloned into the struct. It could simply be moved.
- **Suggestion**: Move `config` into the `Self { config, ... }` struct literal directly, reordering the statements so `build_provider` borrows it by reference before the move.

#### [Medium] `docker_build_source` and `podman_build_source` differ in Dockerfile path resolution with no explanation
- **Lines**: L303–L323
- **Category**: Correctness / Maintainability
- **Description**: `docker_build_source` joins `directory` with the Dockerfile path (L305), while `podman_build_source` joins `directory/.devcontainer/` (L316). This asymmetry is not documented and could produce wrong paths when a user supplies a relative `build.dockerfile` that is already relative to `.devcontainer/`. If the two functions are meant to behave identically, one is wrong. If intentionally different, the distinction needs a comment.
- **Suggestion**: Align both functions to the same path-joining strategy (the devcontainers spec says `build.dockerfile` is relative to the config file, i.e., `.devcontainer/`). Add an explicit test that verifies the resolved path for each provider.

#### [Medium] `copy` method falls through silently when `basedir` is non-UTF-8
- **Lines**: L190
- **Category**: Correctness
- **Description**: If `destpath.parent().to_str()` returns `None` (non-UTF-8 path), `basedir` is set to `"<non-utf8>"` and the code proceeds to call `mkdir -p -- '<non-utf8>'` inside the container, which will almost certainly fail silently (the `exec` result is propagated by `?`, but the command itself may succeed while creating a wrong directory).
- **Suggestion**: Return an `Err` instead of a sentinel string when `parent().to_str()` is `None`.

#### [Medium] `copy_gitconfig` is declared `fn copy_gitconfig(&self) -> std::io::Result<bool>` but callers discard the `bool`
- **Lines**: L228, L177
- **Category**: Correctness / API Design
- **Description**: `copy_gitconfig` returns `Ok(false)` when no `.gitconfig` exists, and `Ok(true)` when the copy succeeds. The call at L177 uses `self.copy_gitconfig()?;` which propagates errors but discards the `bool`. This is inconsistent with `copy_dotfiles` (returns `()`). Adding `#[must_use]` would surface this at the call-site.
- **Suggestion**: Either change the return type to `std::io::Result<()>` (treating "no .gitconfig" as a non-error no-op), or add `#[must_use]` and inspect the result at the call-site.

#### [Low] `create_args` clones `run_args` element-by-element via an iterator when a single `.extend` would suffice
- **Lines**: L259–L261
- **Category**: Idiomatic Rust
- **Description**: The loop `for arg in self.config.run_args.clone() { args.push(arg); }` clones the entire `run_args` Vec and then moves elements one-by-one. This is equivalent to `args.extend(self.config.run_args.iter().cloned())` and slightly less clear.
- **Suggestion**: Use `args.extend(self.config.run_args.iter().cloned())` (or `args.extend_from_slice(&self.config.run_args)`).

#### [Low] `workspace_folder` is cloned unnecessarily in `create_args`
- **Lines**: L255
- **Category**: Idiomatic Rust
- **Description**: `let workspace_folder = self.config.workspace_folder.clone();` clones a `String` just to push it. Could push a clone inline or borrow.
- **Suggestion**: Replace with `args.push(self.config.workspace_folder.clone());` inline, or hold the fields as `Arc<str>` to make clones cheap.

#### [Low] `missing_field` and `sorted_env_vars` are private helpers with no doc comments
- **Lines**: L267–L282
- **Category**: Maintainability
- **Description**: Private helper functions lack doc comments, making their intent less clear for future contributors.
- **Suggestion**: Add brief doc comments explaining what each helper does and when to use it.

#### [Info] `Devcontainer::run` calls `restart` unconditionally after `post_create`
- **Lines**: L123
- **Category**: Correctness
- **Description**: `provider.restart()` is called unconditionally after `post_create()`. If `post_create` ran any hooks, a restart is appropriate to pick up environment changes; however, if `post_create` is a no-op (all hooks absent, no dotfiles), the restart is unnecessary. The behaviour is not wrong (a benign extra restart), but it differs from the devcontainers spec, which only restarts when needed.
- **Suggestion**: Track whether any post-create work was done and skip the restart if nothing changed.

---

### `src/devcontainers/config.rs`

#### [Medium] `Config` fields that should be private are `pub`
- **Lines**: L26–L51
- **Category**: API Design
- **Description**: Many fields of `Config` are `pub` (`image`, `build`, `forward_ports`, `initialize_command`, etc.), allowing arbitrary mutation by any module within the crate. Since `Config` is only populated via deserialization and then consumed read-only, the fields could be `pub(crate)` or expose read-only accessor methods.
- **Suggestion**: Change mutable-intent fields to `pub(crate)` and expose read-only accessors where cross-module reads are needed. Alternatively, derive `#[non_exhaustive]` to signal the struct is not for public construction.

#### [Medium] `dockerfile()` and `build_args()` clone the entire `Build` struct unnecessarily
- **Lines**: L83, L89
- **Category**: Idiomatic Rust
- **Description**: `self.build.clone().and_then(|b| b.dockerfile)` clones the whole `Build` struct (including `args: HashMap`) just to extract one field. `build_args()` does the same.
- **Suggestion**: Use `self.build.as_ref().and_then(|b| b.dockerfile.clone())` to borrow the `Build` and only clone the `Option<String>` field. Similarly, `self.build.as_ref().map(|b| b.args.clone()).unwrap_or_default()`.

#### [Low] `safe_name` trims after lowercasing but the returned string still might have leading/trailing dashes if the original name had spaces at boundaries
- **Lines**: L97–L104
- **Category**: Correctness
- **Description**: The chain is `.to_lowercase().replace(' ', "-").trim().to_string()`. `trim()` removes whitespace, not dashes, so a name like `" my project "` becomes `"my-project"` (correct), but a name like `"- bad name -"` becomes `"-bad-name-"` (leading/trailing dashes in the container name). Container names with leading dashes can cause CLI argument-parsing issues.
- **Suggestion**: Also `trim_matches('-')` after the replace, or validate that the resulting name matches `[a-z0-9][a-z0-9-]*`.

#### [Low] `ShutdownAction` deserialization is unvalidated — unrecognised variants silently cause parse failures with no helpful message
- **Lines**: L8–L13
- **Category**: Maintainability
- **Description**: `ShutdownAction` derives `Deserialize` with no `serde(rename_all)` attribute. The JSON field `shutdownAction` must be an exact match (`"None"`, `"StopContainer"`, `"StopCompose"`) including capitalisation. Typos produce an `Error::ConfigParse` with a serde message that may not clearly identify which field is wrong.
- **Suggestion**: Add `#[serde(rename_all = "camelCase")]` or `PascalCase` as appropriate to the spec, and add a test for each variant.

#### [Info] `default_remote_user` and `default_workspace_folder` are free functions rather than associated constants
- **Lines**: L125–L131
- **Category**: Idiomatic Rust
- **Description**: Serde's `#[serde(default = "...")]` requires a function, so these cannot be constants, but the default values are string literals that could also be documented as constants alongside the struct.
- **Suggestion**: Add `pub(crate) const DEFAULT_REMOTE_USER: &str = "root";` etc. and reference them inside the default functions to avoid duplication if the values are used elsewhere.

---

### `src/devcontainers/one_or_many.rs`

#### [Info] No `Display` implementation for `OneOrMany`
- **Lines**: L9–L13
- **Category**: Maintainability
- **Description**: `OneOrMany` derives `Debug` but has no `Display`. When it appears in error messages, users will see `Rust` debug syntax rather than a clean representation.
- **Suggestion**: Implement `Display` that renders `One(cmd)` as the command string and `Many(parts)` as a space-joined list.

#### [Info] `to_exec_parts` could be made more allocation-friendly
- **Lines**: L23–L29
- **Category**: Idiomatic Rust
- **Description**: For `Many`, `parts[1..].to_vec()` allocates a new `Vec`. This is unavoidable if the return type owns the data, but callers receive owned `Vec<String>` only to immediately borrow them as `Vec<&str>`. An iterator-based API would avoid two allocations.
- **Suggestion**: Low priority. If performance matters, consider returning `(&str, impl Iterator<Item=&str>)`.

---

### `src/provider/mod.rs`

#### [Medium] `print_command` is `pub` but should be `pub(crate)`
- **Lines**: L83
- **Category**: API Design
- **Description**: `print_command` is a debug/display utility used exclusively within the provider implementations. There is no reason for it to be part of the public API surface.
- **Suggestion**: Change to `pub(crate) fn print_command`.

#### [Medium] `print_command` uses `.unwrap_or("<non-utf8>")` silently; non-UTF-8 args are logged as `<non-utf8>` with no indication of which argument
- **Lines**: L87
- **Category**: Safety & Security
- **Description**: When a non-UTF-8 `OsStr` argument appears, it is silently replaced with `<non-utf8>` in the printed command. While this does not affect execution, the logged command will be misleading and could impede debugging.
- **Suggestion**: Use `arg.to_string_lossy()` instead of `to_str().unwrap_or(...)` so that replacement characters appear rather than a generic sentinel, or log the index of the non-UTF-8 argument.

#### [Low] `Provider` trait methods return `Result<bool>` rather than `Result<()>`
- **Lines**: L16–L80
- **Category**: API Design
- **Description**: Most trait methods (`start`, `stop`, `restart`, `attach`, `rm`, `cp`, `exec`, `exec_raw`, `build`, `create`) return `Result<bool>` where `bool` indicates command success. This means callers must check both the `Result` (I/O failure) and the `bool` (exit code). At most call-sites, a `false` result is as problematic as an `Err`, yet many callers (see `src/devcontainers/mod.rs`) use `?` and ignore the `bool`, silently swallowing non-zero exit codes.
- **Suggestion**: Change the return type to `Result<()>` and propagate non-zero exits as `Err(std::io::Error::new(ErrorKind::Other, "command failed"))`. This makes it impossible to silently ignore failures.

---

### `src/provider/docker.rs`

#### [High] `exists()` and `running()` use `--filter name=<name>` which is a prefix match, not an exact match
- **Lines**: L186, L202
- **Category**: Correctness
- **Description**: `docker ps --filter name=X` matches any container whose name *contains* `X`. A container named `devcont-myproject-extra` would cause `exists()` / `running()` to return `true` when checking for `devcont-myproject`. This can produce incorrect lifecycle decisions (skipping build/create when the wrong container is running).
- **Suggestion**: Use `--filter name=^/devcont-myproject$` (anchored regex) as is already done in the integration test at `tests/integration.rs:367`, or use `docker inspect --format='{{.Name}}' <name>` and exact-match the output.

#### [Medium] SSH socket path from `SSH_AUTH_SOCK` env var is passed unsanitised into `--volume` argument
- **Lines**: L84–L88
- **Category**: Safety & Security
- **Description**: `env::var("SSH_AUTH_SOCK")` returns a user-controlled value. While it is passed as a discrete argument (not through a shell), the resulting bind-mount spec `{ssh_auth_sock}:/ssh-agent` is used verbatim. A path containing a comma or colon could corrupt the mount spec understood by Docker. More critically, if `SSH_AUTH_SOCK` were set to a path like `/tmp/sock,type=bind,target=/etc`, it could be misinterpreted by the Docker CLI.
- **Suggestion**: Validate that `SSH_AUTH_SOCK` is an absolute path (starts with `/`) and contains no commas or newlines before use. Log a warning and skip the SSH forwarding if validation fails.

#### [Medium] `Docker` struct fields are all `pub`
- **Lines**: L22–L33
- **Category**: API Design
- **Description**: All fields of `Docker` are `pub`, giving any caller full mutable access to runtime configuration. This is particularly risky for fields like `command` (controls which binary is invoked) and `name` (controls which container is targeted).
- **Suggestion**: Change all fields to `pub(crate)` since `Docker` is only constructed within the crate's own modules.

#### [Low] `IMAGE_NAMESPACE` constant is duplicated between `docker.rs` and `podman.rs`
- **Lines**: L9 (`docker.rs`), L10 (`podman.rs`)
- **Category**: Maintainability
- **Description**: The constant `"devcont"` is defined in both files. If the namespace ever needs to change, it must be updated in two places.
- **Suggestion**: Define `IMAGE_NAMESPACE` once in `provider/mod.rs` (or a shared constants module) and re-use it.

#### [Low] Override command uses `/bin/sh` in Docker but `sh` in Podman
- **Lines**: L127 (`docker.rs`), L108 (`podman.rs`)
- **Category**: Correctness
- **Description**: `Docker::create` uses `/bin/sh -c "while sleep 1000; ..."` while `Podman::create` uses `sh -c "..."`. On most images these are equivalent, but on minimal images without `/bin/sh` in PATH the Podman invocation could fail differently from the Docker one, and the inconsistency is surprising.
- **Suggestion**: Use the same path (`sh`) in both, or document the intentional difference.

---

### `src/provider/podman.rs`

#### [High] `exists()` and `running()` — same prefix-match issue as Docker
- **Lines**: L163–L195
- **Category**: Correctness
- **Description**: Same `--filter name=<name>` prefix-match issue as `docker.rs`. See Docker finding above.
- **Suggestion**: Same fix — use anchored regex or `podman inspect`.

#### [Medium] `Podman` struct fields are all `pub`
- **Lines**: L13–L24
- **Category**: API Design
- **Description**: Same visibility issue as `Docker`. All fields should be `pub(crate)`.
- **Suggestion**: Change all fields to `pub(crate)`.

#### [Medium] `Podman` does not support `mounts` from `devcontainer.json` while `Docker` does
- **Lines**: `podman.rs` entire `create` method vs. `docker.rs:L103–L114`
- **Category**: Correctness
- **Description**: The `Docker::create` implementation processes `self.mounts` (additional bind mounts from `devcontainer.json`), but `Podman::create` has no `mounts` field and ignores the config's `mounts` entirely. Users who switch provider from Docker to Podman will silently lose their mount configuration.
- **Suggestion**: Add `mounts: Option<Vec<HashMap<String, String>>>` to `Podman` and process it the same way as in `Docker::create`.

---

### `src/provider/docker_compose.rs`

#### [High] `DockerCompose::exists()` does not pass the override file — result may be wrong
- **Lines**: L160–L178
- **Category**: Correctness
- **Description**: `exists()` calls `docker compose -f <file> -p <name> ps -aq` without including the override file (`-f <docker_override>`). If the override file defines environment variables or volumes that affect service discovery, the result could differ from other operations that do include it. More practically, the override file path written to `env::temp_dir()` may not exist yet when `exists()` is first called, making the omission benign in that case — but inconsistent.
- **Suggestion**: Align `exists()` and `running()` with the other methods by including the override file, or explicitly document why it is omitted.

#### [Medium] `DockerCompose` struct fields are all `pub`
- **Lines**: L10–L20
- **Category**: API Design
- **Description**: Same visibility issue as `Docker` and `Podman`. Fields like `command` and `file` should be `pub(crate)`.
- **Suggestion**: Change all fields to `pub(crate)`.

#### [Medium] Compose override file is recreated on every single operation (build, start, stop, restart, attach, rm, cp, exec, exec_raw)
- **Lines**: Throughout implementation
- **Category**: Maintainability / Performance
- **Description**: `create_docker_compose()` (and thus `create_compose_override()`) writes a temp file to disk on every provider method call. This means for a `run()` lifecycle (build → start → exec → restart → attach → stop), the file is written 6+ times. This creates unnecessary I/O and a subtle race: if two concurrent compose operations are running (even for different projects), they share the same temp file path (`env::temp_dir()/docker-compose.yml`) and can corrupt each other's override.
- **Suggestion**: Cache the override file path in the struct after first creation. Include the project name in the filename to avoid inter-project collisions: e.g., `docker-compose-<name>.yml`.

---

### `src/provider/podman_compose.rs`

#### [High] Same shared temp file race as `DockerCompose` — amplified by two concurrent callers
- **Lines**: L24–L26 (`create_docker_compose`), and `utils.rs:L32`
- **Category**: Safety & Security / Correctness
- **Description**: `PodmanCompose` shares the same `create_compose_override` utility, which always writes to `env::temp_dir()/docker-compose.yml`. Both `DockerCompose` and `PodmanCompose` will overwrite the same file. Running Docker and Podman compose workflows concurrently (or running two different projects) will cause one to read the other's override file.
- **Suggestion**: Include the project name in the override filename and cache it.

#### [Medium] `PodmanCompose` struct fields are all `pub`
- **Lines**: L10–L21
- **Category**: API Design
- **Description**: Same visibility issue. All fields should be `pub(crate)`.
- **Suggestion**: Change all fields to `pub(crate)`.

#### [Medium] `extract_container_id` is a private static method but has no unit tests for multi-line output with non-empty intermediate lines
- **Lines**: L28–L33
- **Category**: Maintainability
- **Description**: `extract_container_id` takes the first non-blank line after trimming. Existing tests cover single-line, blank-leading, and empty cases. However, `podman ps -q` with multiple matching containers (e.g., if the label filter is too broad) would return multiple IDs and the function would silently use only the first.
- **Suggestion**: Add a test for multi-line output and document the "first container wins" behaviour explicitly.

#### [Low] `PodmanCompose::rm_args` omits `--rmi all` but `DockerCompose::rm` passes `--rmi all`
- **Lines**: `podman_compose.rs:L46–L57`, `docker_compose.rs:L151–L153`
- **Category**: Correctness
- **Description**: `DockerCompose::rm` passes `--rmi all` to remove images on teardown, while `PodmanCompose::rm` (via `rm_args`) does not. This asymmetry means Podman compose workflows leave images behind after `rebuild`. The test at `podman_compose.rs:L378–L385` explicitly asserts `--rmi` is absent, making this a deliberate difference — but it is undocumented.
- **Suggestion**: Add a comment in `rm_args` explaining why `--rmi` is omitted for podman-compose (if intentional, e.g., podman-compose does not support it).

---

### `src/provider/utils.rs`

#### [High] Override file written to `env::temp_dir()/docker-compose.yml` — fixed filename, world-readable, shared across projects
- **Lines**: L31–L65
- **Category**: Safety & Security
- **Description**: The temp file is always written to the same fixed path (`<tmpdir>/docker-compose.yml`). This has three problems:
  1. **Concurrent project collision**: running two different devcont projects simultaneously will overwrite each other's override file.
  2. **World-readable location**: on Linux, `/tmp` is world-readable. The override file may contain environment variable values from `remoteEnv` (e.g., API tokens, database passwords). Any local user can read them.
  3. **Predictable path**: a malicious local process could pre-create the file and intercept or inject content before `docker compose` reads it.
- **Suggestion**: Use `tempfile::NamedTempFile` (or `tempfile::Builder::new().suffix(".yml").tempfile_in(...)`) to get a randomly-named file with restricted permissions (0600). Alternatively, include the project name and a random suffix in the filename.

#### [Medium] SSH socket path (`ssh_auth_sock`) from environment variable is embedded directly in the YAML file content without sanitisation
- **Lines**: L42–L51
- **Category**: Safety & Security
- **Description**: `ssh_auth_sock` is taken from `SSH_AUTH_SOCK` and inserted into the YAML template without any validation or escaping. A value containing YAML-special characters (`{`, `}`, `:`, `#`, etc.) or a newline could break the generated YAML or, in a worst case, inject YAML keys.
- **Suggestion**: Validate that `SSH_AUTH_SOCK` is an absolute path with no YAML-special characters before inserting it, or ensure the template properly quotes the value.

#### [Low] `create_compose_override` is `pub(crate)` but `TemplateContext` and `TemplateEntry` are private — appropriate
- **Lines**: L7–L17
- **Category**: API Design
- **Description**: Visibility is correctly scoped. No issue.
- **Suggestion**: No action required.

---

### `tests/integration.rs`

#### [Medium] Podman tests skip silently rather than being conditionally compiled or marked `#[ignore]`
- **Lines**: L923–L926, L960–L963, L985–L988, etc.
- **Category**: Maintainability
- **Description**: Podman-compose tests return early with `eprintln!("SKIP: ...")` when `podman-compose` is not available. This causes the test to pass (exit 0) rather than being explicitly skipped. In CI, a misconfigured environment silently appears to have passing tests when in reality no Podman coverage is exercised.
- **Suggestion**: Use `#[ignore]` with `cargo test -- --include-ignored` for optional tests, or the `skipif` pattern via a test attribute macro, or check at test module level and emit a compile-time or runtime warning.

#### [Medium] Integration tests do not guard against the Docker daemon being absent
- **Lines**: L326–L335
- **Category**: Maintainability
- **Description**: `test_docker_available` is a prerequisite check, but it is a separate test that could pass while later tests fail. If Docker is not available, all subsequent tests will fail with unhelpful "failed to spawn" panics rather than a clear skip/ignore message.
- **Suggestion**: Add a `command_available("docker")` guard to Docker tests analogous to the podman-compose guard.

#### [Low] `unique_name` uses nanosecond timestamps which may collide on fast machines or VMs with low-resolution clocks
- **Lines**: L167–L173
- **Category**: Correctness
- **Description**: `SystemTime::now().duration_since(UNIX_EPOCH).as_nanos()` provides collision resistance only if the system clock has nanosecond resolution. On some VMs (especially macOS virtualisation layers), the clock resolution may be coarser.
- **Suggestion**: Append a random suffix (e.g., using `rand` or `uuid`) in addition to the timestamp.

#### [Low] `build_image` uses `.expect()` at the test helper level — panics surface as test failures with confusing messages
- **Lines**: L192–L196
- **Category**: Maintainability
- **Description**: Using `expect()` in test helpers is acceptable but means failure messages point to the helper line rather than the test. For a helper used across many tests, `Result`-returning helpers with `?` and `assert!` at the test call-site are more debuggable.
- **Suggestion**: Consider returning `Result<String, ...>` from `build_image` and letting each test assert the result.

---

## Summary Table

| Severity | Count |
|---|---|
| Critical | 0 |
| High | 7 |
| Medium | 17 |
| Low | 12 |
| Info | 6 |
| **Total** | **42** |

---

## Follow-up Track Stubs

For each cluster of related findings, a conductor track is proposed:

1. **`fix_ignored_hook_results`** — Propagate non-zero exit codes from all lifecycle hook calls (`exec_hook`) as errors; audit every `Result<bool>` call-site in `devcontainers/mod.rs` to ensure failures abort the workflow.

2. **`fix_container_name_filter_exactmatch`** — Replace `--filter name=<name>` with anchored regex or `inspect`-based exact matching in `Docker::exists`, `Docker::running`, `Podman::exists`, and `Podman::running` to prevent prefix-match false positives.

3. **`fix_compose_override_tempfile`** — Replace the fixed-path temp file in `provider/utils.rs` with a randomly-named, mode-0600 temporary file (using `tempfile` crate), include the project name in the filename, and cache the path in the struct to avoid per-call I/O and inter-project collision.

4. **`fix_provider_visibility`** — Tighten `pub` fields on `Docker`, `Podman`, `DockerCompose`, `PodmanCompose`, `Settings`, and `Config` to `pub(crate)`, and change `print_command` to `pub(crate)`.

5. **`fix_dockerfile_path_asymmetry`** — Align `docker_build_source` and `podman_build_source` in `devcontainers/mod.rs` to use the same Dockerfile path-resolution strategy (relative to `.devcontainer/`) and add regression tests.

6. **`fix_podman_missing_mounts`** — Add the `mounts` field to `Podman` and implement it in `Podman::create` to match `Docker::create` parity.

7. **`fix_ssh_sock_validation`** — Validate `SSH_AUTH_SOCK` is a safe absolute path before embedding it in bind-mount arguments (`docker.rs`, `podman.rs`) and YAML templates (`utils.rs`).

8. **`unify_error_types`** — Replace the dual `crate::error::Error` / `std::io::Error` pattern with a single application error type (or `anyhow`) that covers all error cases uniformly.

9. **`fix_provider_result_bool_api`** — Change `Provider` trait methods from `Result<bool>` to `Result<()>`, converting non-zero command exits to `Err`, so callers cannot silently ignore failures.

10. **`fix_safe_name_validation`** — Tighten `Config::safe_name()` to produce a name that is guaranteed to be valid for Docker/Podman (trim leading/trailing dashes, enforce character set).
