# Kelper

[![Crates.io Version](https://img.shields.io/crates/v/kelper)](https://crates.io/crates/kelper) [![Crates.io Downloads](https://img.shields.io/crates/d/kelper)](https://crates.io/crates/kelper) [![release](https://github.com/aliabbasjaffri/kelper/actions/workflows/release.yml/badge.svg)](https://github.com/aliabbasjaffri/kelper/actions/workflows/release.yml) [![GitHub release (latest by date)](https://img.shields.io/github/v/release/aliabbasjaffri/kelper)](https://github.com/aliabbasjaffri/kelper/releases/latest) [![License](https://img.shields.io/crates/l/kelper)](https://github.com/aliabbasjaffri/kelper/blob/main/LICENSE) [![Project Status: Active](https://www.repostatus.org/badges/latest/active.svg)](https://www.repostatus.org/#active) [![GitHub last commit](https://img.shields.io/github/last-commit/aliabbasjaffri/kelper)](https://github.com/aliabbasjaffri/kelper/commits/main)

A CLI tool designed as a swiss-army knife for operations on Kubernetes pods and nodes. Kelper helps you quickly inspect container images, labels, annotations, health metrics from probes, and many other useful functionalities from your Kubernetes clusters, with support for filtering by namespace, node, and pod name.

## Features

- [x] List images in a Kubernetes cluster based on different filters:
  - Filter by namespace
  - Filter by node
  - Filter by pod name
  - Filter by container image registry
- [x] Advanced logging capabilities:
  - Multiple verbosity levels (-v, -vv, -vvv, -vvvv)
  - Support for both plain and JSON log formats
- [ ] Get labels and annotations in a pod, namespace, or node (coming soon)
- [ ] Retrieve health and metrics from pods or nodes (coming soon)

## Installation

Kelper can be installed using several package managers. Choose the one that suits your environment:

### Using Cargo (Rust's Package Manager)

If you have Rust and Cargo installed, you can build and install Kelper directly from the source:

```bash
cargo install kelper
```

### Using Homebrew (macOS)

If you are on macOS and use Homebrew, you can install Kelper via our tap:

```bash
brew tap aliabbasjaffri/kelper
brew install kelper
```

### Using Krew (kubectl Plugin Manager)

If you use `kubectl` and have Krew installed, you can install Kelper as a kubectl plugin:

```bash
kubectl krew install kelper
```

## Usage

### Get image details with multiple filters

```bash
# List Pod Images in a Namespace
kelper get images --namespace default

# List Pod Images on a Specific Node
kelper get images -N node-name
# or
kelper get images --node node-name

# Note: When using the `--node` flag, the `--namespace` parameter is ignored as it will show pods from all namespaces on the specified node.

# List Images for a Specific Pod
kelper get images -p pod-name
# or
kelper get images --pod pod-name

# You can combine filters to get more specific results. For example, to get images for a specific pod on a specific node:
kelper get images -N node-name -p pod-name

# List images from all namespaces
kelper get images --all-namespaces

# Filter images by registry
kelper get images --registry "docker.io" --namespace kube-system

# Filter images by registry in a specific node
kelper get images --registry "quay.io" --node node-name

# Filter images by registry across all namespaces
kelper get images --registry "quay.io" --all-namespaces

# Enable verbose logging
kelper get images -v  # WARN
kelper get images -vv  # INFO
kelper get images -vvv  # DEBUG
kelper get images -vvvv  # TRACE

# Use JSON log format
kelper get images -vvv --log-format json
```

Kelper displays information in a clean tabular format:

```
kelper get images -o wide
POD                                NAMESPACE  CONTAINER       REGISTRY         IMAGE                          VERSION      DIGEST                                                            NODE
metrics-server-8664d5f5f7-krxm6    default    linkerd-proxy   cr.l5d.io        linkerd/proxy                  edge-25.3.3  496429c2a4a430d7acb4393d01c4d5971a8e3e385e5f47ceaac29dde009e7189  multi-node-cluster-worker
metrics-server-8664d5f5f7-krxm6    default    metrics-server  registry.k8s.io  metrics-server/metrics-server  v0.7.2       ffcb2bf004d6aa0a17d90e0247cf94f2865c8901dcab4427034c341951c239f9  multi-node-cluster-worker
ollama-model-phi-6b7b67778d-np2tx  default    linkerd-proxy   cr.l5d.io        linkerd/proxy                  edge-25.3.3  496429c2a4a430d7acb4393d01c4d5971a8e3e385e5f47ceaac29dde009e7189  multi-node-cluster-worker
ollama-model-phi-6b7b67778d-np2tx  default    server          docker.io        ollama/ollama                  latest       e2c9ab127d555aa671d06d2a48ab58a2e544bbdaf6fa93313dbb4fb8bb73867c  multi-node-cluster-worker
ollama-models-store-0              default    server          docker.io        ollama/ollama                  latest       e2c9ab127d555aa671d06d2a48ab58a2e544bbdaf6fa93313dbb4fb8bb73867c  multi-node-cluster-worker
```

## Development

### Prerequisites

- Rust 1.85 or later
- Kubernetes cluster access
- `kubectl` installed & configured with your cluster

### Building from Source

```bash
# clone kelper project
cd kelper
cargo build --release
```

### Testing

Kelper includes comprehensive tests covering various aspects of the codebase. The tests are organized in the `tests` directory.

To run the tests:

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_process_pod
```

## Releasing

This project uses `cargo-release` to automate the release process, ensuring that the version in `Cargo.toml` and the Git tag are synchronized.

### Prerequisites

1.  Install `cargo-release`:

    ```bash
    cargo install cargo-release
    ```

2.  Ensure your working directory is clean (all changes committed).
3.  Make sure you are on the main branch and have pulled the latest changes.

### Steps

- Run `bash scripts/cargo_release.sh <VERSION>` to update the version in the `Cargo.toml` file and create a Git tag.
- Once that is done, push the code to main, and a release workflow will be triggered which builds multi-platform binaries and distributes them via multiple channels.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
