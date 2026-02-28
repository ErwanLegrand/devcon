//! Integration tests for devcont.
//!
//! Requires a live Docker daemon. Run with:
//!   cargo test --test integration
//!
//! Each test allocates a unique container name (`devcont-itest-<prefix>-<ts>`)
//! and registers RAII guards that force-remove the container and image on drop,
//! so cleanup runs even when a test panics.

use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use devcont::provider::Provider;
use devcont::provider::docker::Docker;
use devcont::provider::docker_compose::DockerCompose;

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

/// RAII guard: calls `docker rmi -f <tag>` when dropped.
struct ImageGuard {
    tag: String,
}

impl ImageGuard {
    fn new(tag: &str) -> Self {
        ImageGuard {
            tag: tag.to_string(),
        }
    }
}

impl Drop for ImageGuard {
    fn drop(&mut self) {
        Command::new("docker")
            .args(["rmi", "-f", &self.tag])
            .output()
            .ok();
    }
}

/// RAII guard: runs `docker compose -p <name> down --remove-orphans --rmi all` when dropped.
struct ComposeGuard {
    name: String,
}

impl ComposeGuard {
    fn new(name: &str) -> Self {
        ComposeGuard {
            name: name.to_string(),
        }
    }
}

impl Drop for ComposeGuard {
    fn drop(&mut self) {
        Command::new("docker")
            .args([
                "compose",
                "-p",
                &self.name,
                "down",
                "--remove-orphans",
                "--rmi",
                "all",
            ])
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

/// Produces a unique container name `devcont-itest-<prefix>-<timestamp_ns>`.
fn unique_name(prefix: &str) -> String {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock error")
        .as_nanos();
    format!("devcont-itest-{prefix}-{ts}")
}

/// Build a Docker image from a fixture directory.
///
/// Uses the Dockerfile at `<fixture_dir>/Dockerfile` with the fixture
/// directory as build context. Returns the image tag `devcont/<name>`.
fn build_image(fixture: &Path, name: &str) -> String {
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

/// Build, create, and start a container from a fixture.
///
/// Returns `(image_tag, ContainerGuard, ImageGuard)`. The guards ensure
/// cleanup of both the container and image even when the test panics.
fn start_fixture_container(fixture_name: &str, name: &str) -> (String, ContainerGuard, ImageGuard) {
    let fixture = fixture_path(fixture_name);
    let image = build_image(&fixture, name);
    let container_guard = ContainerGuard::new(name);
    let image_guard = ImageGuard::new(&image);

    let status = Command::new("docker")
        .args([
            "create",
            "--name",
            name,
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
        .args(["start", name])
        .status()
        .expect("docker start failed to start");
    assert!(status.success(), "docker start failed");

    (image, container_guard, image_guard)
}

/// Construct a `Docker` provider pointed at the `basic` fixture with the given container name.
fn load_docker_provider(name: &str) -> Docker {
    Docker {
        build_args: std::collections::HashMap::new(),
        command: "docker".to_string(),
        directory: fixture_path("basic").to_string_lossy().into_owned(),
        file: fixture_path("basic")
            .join("Dockerfile")
            .to_string_lossy()
            .into_owned(),
        forward_ports: vec![],
        name: name.to_string(),
        run_args: vec![],
        mounts: None,
        user: "root".to_string(),
        workspace_folder: "/workspace".to_string(),
        override_command: true,
    }
}

/// Construct a `DockerCompose` provider pointed at the `compose` fixture with the given project name.
fn load_compose_provider(name: &str) -> DockerCompose {
    DockerCompose {
        build_args: std::collections::HashMap::new(),
        command: "docker".to_string(),
        directory: fixture_path("compose").to_string_lossy().into_owned(),
        file: fixture_path("compose")
            .join("docker-compose.yml")
            .to_string_lossy()
            .into_owned(),
        name: name.to_string(),
        forward_ports: vec![],
        run_args: vec![],
        service: "app".to_string(),
        user: "root".to_string(),
        // alpine does not have /workspace; use /tmp which always exists.
        workspace_folder: "/tmp".to_string(),
    }
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
    let _container_guard = ContainerGuard::new(&name);
    let fixture = fixture_path("basic");
    let image = build_image(&fixture, &name);
    let _image_guard = ImageGuard::new(&image);

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
}

/// Start a container, exec a command inside it, and assert success.
/// Verifies `docker start` + `docker exec` work.
#[test]
fn test_exec_in_container() {
    let name = unique_name("exec");
    let (_image, _container_guard, _image_guard) = start_fixture_container("basic", &name);

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
}

/// Simulate a `postCreateCommand` lifecycle hook: exec a command that writes
/// a marker file, then verify the file exists in the container.
#[test]
fn test_post_create_command() {
    let name = unique_name("postcreate");
    let (_image, _container_guard, _image_guard) = start_fixture_container("post_create", &name);

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
}

// ---------------------------------------------------------------------------
// Docker provider tests
// ---------------------------------------------------------------------------

/// `exists()` returns `false` for a container that has never been created.
#[test]
fn test_docker_exists_returns_false_before_create() {
    let name = unique_name("docker-pre");
    let provider = load_docker_provider(&name);
    assert!(
        !provider.exists().expect("exists() failed"),
        "exists() should be false before create"
    );
}

/// `build()` + `create()` succeed and `exists()` returns `true` afterwards.
#[test]
fn test_docker_build_and_create() {
    let name = unique_name("docker-create");
    let provider = load_docker_provider(&name);
    let image = format!("devcont/{name}");
    let _container_guard = ContainerGuard::new(&name);
    let _image_guard = ImageGuard::new(&image);

    assert!(
        provider.build(true).expect("build() failed"),
        "build() should succeed"
    );
    assert!(
        provider.create(vec![]).expect("create() failed"),
        "create() should succeed"
    );
    assert!(
        provider.exists().expect("exists() failed"),
        "exists() should be true after create"
    );
}

/// `start()` succeeds and `running()` returns `true` afterwards.
#[test]
fn test_docker_start_and_running() {
    let name = unique_name("docker-start");
    let provider = load_docker_provider(&name);
    let image = format!("devcont/{name}");
    let _container_guard = ContainerGuard::new(&name);
    let _image_guard = ImageGuard::new(&image);

    provider.build(true).expect("build() failed");
    provider.create(vec![]).expect("create() failed");
    assert!(
        provider.start().expect("start() failed"),
        "start() should succeed"
    );
    assert!(
        provider.running().expect("running() failed"),
        "running() should be true after start"
    );
}

/// `stop()` succeeds and `running()` returns `false` afterwards.
#[test]
fn test_docker_running_returns_false_when_stopped() {
    let name = unique_name("docker-stop");
    let provider = load_docker_provider(&name);
    let image = format!("devcont/{name}");
    let _container_guard = ContainerGuard::new(&name);
    let _image_guard = ImageGuard::new(&image);

    provider.build(true).expect("build() failed");
    provider.create(vec![]).expect("create() failed");
    provider.start().expect("start() failed");
    assert!(
        provider.stop().expect("stop() failed"),
        "stop() should succeed"
    );
    assert!(
        !provider.running().expect("running() failed"),
        "running() should be false after stop"
    );
}

/// `restart()` succeeds on a running container and it remains running.
#[test]
fn test_docker_restart() {
    let name = unique_name("docker-restart");
    let provider = load_docker_provider(&name);
    let image = format!("devcont/{name}");
    let _container_guard = ContainerGuard::new(&name);
    let _image_guard = ImageGuard::new(&image);

    provider.build(true).expect("build() failed");
    provider.create(vec![]).expect("create() failed");
    provider.start().expect("start() failed");
    assert!(
        provider.restart().expect("restart() failed"),
        "restart() should succeed"
    );
    assert!(
        provider.running().expect("running() failed"),
        "running() should be true after restart"
    );
}

/// `exec()` runs a command inside the container and returns `true` on success.
#[test]
fn test_docker_exec() {
    let name = unique_name("docker-exec");
    let provider = load_docker_provider(&name);
    let image = format!("devcont/{name}");
    let _container_guard = ContainerGuard::new(&name);
    let _image_guard = ImageGuard::new(&image);

    provider.build(true).expect("build() failed");
    provider.create(vec![]).expect("create() failed");
    provider.start().expect("start() failed");
    assert!(
        provider
            .exec("echo hello".to_string())
            .expect("exec() failed"),
        "exec() should succeed"
    );
}

/// `cp()` copies a host file into the container; the file is then present.
#[test]
fn test_docker_cp() {
    let name = unique_name("docker-cp");
    let provider = load_docker_provider(&name);
    let image = format!("devcont/{name}");
    let _container_guard = ContainerGuard::new(&name);
    let _image_guard = ImageGuard::new(&image);

    provider.build(true).expect("build() failed");
    provider.create(vec![]).expect("create() failed");
    provider.start().expect("start() failed");

    // Write a temp file into the workspace tmp/ directory.
    let tmp_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tmp");
    std::fs::create_dir_all(&tmp_dir).expect("failed to create tmp/");
    let src = tmp_dir.join(format!("cp_test_{name}.txt"));
    std::fs::write(&src, b"hello from host").expect("failed to write temp file");

    let dest = "/tmp/cp_test_file.txt".to_string();
    assert!(
        provider
            .cp(src.to_string_lossy().into_owned(), dest.clone())
            .expect("cp() failed"),
        "cp() should succeed"
    );
    assert!(
        provider
            .exec(format!("test -f {dest}"))
            .expect("exec() failed"),
        "file should exist in container after cp"
    );

    // Clean up host temp file.
    std::fs::remove_file(&src).ok();
}

/// `stop()` + `rm()` remove the container; `exists()` returns `false`.
#[test]
fn test_docker_stop_and_rm() {
    let name = unique_name("docker-rm");
    let provider = load_docker_provider(&name);
    let image = format!("devcont/{name}");
    // ImageGuard still needed; ContainerGuard is a safety net but rm() is explicit here.
    let _container_guard = ContainerGuard::new(&name);
    let _image_guard = ImageGuard::new(&image);

    provider.build(true).expect("build() failed");
    provider.create(vec![]).expect("create() failed");
    provider.start().expect("start() failed");
    provider.stop().expect("stop() failed");
    assert!(provider.rm().expect("rm() failed"), "rm() should succeed");
    assert!(
        !provider.exists().expect("exists() failed"),
        "exists() should be false after rm"
    );
}

// ---------------------------------------------------------------------------
// DockerCompose provider tests
// ---------------------------------------------------------------------------

/// `exists()` returns `false` for a compose project that has never been started.
#[test]
fn test_compose_exists_returns_false_before_build() {
    let name = unique_name("compose-pre");
    let provider = load_compose_provider(&name);
    assert!(
        !provider.exists().expect("exists() failed"),
        "exists() should be false before build"
    );
}

/// `build()` + `start()` succeed; `exists()` and `running()` both return `true`.
#[test]
fn test_compose_build_and_start() {
    let name = unique_name("compose-start");
    let provider = load_compose_provider(&name);
    let _guard = ComposeGuard::new(&name);

    assert!(
        provider.build(true).expect("build() failed"),
        "build() should succeed"
    );
    assert!(
        provider.start().expect("start() failed"),
        "start() should succeed"
    );
    assert!(
        provider.exists().expect("exists() failed"),
        "exists() should be true after start"
    );
    assert!(
        provider.running().expect("running() failed"),
        "running() should be true after start"
    );
}

/// `exec()` runs a command in the service container.
#[test]
fn test_compose_exec() {
    let name = unique_name("compose-exec");
    let provider = load_compose_provider(&name);
    let _guard = ComposeGuard::new(&name);

    provider.build(true).expect("build() failed");
    provider.start().expect("start() failed");
    assert!(
        provider
            .exec("echo hello".to_string())
            .expect("exec() failed"),
        "exec() should succeed"
    );
}

/// `cp()` copies a host file into the service container.
#[test]
fn test_compose_cp() {
    let name = unique_name("compose-cp");
    let provider = load_compose_provider(&name);
    let _guard = ComposeGuard::new(&name);

    provider.build(true).expect("build() failed");
    provider.start().expect("start() failed");

    let tmp_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tmp");
    std::fs::create_dir_all(&tmp_dir).expect("failed to create tmp/");
    let src = tmp_dir.join(format!("compose_cp_{name}.txt"));
    std::fs::write(&src, b"hello from host").expect("failed to write temp file");

    let dest = "/tmp/compose_cp_file.txt".to_string();
    assert!(
        provider
            .cp(src.to_string_lossy().into_owned(), dest.clone())
            .expect("cp() failed"),
        "cp() should succeed"
    );
    assert!(
        provider
            .exec(format!("test -f {dest}"))
            .expect("exec() failed"),
        "file should exist in container after cp"
    );

    std::fs::remove_file(&src).ok();
}

/// `restart()` succeeds and the service remains running.
#[test]
fn test_compose_restart() {
    let name = unique_name("compose-restart");
    let provider = load_compose_provider(&name);
    let _guard = ComposeGuard::new(&name);

    provider.build(true).expect("build() failed");
    provider.start().expect("start() failed");
    assert!(
        provider.restart().expect("restart() failed"),
        "restart() should succeed"
    );
    assert!(
        provider.running().expect("running() failed"),
        "running() should be true after restart"
    );
}

/// `stop()` + `rm()` shut down the project; `exists()` returns `false`.
#[test]
fn test_compose_stop_and_rm() {
    let name = unique_name("compose-rm");
    let provider = load_compose_provider(&name);
    // ComposeGuard is a safety net; rm() is called explicitly here.
    let _guard = ComposeGuard::new(&name);

    provider.build(true).expect("build() failed");
    provider.start().expect("start() failed");
    provider.stop().expect("stop() failed");
    assert!(provider.rm().expect("rm() failed"), "rm() should succeed");
    assert!(
        !provider.exists().expect("exists() failed"),
        "exists() should be false after rm"
    );
}
