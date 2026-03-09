# Architecture Review & STRIDE Threat Model Report
## date: 2026-03-09

---

## 1. Architecture Findings

### [High] Inconsistent Dockerfile Path Resolution Between Docker and Podman Providers

- **Area**: `src/devcontainers/mod.rs` — `docker_build_source` vs `podman_build_source`
- **Description**: The two helper functions resolve the Dockerfile path differently. `docker_build_source` joins the Dockerfile name directly onto `directory` (the project root), while `podman_build_source` joins it onto `directory/.devcontainer/`. This is a silent divergence: the same `devcontainer.json` will resolve to a different Dockerfile path depending on the active provider, which will cause silent build failures that are hard to diagnose.
- **Recommendation**: Unify path resolution in a single `resolve_dockerfile_path(directory, config)` function. The correct join base should be determined once from the location of `devcontainer.json` (which is already established in `Devcontainer::load`), not duplicated per-provider.

---

### [High] Unvalidated `run_args` and `mounts` Passed Directly to Container CLI

- **Area**: `src/devcontainers/config.rs` (Config), `src/provider/docker.rs` (Docker::create), `src/devcontainers/mod.rs` (create_args)
- **Description**: The `runArgs` and `mounts` fields are deserialized from workspace-supplied `devcontainer.json` and forwarded verbatim as CLI arguments to `docker create` / `podman create`. No allowlist, denylist, or shape validation is applied. An attacker who controls `devcontainer.json` can pass arbitrary flags such as `--privileged`, `--cap-add=ALL`, `--pid=host`, or `--security-opt seccomp=unconfined`, escalating from container-level to host-level access without any warning.
- **Recommendation**: Validate `runArgs` against a strict allowlist of safe flags (port publishing, labels, environment variables, named volume references). Reject or warn on flags that grant elevated privileges. Document the accepted set. Consider a schema-validation step immediately after `Config::parse`.

---

### [High] `initializeCommand` Executed on Host Without Confirmation

- **Area**: `src/devcontainers/mod.rs` — `exec_host_hook`, `Devcontainer::run`
- **Description**: When the string form of `initializeCommand` is used (`OneOrMany::One`), it is handed to `sh -c` on the host before any container exists. Any workspace-supplied command string is executed with the full permissions of the invoking user. The Dev Containers spec notes this as a security boundary that implementations should warn about, but `devcont` silently executes it.
- **Recommendation**: Before executing a host-side hook, print the full resolved command and require explicit user confirmation (or provide a `--trust` / `--no-host-hooks` CLI flag). Consider logging the command to stderr at minimum.

---

### [Medium] Compose Override File Written to a Fixed, World-Readable Path

- **Area**: `src/provider/utils.rs` — `create_compose_override`
- **Description**: The override file is written to `env::temp_dir()` (typically `/tmp`) with the static name `docker-compose.yml`. This creates two problems: (1) the file is world-readable on shared systems and contains environment variable names and the SSH agent socket path, both of which are sensitive; (2) the fixed filename is a symlink-attack vector — an adversary who can create `/tmp/docker-compose.yml` as a symlink before `devcont` runs can redirect the write to an arbitrary path, overwriting files owned by the invoking user.
- **Recommendation**: Use a unique, randomly named file (e.g., via `tempfile::NamedTempFile`) with restrictive permissions (mode 0600). Delete the file when the operation completes (RAII approach using `tempfile`). This both prevents information disclosure and eliminates the symlink race.

---

### [Medium] Error Taxonomy Too Coarse — `std::io::Error` Used Across Module Boundaries

- **Area**: `src/error.rs`, `src/devcontainers/mod.rs`, all provider modules
- **Description**: `devcontainers/mod.rs` and all providers return `std::io::Result` rather than the crate's own `Error` type, creating two parallel error domains. The application-level `Error` enum (`Io`, `ConfigParse`, `SettingsLoad`) is used only in the settings and config parsing layers. Lifecycle hook failures, provider spawn failures, and build failures are all collapsed into `std::io::Error::new(ErrorKind::Other, ...)` with unstructured string messages, making programmatic error handling impossible and diagnostic output inconsistent.
- **Recommendation**: Extend the `Error` enum with variants for provider operations (`ProviderBuild`, `ProviderExec`, `HookFailed { hook_name, exit_code }`). Migrate provider return types to `crate::error::Result<T>`. This also enables structured logging if a log facade is added later.

