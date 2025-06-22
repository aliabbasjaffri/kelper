use crate::cli::formats::OutputFormat;
use clap::Subcommand;
use std::path::PathBuf;

/// CLI command structure for Kelper
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Get information about Kubernetes resources
    Get {
        /// The resource type to query
        #[command(subcommand)]
        resource: GetImages,
    },
}

/// Resource types that can be queried in the Kubernetes cluster
#[derive(Subcommand, Debug)]
pub enum GetImages {
    /// List pod images and their registries
    Images {
        /// Kubernetes namespace to query (defaults to "default", ignored when --node is specified)
        #[arg(
            short,
            long,
            default_value = "default",
            conflicts_with = "all_namespaces"
        )]
        namespace: String,

        /// Filter pods by node name
        #[arg(short = 'N', long = "node", conflicts_with = "all_namespaces")]
        node: Option<String>,

        /// Filter pods by pod name
        #[arg(short, long)]
        pod: Option<String>,

        /// Filter pods by container image registry
        #[arg(short = 'R', long = "registry")]
        registry: Option<String>,

        /// Query pods across all namespaces
        #[arg(short = 'A', long = "all-namespaces", conflicts_with = "namespace")]
        all_namespaces: bool,

        /// Output format (default: normal, wide: shows additional columns)
        #[arg(short = 'o', long = "output", default_value = "normal")]
        output: OutputFormat,

        /// Path to kubeconfig file (default: ~/.kube/config)
        #[arg(long = "kubeconfig")]
        kubeconfig: Option<PathBuf>,
    },

    /// List all unique container image registries used in the cluster
    Registries {
        /// Kubernetes namespace to query (defaults to "default")
        #[arg(
            short,
            long,
            default_value = "default",
            conflicts_with = "all_namespaces"
        )]
        namespace: String,

        /// Query pods across all namespaces
        #[arg(short = 'A', long = "all-namespaces", conflicts_with = "namespace")]
        all_namespaces: bool,

        /// Output format (default: normal, wide: shows additional columns)
        #[arg(short = 'o', long = "output", default_value = "normal")]
        output: OutputFormat,

        /// Path to kubeconfig file (default: ~/.kube/config)
        #[arg(long = "kubeconfig")]
        kubeconfig: Option<PathBuf>,
    },
}

impl GetImages {
    /// Get the kubeconfig path for this command
    ///
    /// # Returns
    ///
    /// * `Option<PathBuf>` - The path to the kubeconfig file if specified
    pub fn get_kubeconfig_path(&self) -> Option<PathBuf> {
        match self {
            GetImages::Images { kubeconfig, .. } | GetImages::Registries { kubeconfig, .. } => {
                kubeconfig.clone()
            }
        }
    }

    /// Get the namespace for this command
    ///
    /// # Returns
    ///
    /// * `&str` - The namespace to query
    pub fn get_namespace(&self) -> &str {
        match self {
            GetImages::Images { namespace, .. } | GetImages::Registries { namespace, .. } => {
                namespace
            }
        }
    }

    /// Check if this command should query all namespaces
    ///
    /// # Returns
    ///
    /// * `bool` - True if all namespaces should be queried
    pub fn is_all_namespaces(&self) -> bool {
        match self {
            GetImages::Images { all_namespaces, .. }
            | GetImages::Registries { all_namespaces, .. } => *all_namespaces,
        }
    }
}
