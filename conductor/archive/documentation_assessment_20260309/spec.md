# Spec: Documentation Assessment & README Overhaul

## Objective

Audit the project's documentation — the README, inline rustdoc, and any other user-facing
text — and produce an overhauled README plus a report on doc-comment gaps. The README must
be concise, actionable, and link to the upstream project we forked from.

## Background

`devcont` is a fork of [`guitsaru/devcon`](https://github.com/guitsaru/devcon). The current
README dates to the original project and does not reflect:
- The rename from `devcon` to `devcont`
- New features: Podman support, image field, lifecycle hooks, OneOrMany hook forms, DooD dev container
- The beta status caveat and project scope
- Where to find the upstream/original project

## Scope

### README.md Overhaul

Requirements for the new README:
1. **Concise** — no more than ~80 lines. Cut anything that can be inferred or is obvious.
2. **Actionable** — lead with installation and usage. Users should be able to run `devcont` within 2 minutes of reading.
3. **Accurate** — reflects the current CLI (`devcont`, `devcont rebuild [--no-cache]`) and `devcontainer.json` fields actually supported.
4. **Upstream link** — clearly state that `devcont` is a fork of [`guitsaru/devcon`](https://github.com/guitsaru/devcon) with a credit/attribution section.
5. **Provider support** — document that both Docker and Podman are supported, with the config option.
6. **Lifecycle hooks** — briefly document the supported `devcontainer.json` hook fields.
7. **Beta notice** — retain the beta/not-production-ready caveat.

### Rustdoc Coverage Assessment

For every `pub` item in `src/` (functions, structs, enums, trait methods), check whether:
- A doc comment exists
- The doc comment is accurate and useful (not a placeholder)
- `# Errors` section is present where the function returns `Result`
- `# Panics` section is present if the function can panic

Produce a gap list: file, item name, gap type (missing/inaccurate/incomplete).

### Other Documentation

- Check `CLAUDE.md` — is it up to date?
- Check if a `CHANGELOG.md` or `CONTRIBUTING.md` is warranted (assess, don't necessarily create)

## Deliverable

1. **Rewritten `README.md`** — the actual file, committed.
2. **Report** at `conductor/tracks/documentation_assessment_20260309/report.md`:
   - Summary of README changes made (before/after comparison in prose)
   - Rustdoc gap table (file, item, gap type, priority)
   - Recommendations for CHANGELOG/CONTRIBUTING (create vs skip)

## Output Artefacts

- `README.md` (updated in-place)
- `conductor/tracks/documentation_assessment_20260309/report.md`