---

### [Medium] `build.context` Field Parsed but Silently Ignored

- **Area**: `src/devcontainers/config.rs` — `Build::context`, `src/provider/docker.rs` — `Docker::build`
- **Description**: The `build.context` field from the Dev Containers spec is parsed into `Config` but never used. `Docker::build` always passes `self.directory` (the project root) as the build context. If a project relies on a custom build context, the image will build incorrectly without any error or warning.
- **Recommendation**: Either wire `build.context` through to `Docker::build` / `Podman::build`, or emit a clear warning when the field is present and non-empty, directing the user to the known limitation.

---

### [Medium] `Provider::create` Signature Leaks Internal State — Args Passed as Raw Strings

- **Area**: `src/provider/mod.rs` — `Provider` trait, `src/devcontainers/mod.rs` — `Devcontainer::create_args`
- **Description**: The `create(&self, args: Vec<String>)` method accepts pre-serialised CLI flag strings (e.g., `["-e", "KEY=VALUE", "-w", "/workspace"]`). This leaks the concrete CLI calling convention into the trait's abstraction level: callers must know they are building Docker/Podman CLI flag strings. Compose providers ignore `args` entirely (`create` returns `Ok(true)` unconditionally), causing the environment variables and working directory that come from `create_args` to be silently dropped for Compose-based configs.
- **Recommendation**: Replace the `Vec<String>` parameter with a typed `CreateOptions` struct (`env_vars`, `working_dir`, `run_args`) that each provider can translate to its own CLI representation. This makes the silent-drop in Compose providers visible and fixable.

---

### [Low] Settings Silently Fall Back to Defaults on Parse Error

- **Area**: `src/settings.rs` — `Settings::load`
- **Description**: Any malformed `config.toml` causes a warning on stderr and a return of default settings (Docker provider, no dotfiles). This is a safe degradation strategy, but it means a typo such as `provider = "podman"` written as `provider = "Podman"` (wrong case) silently activates the Docker provider instead of failing. The user has no way to distinguish "no config file" from "broken config file" without reading stderr carefully.
- **Recommendation**: Distinguish the two cases: if no config file exists, return defaults silently; if a config file exists but fails to parse, surface a non-zero exit (or at minimum a prominent, distinct warning) so the operator knows their configuration is broken.

---

### [Low] Lifecycle Hook Ordering After Container Create Is Ambiguous

- **Area**: `src/devcontainers/mod.rs` — `Devcontainer::run`, `post_create`
- **Description**: The Dev Containers spec ordering is: `onCreateCommand` → `updateContentCommand` → `postCreateCommand`. The implementation matches this. However, `postStartCommand` is run immediately after `start()` returns (before `post_create`), which diverges from the spec, where `postStartCommand` runs after `postCreateCommand`. Additionally, `restart()` is called between `post_create` and `attach`, which is non-standard and may cause hooks to execute against a temporarily stopped container if `restart` returns before the container is fully up.
- **Recommendation**: Verify hook ordering against the Dev Containers spec reference implementation. Consider removing the unconditional `restart()` call and replacing it with a readiness poll. Document intentional deviations.

---

### [Low] `safe_name` Does Not Sanitise All Characters That Are Unsafe in Container Names

- **Area**: `src/devcontainers/config.rs` — `Config::safe_name`
- **Description**: `safe_name` lowercases and replaces spaces with dashes, but allows through characters that are illegal in Docker container names (e.g., `/`, `.`, `:`). A project name such as `my/project` yields `devcont-my/project`, which will cause `docker create --name devcont-my/project` to fail with an opaque Docker error rather than a clear validation message.
- **Recommendation**: Apply a regex replacement that retains only `[a-z0-9-_]` characters and enforce a maximum length. Validate the resulting name before constructing the provider.

