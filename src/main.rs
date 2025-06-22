use anyhow::Context;
use clap::Parser;
use kelper::{
    display_pod_images, display_registries, logging, Args, Commands, GetImages, K8sClient,
    KelperResult,
};
use tracing::{debug, info, instrument, warn};

/// Main entry point for the Kelper application
#[tokio::main]
async fn main() -> KelperResult<()> {
    let args = Args::parse();

    // Initialize logging with the specified format
    logging::init_logging(logging::configure_logging(args.verbose), args.log_format)
        .context("Failed to initialize logging")?;

    debug!("Application started with args: {:?}", args);

    // Create the client with improved error context
    let client = K8sClient::new()
        .await
        .context("Failed to create Kubernetes client")?;

    info!("Successfully connected to Kubernetes cluster");

    process_commands(args, client).await?;

    debug!("Application completed successfully");
    Ok(())
}

/// Process the command line arguments and execute the corresponding command
#[instrument(skip(client), level = "debug")]
async fn process_commands(args: Args, client: K8sClient) -> KelperResult<()> {
    match args.command {
        Commands::Get { resource } => match resource {
            GetImages::Images {
                namespace,
                node,
                pod,
                registry,
                all_namespaces,
                output,
                ..
            } => {
                debug!(
                    namespace = %namespace,
                    node = ?node,
                    pod = ?pod,
                    registry = ?registry,
                    all_namespaces = %all_namespaces,
                    output = ?output,
                    "Processing get images command"
                );

                let pod_images = client
                    .get_pod_images(
                        &namespace,
                        node.as_deref(),
                        pod.as_deref(),
                        registry.as_deref(),
                        all_namespaces,
                    )
                    .await
                    .context("Failed to retrieve pod images")?;

                if pod_images.is_empty() {
                    warn!("No pod images found matching your criteria");
                } else {
                    debug!(output = ?output, "Displaying pod images");
                    display_pod_images(&pod_images, &output)
                        .context("Failed to display pod images")?;
                    info!(
                        count = pod_images.len(),
                        "Successfully displayed pod images"
                    );
                }
            }
            GetImages::Registries {
                namespace,
                all_namespaces,
                output,
                ..
            } => {
                debug!(
                    namespace = %namespace,
                    all_namespaces = %all_namespaces,
                    output = ?output,
                    "Processing get registries command"
                );

                let registries = client
                    .get_unique_registries(&namespace, all_namespaces)
                    .await
                    .context("Failed to retrieve registries")?;

                if registries.is_empty() {
                    warn!("No registries found in the specified namespace(s)");
                } else {
                    debug!(output = ?output, "Displaying registries");
                    display_registries(&registries, &output)
                        .context("Failed to display registries")?;
                    info!(
                        count = registries.len(),
                        "Successfully displayed registries"
                    );
                }
            }
        },
    }
    Ok(())
}
