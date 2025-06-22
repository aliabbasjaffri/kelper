use crate::utils::{strip_registry, KNOWN_REGISTRIES};
use anyhow::{Context, Result};
use k8s_openapi::api::core::v1::Pod;
use kube::{api::ListParams, Api, Client};
use thiserror::Error;
use tracing::{debug, error, info, instrument};

/// Represents a container image running in a Kubernetes pod
#[derive(Debug, Clone)]
pub struct PodImage {
    /// Name of the pod containing the image
    pub pod_name: String,
    /// Name of the node where the pod is running
    pub node_name: String,
    /// Kubernetes namespace of the pod
    pub namespace: String,
    /// Name of the container using this image
    pub container_name: String,
    /// Name of the container image
    pub image_name: String,
    /// Version/tag of the container image
    pub image_version: String,
    /// Registry where the image is hosted
    pub registry: String,
    /// Image digest (if available)
    pub digest: String,
}

/// Errors that can occur when interacting with Kubernetes
#[derive(Debug, Error)]
pub enum K8sError {
    /// Configuration-related errors
    #[error("Configuration error: {0}")]
    ConfigError(String),
    /// Connection-related errors
    #[error("Connection error: {0}")]
    ConnectionError(String),
    /// API-related errors
    #[error("API error: {0}")]
    ApiError(String),
    /// Resource not found errors
    #[error("Resource not found: {0}")]
    ResourceNotFound(String),
}

/// Client for interacting with Kubernetes clusters
pub struct K8sClient {
    /// The underlying Kubernetes client
    client: Client,
}

impl K8sClient {
    /// Create a new Kubernetes client
    ///
    /// # Returns
    ///
    /// * `Result<Self>` - A new K8sClient instance or an error if initialization fails
    #[instrument(skip_all)]
    pub async fn new() -> Result<Self> {
        debug!("Initializing Kubernetes client");

        let kubeconfig_path = Self::get_kubeconfig_path()?;
        debug!(path = %kubeconfig_path, "Using kubeconfig path");

        let client = Client::try_default()
            .await
            .context("Failed to create Kubernetes client")?;

        let k8s_client = Self { client };

        // Verify cluster accessibility
        if !k8s_client.is_accessible().await? {
            return Err(
                K8sError::ConnectionError("Kubernetes cluster is not accessible".into()).into(),
            );
        }

        info!("Successfully initialized Kubernetes client");
        Ok(k8s_client)
    }

    /// Get the path to the kubeconfig file
    ///
    /// # Returns
    ///
    /// * `Result<String>` - The path to the kubeconfig file or an error if not found
    fn get_kubeconfig_path() -> Result<String> {
        if let Ok(path) = std::env::var("KUBECONFIG") {
            info!("Using kubeconfig from KUBECONFIG environment variable");
            return Ok(path);
        }

        debug!("KUBECONFIG not set, checking default location");
        let home_dir = std::env::var("HOME").context("Failed to get HOME directory")?;
        let default_kubeconfig = format!("{}/.kube/config", home_dir);

        if !std::path::Path::new(&default_kubeconfig).exists() {
            return Err(
                K8sError::ConfigError("No kubeconfig found at default location".into()).into(),
            );
        }

        info!("Using default kubeconfig location");
        Ok(default_kubeconfig)
    }

    /// Check if the Kubernetes cluster is accessible
    ///
    /// # Returns
    ///
    /// * `Result<bool>` - True if the cluster is accessible, false otherwise
    #[instrument(skip(self))]
    pub async fn is_accessible(&self) -> Result<bool> {
        debug!("Checking cluster accessibility");
        let api: Api<Pod> = Api::namespaced(self.client.clone(), "default");

        match api.list(&Default::default()).await {
            Ok(_) => {
                debug!("Successfully connected to cluster");
                Ok(true)
            }
            Err(e) => match e {
                kube::Error::Api(api_err) => {
                    error!("Kubernetes API error occurred");
                    Err(
                        K8sError::ApiError(format!("{} ({})", api_err.message, api_err.reason))
                            .into(),
                    )
                }
                _ => {
                    error!("Failed to connect to Kubernetes cluster");
                    Err(
                        K8sError::ConnectionError("Failed to connect to Kubernetes cluster".into())
                            .into(),
                    )
                }
            },
        }
    }

