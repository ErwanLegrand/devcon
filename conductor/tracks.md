# Project Tracks

This file tracks all major tracks for the project. Each track has its own detailed plan in its respective folder.

---

## [x] Track: Foundation Modernization
*Rename to `devcont`, Rust 2024 + MSRV 1.85, dependency upgrades, official rust dev container, rust-dev-template integration, CI quality gates.*
*Link: [./archive/foundation_modernization_20260227/](./archive/foundation_modernization_20260227/)*

---

---

## [x] Track: Docker-outside-of-Docker for Dev Container
*Add DooD support (socket mount + Docker CLI) to the dev container, and a minimal integration test suite (`tests/integration.rs`) with purpose-built fixtures.*
*Link: [./archive/dood_devcontainer_20260228/](./archive/dood_devcontainer_20260228/)*

---

## [x] Track: Docker Provider Integration Tests
*Comprehensive integration tests for `Docker` and `DockerCompose` providers — all `Provider` trait methods. Blocked on: DooD Dev Container track.*
*Link: [./archive/integration_tests_docker_20260228/](./archive/integration_tests_docker_20260228/)*

---

## [x] Track: Podman Provider Integration Tests
*Comprehensive integration tests for `Podman` and `PodmanCompose` providers — all `Provider` trait methods. Blocked on: DooD Dev Container track + Docker integration tests track.*
*Link: [./archive/integration_tests_podman_20260228/](./archive/integration_tests_podman_20260228/)*

---

## [x] Track: podman-compose Provider Completion
*Fix concrete bugs and missing wiring in the PodmanCompose provider (running() flags, attach shell, cp() ID handling, remote_env injection, rm() compatibility). Same fixes applied to DockerCompose where applicable.*
*Link: [./archive/podman_compose_completion_20260308/](./archive/podman_compose_completion_20260308/)*

---

## [x] Track: Dev Containers Spec Compliance — Phase 1
*Add image field support, missing lifecycle hooks (postStartCommand, postAttachCommand, initializeCommand), and OneOrMany array form for all hook values.*
*Link: [./archive/spec_compliance_phase1_20260308/](./archive/spec_compliance_phase1_20260308/)*

---

## [x] Track: Provider exec_raw — Injection-Safe Direct Exec for Many Hooks
*Add `exec_raw(&self, prog, args)` to the `Provider` trait and all four implementations so that `OneOrMany::Many` hooks execute without a shell wrapper.*
*Priority: Medium. Originated from review of spec_compliance_phase1.*
*Link: [./archive/provider_exec_raw_20260309/](./archive/provider_exec_raw_20260309/)*

---

## [x] Track: PodmanCompose Unit Tests — running(), cp(), rm() Bug Fix Coverage
*Add targeted unit tests for the three PodmanCompose bug fixes from podman-compose Provider Completion.*
*Priority: Low. Originated from review of podman_compose_completion.*
*Link: [./archive/podman_compose_unit_tests_20260309/](./archive/podman_compose_unit_tests_20260309/)*

---

## [x] Track: build_source_for Refactor — Replace bool Parameter with Explicit Helpers
*Replace the opaque `in_devcontainer: bool` parameter with two dedicated `docker_build_source` / `podman_build_source` helpers.*
*Priority: Low. Originated from review of spec_compliance_phase1.*
*Link: [./archive/build_source_refactor_20260309/](./archive/build_source_refactor_20260309/)*

---

## [x] Track: Architecture Review & STRIDE Threat Modelling
*Structured architecture review of module boundaries, Provider trait design, config pipeline, error propagation, and lifecycle hook semantics. STRIDE threat model applied to all external input surfaces (devcontainer.json, SSH socket, DooD, temp files, hooks).*
*Priority: High.*
*Link: [./archive/arch_review_stride_20260309/](./tracks/arch_review_stride_20260309/)*

---

## [x] Track: Thorough Code Review
*File-by-file review of all `src/` files covering correctness, safety, idiomatic Rust, maintainability, and API design. Produces a severity-rated findings report and follow-up track stubs.*
*Priority: High.*
*Link: [./archive/code_review_thorough_20260309/](./tracks/code_review_thorough_20260309/)*

---

