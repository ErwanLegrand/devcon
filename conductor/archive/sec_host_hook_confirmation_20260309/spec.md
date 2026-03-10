# Spec: Security — Require User Confirmation Before Running initializeCommand on Host

## Problem

`initializeCommand` executes arbitrary shell commands on the **host machine** (not inside the
container) before the container is started. Any cloned repository with a `devcontainer.json` can
run host commands automatically when `devcont start` is invoked, with no user warning or consent.

This is a supply-chain / social-engineering attack vector: a malicious repo can exfiltrate SSH
keys, install backdoors, or delete files on the developer's machine.

STRIDE classification: **Elevation of Privilege / Tampering** (High).

VS Code Dev Containers prompts the user before running `initializeCommand` for the same reason.

## Goal

Show the `initializeCommand` value to the user and require explicit confirmation (yes/no) before
executing it. Provide a `--trust` / `--yes` flag that skips the prompt for CI / scripted use.

## Functional Requirements

- FR-001: Before calling `exec_host_hook` for `initializeCommand`, print the command(s) to
  stderr and prompt: `"Run initializeCommand on host? [y/N] "`.
- FR-002: Default to **No** (safe default). Only proceed on `y` or `Y` input.
- FR-003: If the user declines, abort with a non-zero exit and a clear message.
- FR-004: Add a `--trust` flag to the `start` and `rebuild` subcommands that skips the prompt
  (for CI pipelines / `devcontainer.json` repositories the user trusts).
- FR-005: When `--trust` is active, log a one-line notice that the hook is being run without
  prompting (so CI logs are auditable).
- FR-006: Unit tests: prompt is shown when `--trust` is absent, skipped when present; decline
  input aborts; accept input proceeds.

## Out of Scope

- Confirming `postCreateCommand`, `postStartCommand`, `postAttachCommand` (these run inside the
  container where the user is already at lower risk).
- Storing trust decisions across invocations (future: `.devcontainer/trusted` lockfile).
