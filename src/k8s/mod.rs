use anyhow::{Context, Result};
use colored::*;
use k8s_openapi::api::core::v1::Pod;
use kube::{Api, Client};
use prettytable::{Table, row};

#[derive(Debug)]
pub struct PodImage {
    pub pod_name: String,
    pub namespace: String,
    pub container_name: String,
    pub image_name: String,
    pub image_version: String,
    pub registry: String,
}

pub struct K8sClient {
    client: Client,
}

impl K8sClient {
    pub async fn new() -> Result<Self> {
        let client = Client::try_default()
            .await
            .context("Failed to create kube client")?;
        Ok(Self { client })
    }

    pub async fn get_pod_images(&self, namespace: &str, node_name: Option<&str>) -> Result<Vec<PodImage>> {
        let pods: Api<Pod> = if node_name.is_some() {
            Api::all(self.client.clone())
        } else {
            Api::namespaced(self.client.clone(), namespace)
        };

        let pods_list = pods.list(&Default::default())
            .await
            .context("Failed to list pods")?;

        let mut all_images = Vec::new();
        for pod in pods_list {
            if let Some(node) = node_name {
                if let Some(pod_node) = pod.spec.as_ref().and_then(|s| s.node_name.as_deref()) {
                    if pod_node != node {
                        continue;
                    }
                }
            }
            all_images.extend(process_pod(&pod));
        }

        Ok(all_images)
    }
}

fn extract_registry(image: &str) -> String {
    if let Some(registry) = image.split('/').next() {
        if registry.contains('.') || registry.contains(':') {
            registry.to_string()
        } else {
            "docker.io".to_string()
        }
    } else {
        "docker.io".to_string()
    }
}

fn process_pod(pod: &Pod) -> Vec<PodImage> {
    let mut pod_images = Vec::new();
    let pod_name = pod.metadata.name.clone().unwrap_or_default();
    let namespace = pod.metadata.namespace.clone().unwrap_or_default();
    
    if let Some(spec) = &pod.spec {
        let containers = &spec.containers;
            for container in containers {
                if let Some(image) = &container.image {
                    let (image_name, image_version) = split_image(image);
                    pod_images.push(PodImage {
                        pod_name: pod_name.clone(),
                        namespace: namespace.clone(),
                        container_name: container.name.clone(),
                        image_name,
                        image_version,
                        registry: extract_registry(image),
                    });
                }
            }
        }
    
    pod_images
}

fn split_image(image: &str) -> (String, String) {
    let parts: Vec<&str> = image.split(':').collect();
    let name = parts[0].to_string();
    let version = if parts.len() > 1 {
        parts[1].to_string()
    } else {
        "latest".to_string()
    };
    (name, version)
}

pub fn display_pod_images(images: &[PodImage]) {
    println!("\n{}", "Pod Images and Registries:".green().bold());
    println!("{}", "=".repeat(80));

    let mut table = Table::new();
    table.add_row(row!["Pod Name", "Namespace", "Container", "Image Name", "Version", "Registry"]);
    
    for image in images {
        table.add_row(row![
            image.pod_name,
            image.namespace,
            image.container_name,
            image.image_name,
            image.image_version,
            image.registry.yellow()
        ]);
    }

    table.printstd();
    println!("\n{}", "=".repeat(80));
} 