---

## 2. STRIDE Threat Table

| # | Category | Threat | Component | Likelihood | Impact | Mitigation |
|---|---|---|---|---|---|---|
| 1 | Spoofing | A workspace-supplied `image` field names a lookalike image (e.g., `ubunlu:latest` instead of `ubuntu:latest`). The tool pulls and runs it without digest pinning. | `config.rs` → `docker.rs` `BuildSource::Image` | M | H | Recommend (or enforce) digest-pinned image references (`image@sha256:…`). Display the resolved image name prominently before pulling. |
| 2 | Spoofing | The SSH agent socket path from `SSH_AUTH_SOCK` is forwarded without verifying that the path actually belongs to the user's agent. On a shared host an adversary could set `SSH_AUTH_SOCK` to point at a malicious socket. | `docker.rs`/`podman.rs` `create`, `utils.rs` | L | H | Verify that the socket path resolves to a file owned by the effective UID before mounting it. Warn and skip if ownership check fails. |
| 3 | Tampering | The compose override file at `$TMPDIR/docker-compose.yml` uses a fixed name. A race condition allows an attacker with write access to `$TMPDIR` to replace the file between the write and the `docker compose -f` read, injecting arbitrary compose directives. | `utils.rs` `create_compose_override` | M | H | Use `tempfile::NamedTempFile` for an unguessable path and delete on drop, closing the TOCTOU window. |
| 4 | Tampering | `devcontainer.json` is read from the workspace directory without integrity verification. Any party who can modify the file (e.g., a malicious git commit, supply-chain compromise) controls `initializeCommand`, `runArgs`, image name, and lifecycle hooks. | `devcontainers/mod.rs` `Devcontainer::load` | M | H | Display a summary of security-relevant fields (`initializeCommand`, `runArgs`, image) on first use and after changes, requiring confirmation. Optionally support a config hash stored outside the workspace. |
| 5 | Tampering | `runArgs` from `devcontainer.json` are appended verbatim to `docker create`. A value such as `--volume=/:/host-root` would mount the host filesystem into the container without any warning. | `docker.rs`/`podman.rs` `create` | H | H | Enforce an allowlist of accepted run arguments. Any flag not on the list must be rejected with a clear error. |
| 6 | Tampering | `build_args` key-value pairs are formatted as `KEY=VALUE` and passed to `--build-arg`. If a key contains `=`, the argument is mis-parsed by the Docker CLI, potentially overriding a different build arg silently. | `docker.rs`/`podman.rs` `build` | L | M | Validate that build arg keys match `[A-Za-z_][A-Za-z0-9_]*` before use. Reject keys containing `=` or whitespace. |
| 7 | Repudiation | No structured log is written of which lifecycle hooks were executed, which image was pulled, or which container was created. If a hook causes damage, there is no audit trail. | All lifecycle hook sites in `devcontainers/mod.rs` | H | M | Emit a structured log (JSON or timestamped plaintext) to `~/.local/share/devcont/audit.log` recording the workspace path, image, hooks executed, and their exit codes. |
| 8 | Repudiation | `docker ps` / `podman ps` output used for `exists()` and `running()` is not validated — a container from a *different* project whose name is a prefix of the current name may match the filter, causing false positives. | `docker.rs`/`podman.rs` `exists`, `running` | L | M | Use `--filter name=^<exact-name>$` (anchored regex) or `docker inspect <name>` for exact-name lookup to prevent prefix collisions. |
| 9 | Information Disclosure | The compose override file written to `$TMPDIR/docker-compose.yml` contains all `remoteEnv` key-value pairs in plaintext with world-readable default permissions. Secrets stored in `remoteEnv` (tokens, passwords) are exposed to any local user. | `utils.rs` `create_compose_override` | H | H | Write the file with mode 0600 (or use `tempfile::NamedTempFile` which does this by default). Delete the file immediately after the Compose command finishes. |
| 10 | Information Disclosure | The SSH agent socket path is embedded in the compose override file (world-readable) and also printed to stdout via `print_command`. This leaks the socket path to any process that can read stdout or `/tmp`. | `utils.rs`, `provider/mod.rs` `print_command` | M | M | Redact socket paths and secret env var values from `print_command` output. Restrict override file permissions as above. |
| 11 | Information Disclosure | `~/.gitconfig` is copied into the container. This file may contain plaintext credentials if the user has stored a PAT or password in `[credential]` or `[url "..."]` blocks. The container inherits these secrets for the duration of the session. | `devcontainers/mod.rs` `copy_gitconfig` | M | M | Copy only a safe subset of gitconfig (name, email, alias blocks) or warn the user that credential sections will be copied and offer an opt-out. |
| 12 | Information Disclosure | `print_command` prints the full CLI invocation to stdout in bold blue. This includes image names, volume paths, environment variable keys and values, and the SSH socket path — all visible in CI logs, shell history capture, or shared terminal sessions. | `provider/mod.rs` `print_command` | M | L | Redact values of environment variables known to be sensitive (matching patterns like `*TOKEN*`, `*SECRET*`, `*PASSWORD*`). Provide a `--quiet` flag to suppress command printing entirely. |
| 13 | Denial of Service | The `rebuild` command calls `stop()` then `rm()` on the container before re-running. If `stop()` succeeds but `rm()` fails (e.g., Docker daemon timeout), the container is stopped but not removed, and the subsequent `build` + `create` will fail with a name conflict, leaving the environment in a broken state. | `devcontainers/mod.rs` `Devcontainer::rebuild` | M | M | After `rm()` failure, attempt `docker rm -f <name>` as a fallback. Report both errors to the user with recovery instructions. |
| 14 | Denial of Service | If a lifecycle hook (`onCreateCommand`, `postCreateCommand`) hangs indefinitely, the tool blocks forever with no timeout. A malicious or buggy hook can make the environment permanently unusable without user intervention. | `devcontainers/mod.rs` `exec_hook` | M | M | Apply a configurable timeout to all lifecycle hook executions. Default to a sane upper bound (e.g., 10 minutes). Surface the hook name in the timeout error message. |
| 15 | Elevation of Privilege | Docker provider passes `-v /run/docker.sock:/var/run/docker.sock` if the user adds it to `runArgs`. Combined with a writable Docker socket inside the container (DooD), this gives the container process full Docker daemon access, equivalent to root on the host. `runArgs` is not validated, so this is trivially injectable from `devcontainer.json`. | `docker.rs` `create`, `config.rs` `run_args` | H | H | Block `runArgs` values that reference Docker/Podman sockets (pattern `/docker.sock`, `/podman.sock`). If socket forwarding is needed, provide a first-class opt-in mechanism with explicit confirmation. |
| 16 | Elevation of Privilege | The Dockerfile path from `devcontainer.json` is joined to the project directory using `Path::join` without canonicalization or sandbox validation. A value of `../../etc/malicious-dockerfile` could escape the `.devcontainer` directory and cause the tool to build from an attacker-controlled file anywhere on disk. | `devcontainers/mod.rs` `docker_build_source` / `podman_build_source` | M | H | Canonicalize the resolved Dockerfile path and verify it is a child of the project's `.devcontainer` directory before using it. Return an error if the check fails. |
| 17 | Elevation of Privilege | The `dockerComposeFile` path is joined to the project's `.devcontainer` directory without path traversal checks. A value of `../../etc/passwd` would attempt to feed an arbitrary host file to `docker compose -f`. | `devcontainers/mod.rs` `compose_path_and_service` | M | H | Apply the same canonicalization-and-containment check as for the Dockerfile path. |
| 18 | Elevation of Privilege | Podman provider passes `--security-opt label=disable` unconditionally, disabling SELinux/AppArmor confinement for every container regardless of the security posture of the host. | `podman.rs` `create` | M | M | Make SELinux label disabling opt-in via a setting or config flag. Default to leaving SELinux confinement active. |
| 19 | Elevation of Privilege | `remote_user` from `devcontainer.json` defaults to `root` (`default_remote_user`). This means all lifecycle hooks and the interactive session run as root inside the container unless the project explicitly sets a non-root user. | `config.rs` `default_remote_user` | H | M | Change the default to a non-root user (e.g., `vscode`) consistent with the Dev Containers reference implementation. Warn prominently when `remoteUser` resolves to `root`. |
| 20 | Elevation of Privilege | Environment variable injection via `remoteEnv` is not sanitised. A key or value containing newlines could corrupt the generated compose override YAML, potentially injecting additional YAML keys or altering service definitions in unintended ways. | `utils.rs` `create_compose_override` | L | H | Validate that `remoteEnv` keys match `[A-Za-z_][A-Za-z0-9_]*` and that values do not contain newlines or YAML special characters before embedding them in the template. |