    /// Get pod images matching the specified criteria
    ///
    /// # Arguments
    ///
    /// * `namespace` - The namespace to search in
    /// * `node_name` - Optional node name filter
    /// * `pod_name` - Optional pod name filter
    /// * `registry_filter` - Optional registry filter
    /// * `all_namespaces` - Whether to search in all namespaces
    ///
    /// # Returns
    ///
    /// * `Result<Vec<PodImage>>` - List of matching pod images or an error
    #[instrument(skip(self), fields(
        namespace = %namespace,
        node = ?node_name,
        pod = ?pod_name,
        registry = ?registry_filter,
        all_namespaces = %all_namespaces
    ))]
    pub async fn get_pod_images(
        &self,
        namespace: &str,
        node_name: Option<&str>,
        pod_name: Option<&str>,
        registry_filter: Option<&str>,
        all_namespaces: bool,
    ) -> Result<Vec<PodImage>> {
        debug!(
            namespace = %namespace,
            node = ?node_name,
            pod = ?pod_name,
            registry = ?registry_filter,
            all_namespaces = %all_namespaces,
            "Fetching pod images"
        );

        let list_params = Self::build_list_params(node_name, pod_name);
        let pods = self.get_pods_api(namespace, all_namespaces, node_name)?;

        let pods_list = pods
            .list(&list_params)
            .await
            .context("Failed to list pods")?;

        debug!("Found {} pods", pods_list.items.len());

        if pods_list.items.is_empty() {
            let resource = match (node_name, pod_name) {
                (Some(node), Some(pod)) => format!("pod {} on node {}", pod, node),
                (Some(node), None) => format!("pods on node {}", node),
                (None, Some(pod)) => format!("pod {}", pod),
                (None, None) => format!("pods in namespace {}", namespace),
            };
            return Err(K8sError::ResourceNotFound(resource).into());
        }

        let mut all_images = Vec::new();
        for pod in pods_list {
            if !Self::should_process_pod(&pod, all_namespaces, node_name, pod_name) {
                continue;
            }

            let pod_images = process_pod(&pod);
            debug!(images = pod_images.len(), "Processed pod images");
            all_images.extend(pod_images);
        }

        if let Some(registry_filter) = registry_filter {
            let before_count = all_images.len();
            all_images.retain(|image| image.registry == registry_filter);
            debug!(
                before = before_count,
                after = all_images.len(),
                "Filtered images by registry"
            );
        }

        info!(
            total_images = all_images.len(),
            "Successfully retrieved pod images"
        );
        Ok(all_images)
    }

    /// Build list parameters for pod queries
    fn build_list_params(node_name: Option<&str>, pod_name: Option<&str>) -> ListParams {
        let mut field_selectors = Vec::new();

        if let Some(node) = node_name {
            field_selectors.push(format!("spec.nodeName={}", node));
        }

        if let Some(name) = pod_name {
            field_selectors.push(format!("metadata.name={}", name));
        }

        ListParams::default().fields(&field_selectors.join(","))
    }

    /// Get the pods API for the specified namespace
    fn get_pods_api(
        &self,
        namespace: &str,
        all_namespaces: bool,
        _node_name: Option<&str>,
    ) -> Result<Api<Pod>> {
        let api = if all_namespaces {
            Api::all(self.client.clone())
        } else {
            Api::namespaced(self.client.clone(), namespace)
        };
        Ok(api)
    }

    /// Check if a pod should be processed based on filters
    fn should_process_pod(
        pod: &Pod,
        _all_namespaces: bool,
        node_name: Option<&str>,
        pod_name: Option<&str>,
    ) -> bool {
        if let Some(name) = pod_name {
            if pod.metadata.name.as_deref() != Some(name) {
                return false;
            }
        }

        if let Some(node) = node_name {
            if pod.spec.as_ref().and_then(|s| s.node_name.as_deref()) != Some(node) {
                return false;
            }
        }

        true
    }

    /// Get unique container image registries used in the cluster
    ///
    /// # Arguments
    ///
    /// * `namespace` - The namespace to search in
    /// * `all_namespaces` - Whether to search in all namespaces
    ///
    /// # Returns
    ///
    /// * `Result<Vec<String>>` - List of unique registries or an error
    #[instrument(skip(self), fields(
        namespace = %namespace,
        all_namespaces = %all_namespaces
    ))]
    pub async fn get_unique_registries(
        &self,
        namespace: &str,
        all_namespaces: bool,
    ) -> Result<Vec<String>> {
        debug!(
            namespace = %namespace,
            all_namespaces = %all_namespaces,
            "Fetching unique registries"
        );

        let pods = self.get_pods_api(namespace, all_namespaces, None)?;
        let pods_list = pods
            .list(&Default::default())
            .await
            .context("Failed to list pods")?;

        debug!("Found {} pods", pods_list.items.len());

        if pods_list.items.is_empty() {
            let resource = format!("pods in namespace {}", namespace);
            return Err(K8sError::ResourceNotFound(resource).into());
        }

        let mut registries = std::collections::HashSet::new();
        for pod in pods_list {
            if !Self::should_process_pod(&pod, all_namespaces, None, None) {
                continue;
            }

            for container in pod.spec.unwrap_or_default().containers {
                if let Some(image) = container.image {
                    let registry = extract_registry(&image);
                    registries.insert(registry);
                }
            }
        }

        let mut registries_vec: Vec<String> = registries.into_iter().collect();
        registries_vec.sort();

        info!(
            total_registries = registries_vec.len(),
            "Successfully retrieved unique registries"
        );
        Ok(registries_vec)
    }
}

