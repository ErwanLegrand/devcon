# Plan: Documentation Assessment & README Overhaul

## Phase 1: Audit

- [ ] Task: Audit existing README and source rustdoc
    - [ ] Read current `README.md` — note what is accurate, outdated, missing, or verbose
    - [ ] Read all `pub` items in `src/` and check for doc comment presence and quality
    - [ ] List rustdoc gaps: file, item, gap type (missing/inaccurate/incomplete `# Errors`/`# Panics`)
    - [ ] Check `CLAUDE.md` for accuracy

## Phase 2: README Rewrite

- [ ] Task: Write the new `README.md`
    - [ ] Must be ≤ ~80 lines
    - [ ] Open with one-sentence description + upstream attribution link to `guitsaru/devcon`
    - [ ] Installation section (binary download + build-from-source)
    - [ ] Quick-start / Usage section (`devcont`, `devcont rebuild`)
    - [ ] Provider configuration section (Docker vs Podman)
    - [ ] Supported `devcontainer.json` fields section (image, build, lifecycle hooks, forwardPorts, remoteEnv, etc.)
    - [ ] SSH agent section (keep, it's useful)
    - [ ] Beta notice
    - [ ] `cargo build` must still pass after the edit (README changes don't break it, but verify)
    - [ ] Commit: `docs: overhaul README — concise, accurate, upstream attribution`

## Phase 3: Report & Rustdoc Gap List

- [ ] Task: Write report to `conductor/tracks/documentation_assessment_20260309/report.md`
    - [ ] Summary of README changes
    - [ ] Rustdoc gap table
    - [ ] CHANGELOG/CONTRIBUTING recommendation
    - [ ] Commit: `docs(conductor): add documentation assessment report`

- [ ] Task: Conductor - User Manual Verification 'Documentation Assessment' (Protocol in workflow.md)