---

## 3. Prioritised Action List

1. [Critical] **Block privileged `runArgs` flags** — Validate `runArgs` and `mounts` from `devcontainer.json` against an allowlist before they are passed to the container runtime. Reject flags such as `--privileged`, `--cap-add`, `--pid`, `--net=host`, and volume mounts of Docker/Podman sockets. This is the most direct path to host compromise. Suggested track: `security_runeargs_allowlist_20260309`

2. [Critical] **Fix compose override file security** — Replace the fixed-path `/tmp/docker-compose.yml` with a `tempfile::NamedTempFile` (mode 0600, deleted on drop). This simultaneously closes the information disclosure of `remoteEnv` secrets, the symlink race, and the TOCTOU tampering window. Suggested track: `security_tempfile_override_20260309`

3. [Critical] **Validate and canonicalize path inputs from `devcontainer.json`** — Apply `canonicalize()` + containment check to `dockerComposeFile` and `build.dockerfile` path values. Reject traversals that escape the project directory. Suggested track: `security_path_traversal_20260309`

4. [High] **Require user confirmation before running `initializeCommand` on the host** — Print the resolved command and prompt the user (or require `--trust` CLI flag) before executing any host-side hook from workspace-supplied config. Suggested track: `security_host_hook_confirmation_20260309`