/// Extract the registry from a container image reference
///
/// # Arguments
///
/// * `image` - The container image reference
///
/// # Returns
///
/// * `String` - The registry name
pub fn extract_registry(image: &str) -> String {
    // Split the image string by '/'
    let parts: Vec<&str> = image.split('/').collect();

    // If there's only one part (e.g., "ubuntu" or "nginx"), it's a Docker Hub official image
    if parts.len() == 1 {
        return "docker.io".to_string();
    }

    // If there are two parts without dots or colons in the first part (e.g., "library/ubuntu"),
    // it's likely a Docker Hub image with namespace
    if parts.len() == 2 && !parts[0].contains('.') && !parts[0].contains(':') {
        return "docker.io".to_string();
    }

    // Get the potential registry (first part)
    let potential_registry = parts[0];

    // Check for localhost variants with or without port
    if potential_registry == "localhost"
        || potential_registry.starts_with("localhost:")
        || potential_registry.starts_with("127.0.0.1")
        || potential_registry.starts_with("0.0.0.0")
    {
        return potential_registry.to_string();
    }

    // Check for IP address pattern (more comprehensive check)
    let ip_parts: Vec<&str> = potential_registry.split(':').collect();
    let ip = ip_parts[0];
    if ip.split('.').filter(|&p| !p.is_empty()).count() == 4
        && ip.split('.').all(|p| p.parse::<u8>().is_ok())
    {
        return potential_registry.to_string();
    }

    // Check for known public registries
    let known_registries = KNOWN_REGISTRIES;

    for registry in &known_registries {
        if potential_registry == *registry || potential_registry.ends_with(*registry) {
            return potential_registry.to_string();
        }
    }

    // For any domain with dots (e.g., "my-registry.example.com") or with port (e.g., "registry:5000")
    if potential_registry.contains('.') || potential_registry.contains(':') {
        return potential_registry.to_string();
    }

    // Default to Docker Hub if none of the above matches
    "docker.io".to_string()
}

