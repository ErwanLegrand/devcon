# Fuzzing Feasibility Assessment Report
## date: 2026-03-09

## Recommendation: Partial

Two targets — `fuzz_config_parse` and `fuzz_one_or_many` — have genuine bug-density potential because
they drive third-party parsers (`json5`, serde) over arbitrary byte sequences where testing has
only covered a handful of structured fixtures. The other two candidates either require heavy
harness infrastructure (`fuzz_exec_hook_dispatch`) or produce low incremental value relative to
their effort (`fuzz_compose_override`). Start with the two high-value parser targets; revisit the
remaining candidates after the fuzzing infrastructure is proven.

## Candidate Target Evaluation

| Target | Bug Density | Security Impact | Existing Coverage | Fuzz Effectiveness | Effort | Verdict |
|---|---|---|---|---|---|---|
| fuzz_config_parse | H | H | L | H | L | Implement |
| fuzz_one_or_many | M | M | M | H | L | Implement |
| fuzz_compose_override | L | M | L | L | M | Skip |
| fuzz_exec_hook_dispatch | L | M | M | L | H | Skip |

### Rationale per target

**fuzz_config_parse** — `Config::parse` feeds arbitrary bytes directly into the `json5` crate
followed by serde deserialization. The `json5` crate has a hand-rolled recursive descent parser;
this is exactly the class of code where fuzzing finds panics, stack overflows, and integer
overflows that structured tests miss. Existing tests cover only three fixture files (valid,
invalid, image variant). Bug density and security impact are High because a maliciously crafted
`devcontainer.json` in a workspace could crash the tool before any provider interaction occurs.
The harness is trivial: receive bytes, write to a tempfile, call `Config::parse`. Effort: Low.

**fuzz_one_or_many** — `OneOrMany` uses a hand-written `Visitor` implementation driven by
`deserialize_any`, which means the visitor accepts unexpected types from a deserializer (null,
integer, nested arrays). Existing coverage is moderate — three unit tests cover the string, array,
and empty-array forms, but not integers, booleans, nested arrays, or null values. A fuzzer can
rapidly enumerate these. Harness is a direct `json5::from_str::<OneOrMany>` call. Effort: Low.

**fuzz_compose_override** — The function constructs a `TemplateContext` from the service name and
env vars and passes it to `tinytemplate`. The template itself is a static `include_str!` and
cannot be manipulated by the fuzzer. The service name is embedded as a YAML key but is never
parsed back; tinytemplate is well-tested and unlikely to panic on arbitrary strings. The function
writes to `env::temp_dir()`, making fuzzing produce file I/O side effects. Bug density and fuzz
effectiveness are both Low. Effort is Medium due to managing temp-file cleanup in the harness.

**fuzz_exec_hook_dispatch** — `exec_hook` is a three-branch match over `OneOrMany` that delegates
immediately to `provider.exec` or `provider.exec_raw`; there is no parsing logic to discover
bugs in. A mock provider is already present in `mod.rs` tests but is `cfg(test)`-gated, requiring
re-exposure or duplication for the fuzz crate. The existing four unit tests in `mod.rs` already
exercise all branches including the empty-Many edge case. Fuzz effectiveness and bug density are
both Low. Effort is High due to mock provider extraction.

## Infrastructure Assessment

- cargo-fuzz available: yes (`cargo fuzz list` is installed; no `fuzz/Cargo.toml` exists yet — first-time setup required)
- libFuzzer compatible: yes — the project targets stable Rust 1.85 / edition 2024; `cargo fuzz` requires the nightly toolchain only for compilation of fuzz targets, not for the main crate
- CI integration: feasible — fuzz targets can run in CI with a bounded time limit (`cargo fuzz run <target> -- -max_total_time=60`); crash artifacts are stored as reproducers
- Notes:
  - The project runs on WSL2 (Linux kernel 6.6); libFuzzer is supported on Linux x86-64 without additional setup.
  - No `[profile.fuzz]` or `[[bin]]` fuzz targets exist yet.
  - The `json5` crate (0.4.x) and `serde` are the primary third-party parsers under test — both are candidates for upstream bugs discoverable through this project's harnesses.
  - `create_compose_override` writes to `env::temp_dir()` — this path must be excluded from fuzz harnesses or cleaned up to avoid temp-file accumulation during corpus runs.