5. [High] **Change default `remoteUser` from `root` to `vscode`** — Align with the Dev Containers reference implementation. Add a clear warning when the effective user inside the container is `root`. Suggested track: `security_default_user_20260309`

6. [High] **Fix Dockerfile path resolution divergence between Docker and Podman** — Unify into a single function so provider selection does not silently change which Dockerfile is used. Suggested track: `refactor_dockerfile_path_20260309`

7. [High] **Extend `Error` enum with structured provider and hook error variants** — Replace the `std::io::Error::new(Other, string)` pattern throughout provider and lifecycle code with typed error variants for actionable diagnostics. Suggested track: `refactor_error_taxonomy_20260309`

8. [Medium] **Redact secrets from `print_command` output** — Suppress environment variable values matching `*TOKEN*`, `*SECRET*`, `*PASSWORD*`, `*KEY*` and the SSH socket path in the blue command preview printed to stdout. Suggested track: `security_command_redaction_20260309`

9. [Medium] **Add an audit log for lifecycle operations** — Emit a timestamped log entry for every hook executed (name, command, exit code) and every container created/destroyed. Suggested track: `observability_audit_log_20260309`

10. [Medium] **Wire `build.context` into providers or emit a warning** — Either implement the `build.context` field end-to-end or warn loudly when a project sets it, preventing silent build context mismatch. Suggested track: `feature_build_context_20260309`

11. [Medium] **Make Podman's `--security-opt label=disable` opt-in** — Remove the unconditional SELinux/AppArmor disabling from `Podman::create` and expose it as an explicit setting. Suggested track: `security_podman_selinux_20260309`

12. [Low] **Anchor container name filters to prevent prefix collisions** — Use exact-name filters in `exists()` and `running()` to avoid false positives from containers whose names share a prefix. Suggested track: `fix_container_name_filter_20260309`