## [x] Track: Test Coverage Assessment
*Measure baseline line/branch coverage with cargo-llvm-cov, identify gaps below 80%, classify by risk and feasibility, and produce a prioritised plan for closing the most important coverage holes.*
*Priority: Medium.*
*Link: [./archive/test_coverage_assessment_20260309/](./tracks/test_coverage_assessment_20260309/)*

---

## [x] Track: Fuzzing Feasibility Assessment
*Evaluate whether cargo-fuzz (libFuzzer) is worthwhile for devcont. Assess candidate targets (config parsing, OneOrMany, compose override, hook dispatch), rate by ROI, and produce a Go/No-Go recommendation with implementation plan.*
*Priority: Medium.*
*Link: [./archive/fuzzing_assessment_20260309/](./tracks/fuzzing_assessment_20260309/)*

---

## [x] Track: Documentation Assessment & README Overhaul
*Assess README.md and inline doc coverage. README should be concise, actionable, and link to the upstream project. Identify and fill doc-comment gaps on public API items.*
*Priority: Medium.*
*Link: [./archive/documentation_assessment_20260309/](./tracks/documentation_assessment_20260309/)*

---

---

## Remediation Tracks (from assessment reports, 2026-03-09)

### Critical Security

## [x] Track: Security — Validate runArgs Against Allowlist
*Validate `runArgs` from devcontainer.json against a privilege-escalation denylist before passing to container runtime. Abort on `--privileged`, `--cap-add`, etc.*
*Priority: Critical. From: arch_review_stride + code_review_thorough.*
*Link: [./tracks/sec_runargs_allowlist_20260309/](./tracks/sec_runargs_allowlist_20260309/)*

---

## [x] Track: Security — Fix Compose Override Temp File Permissions and Cleanup
*Create compose override file with mode 0o600 and auto-delete via Drop guard. Prevents information disclosure of SSH socket paths and env var values.*
*Priority: Critical. From: arch_review_stride + code_review_thorough.*
*Link: [./tracks/sec_tempfile_secret_20260309/](./tracks/sec_tempfile_secret_20260309/)*

---

## [x] Track: Security — Prevent Path Traversal in Workspace and Dockerfile Paths
*Canonicalise and validate build.dockerfile, build.context, and relative mounts paths so they cannot escape the workspace root.*
*Priority: Critical. From: arch_review_stride + code_review_thorough.*
*Link: [./tracks/sec_path_traversal_20260309/](./tracks/sec_path_traversal_20260309/)*

---

### High Security

## [x] Track: Security — Require User Confirmation Before Running initializeCommand on Host
*Show `initializeCommand` to the user and require `y` confirmation (or `--trust` flag) before executing on the host. Mirrors VS Code Dev Containers behavior.*
*Priority: High. From: arch_review_stride.*
*Link: [./tracks/sec_host_hook_confirmation_20260309/](./tracks/sec_host_hook_confirmation_20260309/)*

---

## [x] Track: Security — Warn When Container Runs as Root With No remoteUser Configured
*Inspect container user after creation; emit a prominent warning when root with no remoteUser set. Non-fatal. Suppressible with `--no-root-check`.*
*Priority: High. From: arch_review_stride.*
*Link: [./tracks/sec_default_user_20260309/](./tracks/sec_default_user_20260309/)*

---

### High Correctness

## [x] Track: Fix — Propagate Non-Zero Exit From In-Container Lifecycle Hooks
*Check exec_hook return value for postCreateCommand, postStartCommand, postAttachCommand in run() and rebuild(). Abort with named error on Ok(false).*
*Priority: High. From: code_review_thorough.*
*Link: [./tracks/fix_hook_exit_propagation_20260309/](./tracks/fix_hook_exit_propagation_20260309/)*

---

## [x] Track: Fix — Container Name Filter in exists() and running() is Too Broad
*`--filter name=` is a substring match; use anchored regex or exact comparison. Fixes false-positives for containers with similar names.*
*Priority: High. From: code_review_thorough.*
*Link: [./tracks/fix_container_name_filter_20260309/](./tracks/fix_container_name_filter_20260309/)*

---

