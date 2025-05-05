use anyhow::Result;
use colored::*;
use kelper::{
    cli::{Args, Commands, GetResource},
    k8s::{display_pod_images, display_pods, K8sClient},
};

#[tokio::main]
async fn main() -> Result<()> {
    use clap::Parser;
    let args = Args::parse();

    // Try to create the client first with better error handling
    let client = match K8sClient::new().await {
        Ok(client) => client,
        Err(e) => {
            eprintln!("\n{} {}", "Kubernetes Error:".red().bold(), e);

            // Provide helpful troubleshooting tips based on the error
            if e.to_string().contains("No kubeconfig found") {
                eprintln!("\n{}", "Troubleshooting Tips:".yellow().bold());
                eprintln!(" - Ensure kubectl is configured on your machine");
                eprintln!(" - Run 'kubectl config view' to check your configuration");
                eprintln!(" - Set KUBECONFIG environment variable if using a non-default config");
            } else if e.to_string().contains("not accessible") {
                eprintln!("\n{}", "Troubleshooting Tips:".yellow().bold());
                eprintln!(" - Check if your cluster is running with 'kubectl cluster-info'");
                eprintln!(" - Verify your network connection to the cluster");
                eprintln!(" - Check if your credentials are valid and not expired");
                eprintln!(" - Ensure your VPN is connected if accessing a remote cluster");
            }

            std::process::exit(1);
        }
    };

    match args.command {
        Commands::Get { resource } => match resource {
            GetResource::Images {
                namespace,
                node,
                pod,
                all_namespaces,
            } => {
                match client
                    .get_pod_images(&namespace, node.as_deref(), pod.as_deref(), all_namespaces)
                    .await
                {
                    Ok(pod_images) => {
                        if pod_images.is_empty() {
                            println!(
                                "\n{}",
                                "No pod images found matching your criteria.".yellow()
                            );
                        } else {
                            // Determine which columns to show
                            let show_node = node.is_none();

                            // Always show namespace when --all-namespaces is used
                            let show_namespace =
                                all_namespaces || (node.is_some() && namespace == "default"); // Keep existing behavior

                            let show_pod = pod.is_none();

                            display_pod_images(&pod_images, show_node, show_namespace, show_pod);
                        }
                    }
                    Err(e) => {
                        eprintln!("\n{} {}", "Error retrieving pod images:".red().bold(), e);
                        std::process::exit(1);
                    }
                }
            }
            GetResource::Pods {
                namespace,
                node,
                registry,
                all_namespaces,
            } => {
                // Call the new function to get pods based on criteria
                match client
                    .get_pods_with_registry(
                        &namespace,
                        node.as_deref(),
                        registry.as_deref(),
                        all_namespaces,
                    )
                    .await
                {
                    Ok(pods) => {
                        if pods.is_empty() {
                            println!("\n{}", "No pods found matching your criteria.".yellow());
                        } else {
                            // Display the pods (implementation needed in k8s module)
                            // For now, just print the count
                            println!("\nFound {} pods matching criteria:", pods.len());
                            display_pods(&pods, all_namespaces, node.is_none(), registry.is_some());
                        }
                    }
                    Err(e) => {
                        eprintln!("\n{} {}", "Error retrieving pods:".red().bold(), e);
                        std::process::exit(1);
                    }
                }
            }
        },
    }

    Ok(())
}
