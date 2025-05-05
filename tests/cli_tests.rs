use clap::Parser;
use kelper::cli::{Args, Commands, GetResource};

#[test]
fn test_cli_parse_get_images_default() {
    let args = Args::parse_from(["kelper", "get", "images"]);

    let Commands::Get { resource } = args.command;
    if let GetResource::Images {
        namespace,
        node,
        pod,
        all_namespaces,
    } = resource
    {
        assert_eq!(namespace, "default");
        assert!(node.is_none());
        assert!(pod.is_none());
        assert!(!all_namespaces);
    } else {
        panic!("Parsed GetResource::Pods when expecting GetResource::Images");
    }
}

#[test]
fn test_cli_parse_get_images_namespace() {
    let args = Args::parse_from(["kelper", "get", "images", "--namespace", "test-ns"]);
    let Commands::Get { resource } = args.command;
    if let GetResource::Images {
        namespace,
        node,
        pod,
        all_namespaces,
    } = resource
    {
        assert_eq!(namespace, "test-ns");
        assert!(node.is_none());
        assert!(pod.is_none());
        assert!(!all_namespaces);
    } else {
        panic!("Parsed GetResource::Pods when expecting GetResource::Images");
    }
}

#[test]
fn test_cli_parse_get_images_all_namespaces() {
    let args = Args::parse_from(["kelper", "get", "images", "--all-namespaces"]);
    let Commands::Get { resource } = args.command;
    if let GetResource::Images {
        namespace,
        node,
        pod,
        all_namespaces,
    } = resource
    {
        // namespace should still be default, but all_namespaces flag should be true
        assert_eq!(namespace, "default");
        assert!(node.is_none());
        assert!(pod.is_none());
        assert!(all_namespaces);
    } else {
        panic!("Parsed GetResource::Pods when expecting GetResource::Images");
    }
}

#[test]
fn test_cli_parse_get_images_all_namespaces_short() {
    // Test the short flag version (-A)
    let args = Args::parse_from(["kelper", "get", "images", "-A"]);
    let Commands::Get { resource } = args.command;
    if let GetResource::Images {
        namespace,
        node,
        pod,
        all_namespaces,
    } = resource
    {
        assert_eq!(namespace, "default");
        assert!(node.is_none());
        assert!(pod.is_none());
        assert!(all_namespaces);
    } else {
        panic!("Parsed GetResource::Pods when expecting GetResource::Images");
    }
}

#[test]
fn test_cli_parse_get_images_node() {
    // Test combining node filter
    let args = Args::parse_from(["kelper", "get", "images", "--node", "worker1"]);
    let Commands::Get { resource } = args.command;
    if let GetResource::Images {
        namespace,
        node,
        pod,
        all_namespaces,
    } = resource
    {
        assert_eq!(namespace, "default");
        assert_eq!(node, Some("worker1".to_string()));
        assert!(pod.is_none());
        assert!(!all_namespaces);
    } else {
        panic!("Parsed GetResource::Pods when expecting GetResource::Images");
    }
}

#[test]
fn test_cli_parse_get_images_pod_and_all_namespaces() {
    // Test combining pod filter with all-namespaces
    let args = Args::parse_from([
        "kelper",
        "get",
        "images",
        "--pod",
        "nginx-pod",
        "--all-namespaces",
    ]);
    let Commands::Get { resource } = args.command;
    if let GetResource::Images {
        namespace,
        node,
        pod,
        all_namespaces,
    } = resource
    {
        assert_eq!(namespace, "default");
        assert!(node.is_none());
        assert_eq!(pod, Some("nginx-pod".to_string()));
        assert!(all_namespaces);
    } else {
        panic!("Parsed GetResource::Pods when expecting GetResource::Images");
    }
}

#[test]
fn test_cli_parse_get_images_namespace_and_all_namespaces_conflict() {
    // This test verifies that clap correctly rejects the combination of
    // --namespace and --all-namespaces flags for the images command
    let result = Args::try_parse_from([
        "kelper",
        "get",
        "images",
        "--namespace",
        "test-ns",
        "--all-namespaces",
    ]);
    assert!(
        result.is_err(),
        "Expected parser to reject conflicting arguments for 'get images'"
    );
}

// --- Tests for 'get pods' ---

