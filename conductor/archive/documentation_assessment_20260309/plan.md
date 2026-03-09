# Plan: Documentation Assessment & README Overhaul

## Phase 1: Audit

- [x] Task: Audit existing README and source rustdoc
    - [x] Read current `README.md` — noted outdated podman-compose status, missing fields, verbosity
    - [x] Read all `pub` items in `src/` and check for doc comment presence and quality
    - [x] Listed 12 rustdoc gaps (provider structs, print_command, crate-level //!, command run fns)
    - [x] Checked `CLAUDE.md` for accuracy

## Phase 2: README Rewrite

- [x] Task: Write the new `README.md`
    - [x] 70 lines (≤ 80 target met)
    - [x] One-sentence description + attribution link to `guitsaru/devcon` + beta notice
    - [x] Installation section (binary + cargo install)
    - [x] Usage with optional `[dir]` argument documented
    - [x] All four engines shown as supported (Docker, Podman, docker-compose, podman-compose)
    - [x] Full devcontainer.json field reference table (17 fields)
    - [x] SSH agent section retained
    - [x] Commit: `docs: overhaul README — concise, accurate, upstream attribution` [3063dcd]

## Phase 3: Report & Rustdoc Gap List

- [x] Task: Write report to `conductor/tracks/documentation_assessment_20260309/report.md`
    - [x] Summary of README changes
    - [x] 12 rustdoc gap items in table form
    - [x] CHANGELOG/CONTRIBUTING: skip until first stable release
    - [x] Commit: `docs(conductor): add documentation assessment report` [3063dcd]

- [ ] Task: Conductor - User Manual Verification 'Documentation Assessment' (Protocol in workflow.md)