## [x] Track: Fix — Dockerfile Path Resolution Asymmetry Between Docker and Podman
*Extract `resolve_dockerfile_path` helper and use it uniformly across all four providers. Relative paths must resolve relative to build.context.*
*Priority: High. From: code_review_thorough.*
*Link: [./tracks/fix_dockerfile_path_asymmetry_20260309/](./tracks/fix_dockerfile_path_asymmetry_20260309/)*

---

## [x] Track: Fix — DockerCompose exists() Uses Wrong Container Name Source
*Use `docker compose ps` output rather than a guessed name to detect container existence in DockerCompose and PodmanCompose.*
*Priority: High. From: code_review_thorough.*
*Link: [./tracks/fix_docker_compose_exists_20260309/](./tracks/fix_docker_compose_exists_20260309/)*

---

### Medium Security

## [x] Track: Security — Sanitise Container Names and Label Values Used in Shell Commands
*Validate safe_name() output and remoteEnv key/value formats against strict character set rules. Reject invalid names with clear errors.*
*Priority: Medium. From: code_review_thorough.*
*Link: [./tracks/sec_input_sanitisation_20260309/](./tracks/sec_input_sanitisation_20260309/)*

---

## [x] Track: Security — Redact Sensitive Values From Logged Commands
*In print_command, replace `--env KEY=VALUE` values with `***` in printed output. Actual subprocess args are not modified.*
*Priority: Medium. From: code_review_thorough.*
*Link: [./tracks/sec_command_redaction_20260309/](./tracks/sec_command_redaction_20260309/)*

---

## [x] Track: Security — Add SELinux Label Support for Podman SSH Socket Mount
*Auto-detect SELinux enforcing mode; append `:z` to SSH socket volume mount in compose override. Override via `selinuxRelabel` config field.*
*Priority: Medium. From: arch_review_stride.*
*Link: [./tracks/sec_podman_selinux_20260309/](./tracks/sec_podman_selinux_20260309/)*

---

### Medium Correctness

## [x] Track: Fix — Podman Providers Missing mounts and remoteEnv Support
*Implement `mounts` and `remoteEnv` in Podman::create() and PodmanCompose override template, matching Docker provider behavior.*
*Priority: Medium. From: code_review_thorough.*
*Link: [./tracks/fix_podman_mounts_parity_20260309/](./tracks/fix_podman_mounts_parity_20260309/)*

---

### Medium Architecture

## [ ] Track: Refactor — Introduce Typed Error Enum Instead of io::Error for All Failures
*Replace all `io::Error::new(Other, ...)` call sites with named `Error` enum variants. Add HookFailed, InvalidConfig, ProviderError, PathError variants.*
*Priority: Medium. From: code_review_thorough + arch_review_stride.*
*Link: [./tracks/refactor_error_taxonomy_20260309/](./tracks/refactor_error_taxonomy_20260309/)*

---

## [ ] Track: Refactor — Provider Trait exec Methods Return Result<()>
*Change exec() and exec_raw() to return Result<()> (Err on non-zero exit) instead of Result<bool>. Simplifies callers via ? propagation.*
*Priority: Medium. From: code_review_thorough.*
*Link: [./tracks/refactor_provider_result_api_20260309/](./tracks/refactor_provider_result_api_20260309/)*

---

## [ ] Track: Refactor — Extract ContainerOptions Struct
*Extract ContainerOptions from inline argument assembly in run()/rebuild(). All four provider create() methods accept &ContainerOptions.*
*Priority: Medium. From: arch_review_stride.*
*Link: [./tracks/refactor_create_options_20260309/](./tracks/refactor_create_options_20260309/)*

---

## [ ] Track: Feature — Structured Audit Log for All Container Lifecycle Events
*Append-only NDJSON audit log at $XDG_DATA_HOME/devcont/audit.log (mode 0o600). Captures container_start, hook_executed, command_run (redacted) events.*
*Priority: Medium. From: arch_review_stride.*
*Link: [./tracks/feature_audit_log_20260309/](./tracks/feature_audit_log_20260309/)*

---

### Medium Features

## [ ] Track: Feature — Full build.context and build.args Support for All Providers
*Pass build.context as build root and build.args as --build-arg flags in all four providers. Inject into compose override template.*
*Priority: Medium. From: code_review_thorough + test_coverage_assessment.*
*Link: [./tracks/feature_build_context_20260309/](./tracks/feature_build_context_20260309/)*

