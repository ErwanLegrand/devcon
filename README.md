# Devcont

Run [devcontainers](https://containers.dev/) outside of Visual Studio Code, from your terminal.

Forked from [`guitsaru/devcon`](https://github.com/guitsaru/devcon). **Beta — not ready for production use.**

## Installation

Binary: [releases page](https://github.com/guitsaru/devcon/releases).
From source: `cargo install --path .`

## Usage

Run from a project that contains `.devcontainer/devcontainer.json` (or `.devcontainer.json`):

```sh
devcont                  # start (or resume) the container
devcont rebuild          # destroy and rebuild the container
devcont rebuild --no-cache  # rebuild without layer cache
```

Both commands accept an optional `[dir]` argument to target a different directory.

## SSH Agent

`devcont` forwards your SSH agent socket into the container via `$SSH_AUTH_SOCK`
automatically — no key copying needed.

## Configuration

`~/.config/devcont/config.toml`:

```toml
# "docker" (default) or "podman"
provider = "docker"

# Dotfiles to copy into the container (paths relative to $HOME)
dotfiles = [".zshrc", ".config/nvim"]
```

`~/.gitconfig` is always copied when present.

## Supported Engines

docker, docker-compose, podman, podman-compose

## Supported `devcontainer.json` Fields

| Field | Notes |
|---|---|
| `name` | required |
| `image` | pull a pre-built image |
| `build.dockerfile`, `build.args` | build from a Dockerfile |
| `forwardPorts` | host↔container port mapping |
| `remoteEnv` | environment variables injected at runtime |
| `remoteUser` | user inside the container (default: `root`) |
| `workspaceFolder` | working directory (default: `/workspace`) |
| `dockerComposeFile` + `service` | compose mode |
| `runArgs` | extra `docker/podman create` arguments |
| `overrideCommand` | keep container alive with a sleep loop |
| `shutdownAction` | `none` / `stopContainer` / `stopCompose` |
| `mounts` | additional bind/volume mounts |
| `initializeCommand` | host — before container creation |
| `onCreateCommand` | container — after first create |
| `updateContentCommand` | container — after content update |
| `postCreateCommand` | container — after first create (post-setup) |
| `postStartCommand` | container — after each start |
| `postAttachCommand` | container — after each attach |

Hooks accept a string (`sh -c` form) or an array (no shell interpolation).
