//! Integration tests for devcont.
//!
//! Requires a live Docker daemon. Run with:
//!   cargo test --test integration
//!
//! Each test allocates a unique container name (`devcont-itest-<prefix>-<ts>`)
//! and registers a RAII guard that force-removes the container on drop, so
//! cleanup runs even when a test panics.

use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// RAII guard: calls `docker rm -f <name>` when dropped.
struct ContainerGuard {
    name: String,
}

impl ContainerGuard {
    fn new(name: &str) -> Self {
        ContainerGuard {
            name: name.to_string(),
        }
    }
}

impl Drop for ContainerGuard {
    fn drop(&mut self) {
        Command::new("docker")
            .args(["rm", "-f", &self.name])
            .output()
            .ok();
    }
}

/// Returns the path to `tests/fixtures/integration/<name>`.
fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("integration")
        .join(name)
}

/// Produces a unique container name `devcont-itest-<prefix>-<timestamp_ms>`.
fn unique_name(prefix: &str) -> String {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock error")
        .as_millis();
    format!("devcont-itest-{prefix}-{ts}")
}

/// Build a Docker image from a fixture directory.
///
/// Uses the Dockerfile at `<fixture_dir>/Dockerfile` with the fixture
/// directory as build context. Returns the image tag `devcont/<name>`.
fn build_image(fixture: &PathBuf, name: &str) -> String {
    let image = format!("devcont/{name}");
    let status = Command::new("docker")
        .args([
            "build",
            "-t",
            &image,
            fixture.to_str().expect("non-utf8 fixture path"),
        ])
        .status()
        .expect("docker build failed to start");
    assert!(status.success(), "docker build failed for {name}");
    image
}

/// Remove an image (best-effort; used for cleanup after tests).
fn remove_image(image: &str) {
    Command::new("docker")
        .args(["rmi", "-f", image])
        .output()
        .ok();
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

/// Verify that Docker is available and the socket is accessible.
#[test]
fn test_docker_available() {
    let status = Command::new("docker")
        .arg("version")
        .status()
        .expect("failed to run docker version");
    assert!(
        status.success(),
        "docker version should succeed — is the socket mounted?"
    );
}

/// Build an image from the `basic` fixture, create a container, and assert it
/// exists. Verifies `docker build` + `docker create` + `docker ps` work.
#[test]
fn test_basic_build_and_create() {
    let name = unique_name("basic");
    let _guard = ContainerGuard::new(&name);
    let fixture = fixture_path("basic");
    let image = build_image(&fixture, &name);

    // Create container with override command so it stays alive
    let status = Command::new("docker")
        .args([
            "create",
            "--name",
            &name,
            "-u",
            "root",
            "-w",
            "/workspace",
            &image,
            "/bin/sh",
            "-c",
            "while sleep 1000; do :; done",
        ])
        .status()
        .expect("docker create failed to start");
    assert!(status.success(), "docker create failed");

    // Verify container exists
    let output = Command::new("docker")
        .args(["ps", "-aq", "--filter", &format!("name=^/{name}$")])
        .output()
        .expect("docker ps failed to start");
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    assert!(!stdout.is_empty(), "container should exist after create");

    remove_image(&image);
}

/// Start a container, exec a command inside it, and assert success.
/// Verifies `docker start` + `docker exec` work.
#[test]
fn test_exec_in_container() {
    let name = unique_name("exec");
    let _guard = ContainerGuard::new(&name);
    let fixture = fixture_path("basic");
    let image = build_image(&fixture, &name);

    // Create and start
    let status = Command::new("docker")
        .args([
            "create",
            "--name",
            &name,
            "-u",
            "root",
            "-w",
            "/workspace",
            &image,
            "/bin/sh",
            "-c",
            "while sleep 1000; do :; done",
        ])
        .status()
        .expect("docker create failed to start");
    assert!(status.success(), "docker create failed");

    let status = Command::new("docker")
        .args(["start", &name])
        .status()
        .expect("docker start failed to start");
    assert!(status.success(), "docker start failed");

    // Exec a simple command
    let status = Command::new("docker")
        .args([
            "exec",
            "-u",
            "root",
            "-w",
            "/workspace",
            &name,
            "sh",
            "-c",
            "echo hello",
        ])
        .status()
        .expect("docker exec failed to start");
    assert!(status.success(), "exec should succeed");

    // Cleanup
    Command::new("docker").args(["stop", &name]).output().ok();
    remove_image(&image);
}

/// Simulate a `postCreateCommand` lifecycle hook: exec a command that writes
/// a marker file, then verify the file exists in the container.
#[test]
fn test_post_create_command() {
    let name = unique_name("postcreate");
    let _guard = ContainerGuard::new(&name);
    let fixture = fixture_path("post_create");
    let image = build_image(&fixture, &name);

    // Create and start
    let status = Command::new("docker")
        .args([
            "create",
            "--name",
            &name,
            "-u",
            "root",
            "-w",
            "/workspace",
            &image,
            "/bin/sh",
            "-c",
            "while sleep 1000; do :; done",
        ])
        .status()
        .expect("docker create failed to start");
    assert!(status.success(), "docker create failed");

    let status = Command::new("docker")
        .args(["start", &name])
        .status()
        .expect("docker start failed to start");
    assert!(status.success(), "docker start failed");

    // Simulate postCreateCommand: touch /tmp/post_create_marker
    let status = Command::new("docker")
        .args(["exec", &name, "sh", "-c", "touch /tmp/post_create_marker"])
        .status()
        .expect("exec postCreateCommand failed to start");
    assert!(status.success(), "postCreateCommand exec failed");

    // Verify marker file exists
    let status = Command::new("docker")
        .args(["exec", &name, "test", "-f", "/tmp/post_create_marker"])
        .status()
        .expect("verify marker failed to start");
    assert!(
        status.success(),
        "marker file should exist after postCreateCommand"
    );

    // Cleanup
    Command::new("docker").args(["stop", &name]).output().ok();
    remove_image(&image);
}
