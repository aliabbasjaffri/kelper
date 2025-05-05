use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Get information about Kubernetes resources
    Get {
        #[command(subcommand)]
        resource: GetResource,
    },
}

#[derive(Subcommand, Debug)]
pub enum GetResource {
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

        #[arg(short = 'N', long = "node", conflicts_with = "all_namespaces")]
        node: Option<String>,

        #[arg(short, long)]
        pod: Option<String>,

        #[arg(short = 'A', long = "all-namespaces", conflicts_with = "namespace")]
        all_namespaces: bool,
    },
    /// Get pods based on node and/or registry
    Pods {
        /// Kubernetes namespace to query
        #[arg(
            short,
            long,
            default_value = "default",
            conflicts_with = "all_namespaces"
        )]
        namespace: String,

        /// Node name to filter pods by
        #[arg(short = 'N', long)]
        node: Option<String>,

        /// Filter pods by container image registry
        #[arg(short = 'R', long)]
        registry: Option<String>,

        /// Consider pods in all namespaces
        #[arg(short = 'A', long, conflicts_with = "namespace")]
        all_namespaces: bool,
    },
}