#[test]
fn test_cli_parse_get_pods_default() {
    let args = Args::parse_from(["kelper", "get", "pods"]);
    let Commands::Get { resource } = args.command;
    if let GetResource::Pods {
        namespace,
        node,
        registry,
        all_namespaces,
    } = resource
    {
        assert_eq!(namespace, "default");
        assert!(node.is_none());
        assert!(registry.is_none());
        assert!(!all_namespaces);
    } else {
        panic!("Parsed GetResource::Images when expecting GetResource::Pods");
    }
}

#[test]
fn test_cli_parse_get_pods_namespace() {
    let args = Args::parse_from(["kelper", "get", "pods", "-n", "kube-system"]);
    let Commands::Get { resource } = args.command;
    if let GetResource::Pods {
        namespace,
        node,
        registry,
        all_namespaces,
    } = resource
    {
        assert_eq!(namespace, "kube-system");
        assert!(node.is_none());
        assert!(registry.is_none());
        assert!(!all_namespaces);
    } else {
        panic!("Parsed GetResource::Images when expecting GetResource::Pods");
    }
}

#[test]
fn test_cli_parse_get_pods_node() {
    let args = Args::parse_from(["kelper", "get", "pods", "-N", "node-1"]);
    let Commands::Get { resource } = args.command;
    if let GetResource::Pods {
        namespace,
        node,
        registry,
        all_namespaces,
    } = resource
    {
        assert_eq!(namespace, "default");
        assert_eq!(node, Some("node-1".to_string()));
        assert!(registry.is_none());
        assert!(!all_namespaces);
    } else {
        panic!("Parsed GetResource::Images when expecting GetResource::Pods");
    }
}

#[test]
fn test_cli_parse_get_pods_registry() {
    let args = Args::parse_from(["kelper", "get", "pods", "-R", "quay.io"]);
    let Commands::Get { resource } = args.command;
    if let GetResource::Pods {
        namespace,
        node,
        registry,
        all_namespaces,
    } = resource
    {
        assert_eq!(namespace, "default");
        assert!(node.is_none());
        assert_eq!(registry, Some("quay.io".to_string()));
        assert!(!all_namespaces);
    } else {
        panic!("Parsed GetResource::Images when expecting GetResource::Pods");
    }
}

#[test]
fn test_cli_parse_get_pods_all_namespaces() {
    let args = Args::parse_from(["kelper", "get", "pods", "-A"]);
    let Commands::Get { resource } = args.command;
    if let GetResource::Pods {
        namespace,
        node,
        registry,
        all_namespaces,
    } = resource
    {
        assert_eq!(namespace, "default"); // Default namespace is retained but ignored in logic
        assert!(node.is_none());
        assert!(registry.is_none());
        assert!(all_namespaces);
    } else {
        panic!("Parsed GetResource::Images when expecting GetResource::Pods");
    }
}

#[test]
fn test_cli_parse_get_pods_node_and_registry() {
    let args = Args::parse_from(["kelper", "get", "pods", "-N", "node-2", "-R", "ghcr.io"]);
    let Commands::Get { resource } = args.command;
    if let GetResource::Pods {
        namespace,
        node,
        registry,
        all_namespaces,
    } = resource
    {
        assert_eq!(namespace, "default");
        assert_eq!(node, Some("node-2".to_string()));
        assert_eq!(registry, Some("ghcr.io".to_string()));
        assert!(!all_namespaces);
    } else {
        panic!("Parsed GetResource::Images when expecting GetResource::Pods");
    }
}

#[test]
fn test_cli_parse_get_pods_all_namespaces_node_and_registry() {
    let args = Args::parse_from([
        "kelper",
        "get",
        "pods",
        "-A",
        "-N",
        "node-3",
        "-R",
        "docker.io",
    ]);
    let Commands::Get { resource } = args.command;
    if let GetResource::Pods {
        namespace,
        node,
        registry,
        all_namespaces,
    } = resource
    {
        assert_eq!(namespace, "default");
        assert_eq!(node, Some("node-3".to_string()));
        assert_eq!(registry, Some("docker.io".to_string()));
        assert!(all_namespaces);
    } else {
        panic!("Parsed GetResource::Images when expecting GetResource::Pods");
    }
}

#[test]
fn test_cli_parse_get_pods_namespace_and_all_namespaces_conflict() {
    // This test verifies that clap correctly rejects the combination of
    // -n and -A flags for the pods command
    let result = Args::try_parse_from(["kelper", "get", "pods", "-n", "test-ns", "-A"]);
    assert!(
        result.is_err(),
        "Expected parser to reject conflicting arguments (-n and -A) for 'get pods'"
    );
}