---

## [ ] Track: Feature — Configurable Timeout for Lifecycle Hooks
*Add `hookTimeoutSeconds` config field and `--hook-timeout` CLI flag. Abort with named error if any hook exceeds the timeout.*
*Priority: Medium. From: code_review_thorough.*
*Link: [./tracks/feature_hook_timeout_20260309/](./tracks/feature_hook_timeout_20260309/)*

---

### Low Priority

## [x] Track: Refactor — Tighten Provider Module Visibility
*Change pub → pub(crate) or private for all items used only within the crate. Resolves dead_code warnings. No behavioral changes.*
*Priority: Low. From: code_review_thorough.*
*Link: [./tracks/refactor_provider_visibility_20260309/](./tracks/refactor_provider_visibility_20260309/)*

---

## [x] Track: Fix — Settings Load Silently Falls Back on Any Error
*Distinguish "file not found" (Ok default) from "file exists but invalid" (Err with path). Prevents silent config misconfig.*
*Priority: Low. From: code_review_thorough.*
*Link: [./tracks/fix_settings_error_handling_20260309/](./tracks/fix_settings_error_handling_20260309/)*

---

## [x] Track: Fix — safe_name() Silently Truncates Unicode
*Return error with helpful message when workspace path produces empty container name. Prepend "dev-" if name starts with non-alphanumeric.*
*Priority: Low. From: code_review_thorough.*
*Link: [./tracks/fix_safe_name_validation_20260309/](./tracks/fix_safe_name_validation_20260309/)*

---

### Test Coverage

## [ ] Track: Tests — Devcontainer Lifecycle run(), rebuild(), All Hook Paths
*Unit tests for run() and rebuild() using MockProvider. Happy path + each hook failure mode. Target ≥80% line coverage for mod.rs.*
*Priority: Medium. From: test_coverage_assessment.*
*Link: [./tracks/test_devcontainer_lifecycle_20260309/](./tracks/test_devcontainer_lifecycle_20260309/)*

---

## [ ] Track: Tests — exec_host_hook, OneOrMany, safe_name, and Utility Functions
*Targeted tests for exec_host_hook (real process calls), OneOrMany edge cases (null/integer/nested), safe_name, and Config::parse error paths.*
*Priority: Medium. From: test_coverage_assessment.*
*Link: [./tracks/test_exec_host_hook_and_utils_20260309/](./tracks/test_exec_host_hook_and_utils_20260309/)*

---

## [ ] Track: Tests — PodmanCompose Provider Unit Coverage
*Arg construction and output parsing tests for PodmanCompose. Target ≥70% line coverage for podman_compose.rs.*
*Priority: Medium. From: test_coverage_assessment.*
*Link: [./tracks/test_podman_compose_integration_20260309/](./tracks/test_podman_compose_integration_20260309/)*

---

## [ ] Track: Tests — Config and Settings Full Field Coverage
*One test per devcontainer.json field (all 17). All Settings engine variants. Target ≥85% for config.rs, ≥80% for settings.rs.*
*Priority: Medium. From: test_coverage_assessment.*
*Link: [./tracks/test_config_settings_gaps_20260309/](./tracks/test_config_settings_gaps_20260309/)*

---

### Fuzzing

## [ ] Track: Feature — cargo-fuzz Harnesses for Config and OneOrMany Parsing
*Implement fuzz_config_parse and fuzz_one_or_many targets. Seed corpus from existing fixtures. CI bounded to 60s per target.*
*Priority: Medium. From: fuzzing_assessment.*
*Link: [./tracks/fuzzing_impl_20260309/](./tracks/fuzzing_impl_20260309/)*

---

### Documentation

## [ ] Track: Docs — Fill Rustdoc Gaps on Public and Complex Functions
*Add //! crate doc, /// on print_command/run/rebuild, provider struct docs, Config/Build field docs, exec_host_hook doc. cargo doc --no-deps must be warning-free.*
*Priority: Low. From: documentation_assessment.*
*Link: [./tracks/rustdoc_gaps_20260309/](./tracks/rustdoc_gaps_20260309/)*

---
