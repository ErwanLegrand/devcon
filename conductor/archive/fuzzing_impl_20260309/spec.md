# Spec: Feature — Implement cargo-fuzz Harnesses for Config and OneOrMany Parsing

## Problem

The fuzzing feasibility assessment (`fuzzing_assessment_20260309`) recommended implementing two
fuzz targets with high ROI:

1. **`fuzz_config_parse`**: `Config::parse` feeds bytes into the `json5` crate's hand-rolled
   recursive descent parser — exactly the class of code where fuzzing finds panics and
   overflows that structured tests miss.
2. **`fuzz_one_or_many`**: `OneOrMany`'s hand-written serde `Visitor` accepts unexpected types
   (`null`, `integer`, nested arrays) that unit tests don't cover.

## Goal

Set up the `cargo-fuzz` infrastructure and implement both harnesses. Run locally until no crashes
in 10 minutes per target. Add CI integration with bounded time.

## Functional Requirements

- FR-001: Run `cargo fuzz init` to create `fuzz/Cargo.toml` and `fuzz/fuzz_targets/` directory.
- FR-002: Expose `Config::parse_str(s: &str) -> Result<Config>` (thin wrapper over
  `json5::from_str`) as `pub(crate)` or `pub` to avoid filesystem I/O in the harness.
- FR-003: Implement `fuzz/fuzz_targets/fuzz_config_parse.rs`:
  - Receive bytes, convert to UTF-8, call `Config::parse_str`, must not panic.
- FR-004: Implement `fuzz/fuzz_targets/fuzz_one_or_many.rs`:
  - Receive bytes, call `json5::from_str::<OneOrMany>`, must not panic.
- FR-005: Seed corpus for `fuzz_config_parse`: copy three existing fixture JSON files.
- FR-006: Seed corpus for `fuzz_one_or_many`: three forms (string, array, empty array) plus
  degenerate values (`null`, `42`, `true`, `{}`).
- FR-007: Run each target locally: `cargo +nightly fuzz run <target> -- -max_total_time=600`.
  No crashes or panics.
- FR-008: Add CI step (GitHub Actions or equivalent) with 60-second time bound.
- FR-009: Document crash triage process in a `fuzz/README.md` file.

## Out of Scope

- `fuzz_compose_override` and `fuzz_exec_hook_dispatch` (assessed as low ROI, skip).
- Maintaining the fuzz corpus long-term (future).
