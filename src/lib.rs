//! Kelper - A Kubernetes image management tool
//!
//! This crate provides functionality for managing and inspecting container images
//! in Kubernetes clusters.

// Public API
pub use cli::Args;
pub use k8s::K8sClient;

// Internal modules
mod cli;
mod k8s;
mod utils;

// Re-export commonly used items
pub use cli::{Commands, GetImages, LogFormat, OutputFormat};
pub use k8s::{extract_registry, process_pod, split_image, K8sError};
pub use utils::display_pod_images;
pub use utils::logging;

/// Result type for Kelper operations
pub type KelperResult<T> = anyhow::Result<T>;

/// Error type for Kelper operations
pub type KelperError = anyhow::Error;
