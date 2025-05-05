# Kelper

[![Crates.io Version](https://img.shields.io/crates/v/kelper)](https://crates.io/crates/kelper) [![Crates.io Downloads](https://img.shields.io/crates/d/kelper)](https://crates.io/crates/kelper) [![release](https://github.com/aliabbasjaffri/kelper/actions/workflows/release.yml/badge.svg)](https://github.com/aliabbasjaffri/kelper/actions/workflows/release.yml) [![GitHub release (latest by date)](https://img.shields.io/github/v/release/aliabbasjaffri/kelper)](https://github.com/aliabbasjaffri/kelper/releases/latest) [![License](https://img.shields.io/crates/l/kelper)](https://github.com/aliabbasjaffri/kelper/blob/main/LICENSE) [![Project Status: Active](https://www.repostatus.org/badges/latest/active.svg)](https://www.repostatus.org/#active) [![GitHub last commit](https://img.shields.io/github/last-commit/aliabbasjaffri/kelper)](https://github.com/aliabbasjaffri/kelper/commits/main)

A CLI tool designed as a swiss-army knife for operations on Kubernetes pods and nodes. Kelper helps you quickly inspect container images, labels, annotations, health metrics from probes, and many other useful functionalities from your Kubernetes clusters, with support for filtering by namespace, node, and pod name.

## Features

- [x] List images in a Kubernetes cluster based on different filters, e.g., image details in a pod, namespace, or node.
- [ ] Get labels and annotations in a pod, namespace, or node.
- [ ] Retrieve health and metrics from pods or nodes.

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
### List Pod Images in a Namespace
kelper get images --namespace default

### List Pod Images on a Specific Node
kelper get images -N node-name
# or
kelper get images --node node-name


# Note: When using the `--node` flag, the `--namespace` parameter is ignored as it will show pods from all namespaces on the specified node.

### List Images for a Specific Pod
kelper get images -p pod-name
# or
kelper get images --pod pod-name

# You can combine filters to get more specific results. For example, to get images for a specific pod on a specific node:
kelper get images -N node-name -p pod-name

# You can also use the `--all-namespaces` flag to list images from all namespaces:
kelper get images --all-namespaces
```

### Get pods using image from a given registry

```bash
### List pods with a specific registry
kelper get pods --registry "docker.io" --namespace kube-system

### List pods with a specific registry in a Node
kelper get pods --registry "quay.io" --node node-name

### List all pods with a specific registry
kelper get pods --registry "quay.io" --all-namespaces
```

Kelper displays information in a clean tabular format:

```
Pod Images and Registries:
================================================================================
+----------------+-----------+------------+------------+---------+-------------+
| Pod Name       | Namespace | Container  | Image Name | Version | Registry    |
+----------------+-----------+------------+------------+---------+-------------+
| nginx-pod      | default   | nginx      | nginx      | latest  | docker.io   |
| redis-pod      | prod      | redis      | redis      | 6.2     | docker.io   |
| api-pod        | staging   | api        | api        | v1.0    | registry.io |
+----------------+-----------+------------+------------+---------+-------------+
================================================================================
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