/// Split a container image reference into name and version
///
/// # Arguments
///
/// * `image` - The container image reference
///
/// # Returns
///
/// * `(String, String)` - Tuple of (image name, image version)
pub fn split_image(image: &str) -> (String, String) {
    // First check for a digest (SHA)
    if let Some(digest_index) = image.find('@') {
        // We have a digest, get the part before the digest
        let image_with_tag = &image[..digest_index];
        let digest = &image[digest_index..]; // includes the @ symbol

        // Find the last colon which separates the image name from the tag
        if let Some(tag_index) = image_with_tag.rfind(':') {
            // Check if this colon is part of a port number in the registry
            // Look for slashes to determine if this is likely a registry port
            let last_slash_index = image_with_tag.rfind('/').unwrap_or(0);

            if tag_index > last_slash_index {
                // This colon is after the last slash, so it's a tag separator
                let name = &image_with_tag[..tag_index];
                let tag = &image_with_tag[tag_index + 1..];
                (name.to_string(), format!("{}@{}", tag, &digest[1..]))
            } else {
                // This colon is part of the registry address, no tag specified
                (
                    image_with_tag.to_string(),
                    format!("latest@{}", &digest[1..]),
                )
            }
        } else {
            // No tag present, use "latest" with the digest
            (
                image_with_tag.to_string(),
                format!("latest@{}", &digest[1..]),
            )
        }
    } else {
        // No digest, handle image name and tag
        // Find the last colon which might separate the image name from the tag
        if let Some(tag_index) = image.rfind(':') {
            // Check if this colon is part of a port number in the registry
            // Look for slashes to determine if this is likely a registry port
            let last_slash_index = image.rfind('/').unwrap_or(0);

            if tag_index > last_slash_index {
                // This colon is after the last slash, so it's a tag separator
                let name = &image[..tag_index];
                let tag = &image[tag_index + 1..];
                return (name.to_string(), tag.to_string());
            }
        }

        // No valid tag separator found
        (image.to_string(), "latest".to_string())
    }
}

/// Extract the digest of a container from a pod
///
/// # Arguments
///
/// * `pod` - The pod containing the container
/// * `container_name` - The name of the container
///
/// # Returns
///
/// * `Option<String>` - The container digest if available
fn extract_container_digest(pod: &Pod, container_name: &str) -> Option<String> {
    pod.status
        .as_ref()?
        .container_statuses
        .as_ref()?
        .iter()
        .find(|cs| cs.name == container_name)?
        .image_id
        .split(':')
        .nth(1)
        .map(String::from)
}

/// Process a pod to extract information about its container images
///
/// # Arguments
///
/// * `pod` - The pod to process
///
/// # Returns
///
/// * `Vec<PodImage>` - List of container images in the pod
pub fn process_pod(pod: &Pod) -> Vec<PodImage> {
    let mut pod_images = Vec::new();
    let pod_name = pod.metadata.name.clone().unwrap_or_default();
    let namespace = pod.metadata.namespace.clone().unwrap_or_default();
    let node_name = pod
        .spec
        .as_ref()
        .and_then(|spec| spec.node_name.clone())
        .unwrap_or_default();

    if let Some(spec) = &pod.spec {
        let containers = &spec.containers;
        for container in containers {
            if let Some(image) = &container.image {
                let registry = extract_registry(image);
                let (_image_name, image_version) = split_image(image);
                let image_name = strip_registry(&_image_name, &registry);
                let digest = extract_container_digest(pod, &container.name).unwrap_or_default();

                pod_images.push(PodImage {
                    pod_name: pod_name.clone(),
                    namespace: namespace.clone(),
                    container_name: container.name.clone(),
                    image_name,
                    image_version,
                    node_name: node_name.clone(),
                    registry,
                    digest,
                });
            }
        }
    }

    pod_images
}
