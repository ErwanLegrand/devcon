FROM rust:1-slim-bookworm

ARG USER=devcont

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        git \
        curl \
        build-essential \
        pkg-config \
        libssl-dev \
        ca-certificates \
        sudo && \
    rm -rf /var/lib/apt/lists/*

RUN cargo install cargo-llvm-cov cargo-deny

RUN adduser --disabled-password --gecos "" $USER && \
    echo "${USER} ALL=(ALL) NOPASSWD: ALL" >> /etc/sudoers

WORKDIR /workspace

USER $USER
