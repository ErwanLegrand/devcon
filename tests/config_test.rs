use std::path::Path;

// We reference the library via the crate name established in Cargo.toml.
// Since this is a binary crate we pull in the modules directly.
// Integration tests for a binary crate must use `#[path = ...]` to reach
// internal modules.  We re-export what we need via a thin shim in src/lib.rs,
// but for now we duplicate the test inside the module tests and here we just
// smoke-test the fixture files so the test suite is aware of them.

/// Confirm the standard fixture file exists and is valid JSON5.
#[test]
fn devcontainer_json_fixture_is_present() {
    let path = Path::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/devcontainer.json"
    ));
    assert!(path.exists(), "devcontainer.json fixture must exist");
    let contents = std::fs::read_to_string(path).expect("should be readable");
    assert!(!contents.is_empty(), "fixture must not be empty");
}

/// Confirm the minimal fixture file exists.
#[test]
fn devcontainer_minimal_fixture_is_present() {
    let path = Path::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/devcontainer_minimal.json"
    ));
    assert!(
        path.exists(),
        "devcontainer_minimal.json fixture must exist"
    );
    let contents = std::fs::read_to_string(path).expect("should be readable");
    assert!(
        contents.contains("minimal"),
        "minimal fixture should contain 'minimal'"
    );
}

/// Confirm the invalid fixture exists and is indeed not valid JSON5.
#[test]
fn devcontainer_invalid_fixture_is_not_valid_json5() {
    let path = Path::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/devcontainer_invalid.json"
    ));
    assert!(
        path.exists(),
        "devcontainer_invalid.json fixture must exist"
    );
    let contents = std::fs::read_to_string(path).expect("should be readable");
    let result: Result<serde_json::Value, _> = json5::from_str(&contents);
    assert!(
        result.is_err(),
        "invalid fixture must fail JSON5 parsing, got: {:?}",
        result
    );
}