## Implementation Plan

### Step 1: Setup

1. Install nightly toolchain if not present: `rustup toolchain add nightly`.
2. Install cargo-fuzz: `cargo install cargo-fuzz`.
3. Initialise the fuzz workspace: `cargo fuzz init` (creates `fuzz/Cargo.toml` and `fuzz/fuzz_targets/`).
4. Add `devcont` as a dependency in `fuzz/Cargo.toml` with `path = ".."` and expose
   `Config::parse_str` (a new `pub(crate)` or `pub` helper that accepts `&str` rather than `&Path`)
   to avoid requiring real filesystem I/O in the harness.

### Step 2: Harness — fuzz_config_parse

File: `fuzz/fuzz_targets/fuzz_config_parse.rs`

```rust
#![no_main]
use libfuzzer_sys::fuzz_target;
use devcont::devcontainers::config::Config;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        // Must not panic; only Ok(_) or Err(_) are acceptable
        let _ = Config::parse_str(s);
    }
});
```

Prerequisite: expose `Config::parse_str(s: &str) -> Result<Config>` (thin wrapper over
`json5::from_str`). This requires making `parse_str` `pub` or `pub(crate)` and gating the
fuzz crate appropriately.

Corpus seed: copy the three existing fixture JSON files into `fuzz/corpus/fuzz_config_parse/`.

### Step 3: Harness — fuzz_one_or_many

File: `fuzz/fuzz_targets/fuzz_one_or_many.rs`

```rust
#![no_main]
use libfuzzer_sys::fuzz_target;
use devcont::devcontainers::one_or_many::OneOrMany;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = json5::from_str::<OneOrMany>(s);
    }
});
```

`OneOrMany` is already `pub`; `json5` must be added to `fuzz/Cargo.toml` as a direct dependency.

Corpus seed: `"\"echo hello\""`, `"[\"npm\",\"install\"]"`, `"[]"` as three small seed files.

### Step 4: CI integration

Add a GitHub Actions workflow step (or equivalent) that runs each fuzz target for a bounded
duration on pull requests:

```yaml
- name: Fuzz (fuzz_config_parse)
  run: cargo +nightly fuzz run fuzz_config_parse -- -max_total_time=60 -runs=100000
- name: Fuzz (fuzz_one_or_many)
  run: cargo +nightly fuzz run fuzz_one_or_many -- -max_total_time=30 -runs=100000
```

Store crash/timeout artifacts using `actions/upload-artifact` keyed on the fuzz target name.

### Corpus seeding strategy

- Use the three existing `tests/fixtures/*.json` files as seed inputs for `fuzz_config_parse`.
- Supplement with hand-crafted edge cases: empty string `""`, bare `{}`, deeply nested objects,
  strings containing null bytes, Unicode surrogates, and oversized string values.
- For `fuzz_one_or_many`: seed with the three forms (string, array, empty array) plus
  degenerate JSON values (`null`, `42`, `true`, `{}`).
- Use coverage-guided fuzzing (`cargo fuzz run`) locally; in CI use `-runs` to bound execution time.

## Follow-up Track Stub

**`fuzzing_impl_20260309`** — Implement cargo-fuzz harnesses for `fuzz_config_parse` and
`fuzz_one_or_many`.

Scope:
1. Add `Config::parse_str` helper (or equivalent visibility adjustment).
2. Initialise `fuzz/` workspace with `cargo fuzz init`.
3. Write and validate both harness files.
4. Seed corpus from existing fixtures.
5. Run locally until no crashes found in 10 minutes per target.
6. Add CI step with 60-second bound.
7. Document crash triage process in `CONTRIBUTING.md`.

Estimated effort: 1–2 days (one engineer). No architectural changes required.
