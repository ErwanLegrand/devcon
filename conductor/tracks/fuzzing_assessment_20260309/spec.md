# Spec: Fuzzing Feasibility Assessment

## Objective

Assess whether adding fuzz testing to `devcont` is worthwhile, identify the most valuable fuzz
targets, evaluate the implementation effort, and produce a go/no-go recommendation with a
concrete implementation plan if the answer is yes.

## Background

The tech stack already lists `cargo-fuzz` (libFuzzer) as planned. The question is: which entry
points are genuinely worth fuzzing, and what is the expected return on the implementation cost?

## Assessment Criteria

### 1. Attack Surface & Input Sources

Identify all points where external/untrusted data enters the system:
- `devcontainer.json` parsing (`Config::parse`) — JSON5 input from the workspace
- Dockerfile paths and image names from config — used to construct shell commands
- Compose file paths — passed to docker-compose
- Remote env vars (`remoteEnv`) — injected into compose override
- Lifecycle hook values (`OneOrMany` fields) — passed to `exec` / `exec_raw`
- User settings file (`~/.config/devcont/config.toml`) — TOML input

### 2. Fuzzing ROI Analysis

For each candidate target, rate:
- **Bug density potential** — how complex is the parsing/processing logic? Could it panic?
- **Security impact** — if a crafted input causes misbehaviour, what's the blast radius?
- **Existing coverage** — how well are edge cases already handled by unit/integration tests?
- **Fuzz effectiveness** — is this a domain where fuzzing finds bugs that tests miss? (e.g. binary parsers, complex state machines)
- **Implementation effort** — how much glue code is needed to write the fuzz harness?

### 3. Candidate Fuzz Targets

Evaluate at minimum:
- `fuzz_config_parse` — feed arbitrary bytes as JSON5 to `Config::parse`; verify no panic, only `Err` on invalid input
- `fuzz_one_or_many` — feed arbitrary JSON to `OneOrMany` deserialiser
- `fuzz_compose_override` — feed arbitrary `service` strings and env var key/value pairs to `create_compose_override`; verify no panic, valid YAML output or error
- `fuzz_exec_hook_dispatch` — feed arbitrary `OneOrMany` values to `exec_hook`; verify no panic

### 4. Infrastructure Requirements

- Does `cargo-fuzz` work in the current dev container environment?
- Is libFuzzer available on the CI runners?
- What corpus management strategy is appropriate (seed corpus, coverage-guided, persistent)?
- How should fuzz failures be reported and reproduced?

## Deliverable

A markdown report saved to `conductor/tracks/fuzzing_assessment_20260309/report.md`:

1. **Recommendation** — Go / No-Go / Partial (with rationale)
2. **Candidate target evaluation table** — one row per target: target name, bug density,
   security impact, existing coverage, fuzz effectiveness, effort, final verdict (Implement/Skip)
3. **Implementation plan** (if Go/Partial) — step-by-step plan to add `cargo-fuzz` harnesses
   for the selected targets, including corpus seeding and CI integration
4. **Follow-up track stub** — a suggested conductor track for the implementation work

## Output Artefact

`conductor/tracks/fuzzing_assessment_20260309/report.md`
