# Plan: Feature — cargo-fuzz Harnesses for Config and OneOrMany

## Phase 1: Setup

- [ ] Task: Install nightly toolchain if not present
    - [ ] `rustup toolchain add nightly`
- [ ] Task: Initialise fuzz workspace
    - [ ] `cargo fuzz init` (creates `fuzz/Cargo.toml` and `fuzz/fuzz_targets/`)
- [ ] Task: Expose `Config::parse_str` helper
    - [ ] Add `pub fn parse_str(s: &str) -> Result<Config>` to `src/devcontainers/config.rs`
    - [ ] `cargo build` must pass
- [ ] Task: Add `devcont` and `json5` as fuzz dependencies
    - [ ] Add to `fuzz/Cargo.toml`: `devcont = { path = ".." }` and `json5 = "0.4"`

## Phase 2: Harness Implementation

- [ ] Task: Implement `fuzz/fuzz_targets/fuzz_config_parse.rs`
    - [ ] Receive bytes, UTF-8 decode, call `Config::parse_str`, must not panic
    - [ ] `cargo +nightly fuzz build fuzz_config_parse` must succeed
- [ ] Task: Implement `fuzz/fuzz_targets/fuzz_one_or_many.rs`
    - [ ] Receive bytes, call `json5::from_str::<OneOrMany>`, must not panic
    - [ ] `cargo +nightly fuzz build fuzz_one_or_many` must succeed
- [ ] Task: Seed corpus files
    - [ ] `fuzz/corpus/fuzz_config_parse/`: copy three fixture JSON files
    - [ ] `fuzz/corpus/fuzz_one_or_many/`: three seed strings (string, array, empty)
    - [ ] Commit: `feat(fuzz): implement cargo-fuzz harnesses for Config and OneOrMany parsing`

## Phase 3: Local Run and CI

- [ ] Task: Run each target locally for 10 minutes
    - [ ] `cargo +nightly fuzz run fuzz_config_parse -- -max_total_time=600`
    - [ ] `cargo +nightly fuzz run fuzz_one_or_many -- -max_total_time=600`
    - [ ] No crashes or panics found
- [ ] Task: Add CI step
    - [ ] 60-second bound: `cargo +nightly fuzz run <target> -- -max_total_time=60 -runs=100000`
    - [ ] Upload crash artifacts with `actions/upload-artifact`
- [ ] Task: Write `fuzz/README.md` documenting crash triage process
    - [ ] Commit: `ci(fuzz): add bounded fuzz run steps and crash artifact upload`

## Phase 4: Quality Gate

- [ ] Task: Verify fuzz targets build cleanly on nightly
    - [ ] `cargo +nightly fuzz build`
    - [ ] No warnings on `cargo clippy` for main crate
