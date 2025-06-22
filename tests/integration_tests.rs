use anyhow::Result;
use kelper::{K8sClient, K8sError};

#[tokio::test]
async fn test_k8s_client_creation() -> Result<()> {
    let client = K8sClient::new().await?;
    assert!(client.is_accessible().await?);
    Ok(())
}

#[tokio::test]
async fn test_get_pod_images() -> Result<()> {
    let client = K8sClient::new().await?;
    // Updated to include the new all_namespaces parameter
    let result = client
        .get_pod_images("default", None, None, None, false)
        .await;

    // In CI environments, there might not be any pods in the default namespace
    // So we should accept both successful empty results and ResourceNotFound errors
    match result {
        Ok(images) => {
            // If we get images, verify they're valid
            for image in images {
                assert!(!image.pod_name.is_empty());
                assert!(!image.namespace.is_empty());
                assert!(!image.container_name.is_empty());
                assert!(!image.image_name.is_empty());
            }
        }
        Err(e) => {
            // If we get an error, it should be a ResourceNotFound error
            assert!(matches!(
                e.downcast_ref::<K8sError>(),
                Some(K8sError::ResourceNotFound(_))
            ));
        }
    }
    Ok(())
}

#[tokio::test]
async fn test_get_pod_images_with_node() -> Result<()> {
    let client = K8sClient::new().await?;
    // Test that we get a ResourceNotFound error when querying a non-existent node
    let result = client
        .get_pod_images("default", Some("non-existent-node"), None, None, false)
        .await;
    assert!(matches!(result, Err(e) if e.downcast_ref::<K8sError>().is_some()));
    Ok(())
}

#[tokio::test]
async fn test_get_pod_images_all_namespaces() -> Result<()> {
    let client = K8sClient::new().await?;
    // Test the new all_namespaces functionality
    let _images = client
        .get_pod_images("default", None, None, None, true)
        .await?;
    // We can't assert specific values here as they depend on the cluster state
    // but we can verify the function doesn't panic
    Ok(())
}

#[tokio::test]
async fn test_get_pod_images_with_node_and_all_namespaces() -> Result<()> {
    let client = K8sClient::new().await?;
    // Test that we get a ResourceNotFound error when querying a non-existent node across all namespaces
    let result = client
        .get_pod_images("default", Some("non-existent-node"), None, None, true)
        .await;
    assert!(matches!(result, Err(e) if e.downcast_ref::<K8sError>().is_some()));
    Ok(())
}

#[tokio::test]
async fn test_get_pod_images_with_pod_and_all_namespaces() -> Result<()> {
    let client = K8sClient::new().await?;
    // Test that we get a ResourceNotFound error when querying a non-existent pod across all namespaces
    let result = client
        .get_pod_images("default", None, Some("non-existent-pod"), None, true)
        .await;
    assert!(matches!(result, Err(e) if e.downcast_ref::<K8sError>().is_some()));
    Ok(())
}

#[tokio::test]
async fn test_get_unique_registries() -> Result<()> {
    let client = K8sClient::new().await?;
    let result = client.get_unique_registries("default", true).await;

    match result {
        Ok(registries) => {
            for registry in registries {
                assert!(!registry.is_empty());
            }
        }
        Err(e) => {
            assert!(matches!(
                e.downcast_ref::<K8sError>(),
                Some(K8sError::ResourceNotFound(_))
            ));
        }
    }
    Ok(())
}
