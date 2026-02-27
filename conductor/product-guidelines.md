# Product Guidelines: devcont

## Communication Style

- **Tone:** Direct, precise, and developer-friendly. No marketing fluff.
- **Error messages:** Always actionable. Tell the user what went wrong and what to do next.
- **Output:** Minimal by default; verbose mode (`--verbose`) for debugging.
- **Progress:** Print the commands being run (current behavior), formatted clearly.

## CLI UX Principles

- Follow the [Command Line Interface Guidelines](https://clig.dev/) conventions
- Exit codes: `0` for success, non-zero for failure (align with POSIX conventions)
- Flags: use long-form flags in scripts, short flags for interactive use
- Subcommands are the primary interface; bare `devcont` defaults to `start`
- Never silently succeed when an error occurred

## Naming Conventions

- Binary name: `devcont` (the new name replacing `devcon`)
- Cargo subcommand: `cargo devcont`
- Container name prefix: `devcont-<project-name>` (replacing `devcon-`)
- Config directory: follows OS conventions via the `directories` crate

## Documentation Style

- README: concise, focused on installation and quick-start
- Inline doc comments (`///`) on all public items
- Error messages use sentence case, no trailing period
- Man page generated from CLI help text

## Versioning

- Follows [Semantic Versioning](https://semver.org/)
- Breaking spec behavior changes bump the minor version until 1.0
- CHANGELOG maintained for each release
