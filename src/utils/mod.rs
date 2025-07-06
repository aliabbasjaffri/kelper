use crate::{k8s::PodImage, OutputFormat};
use anyhow::Result;
use prettytable::{format::FormatBuilder, Cell, Row, Table};
use tracing::warn;

pub mod logging;

/// List of known container image registries
pub const KNOWN_REGISTRIES: [&str; 11] = [
    "docker.io",
    "registry.hub.docker.com",
    "ghcr.io",
    "gcr.io",
    "quay.io",
    "registry.gitlab.com",
    "mcr.microsoft.com",
    "registry.k8s.io",
    "public.ecr.aws",
    "docker.pkg.github.com",
    "pkg.dev",
];

/// Error type for table display operations
#[derive(Debug)]
pub struct TableDisplayError {
    /// Error message describing what went wrong
    message: String,
}

impl std::error::Error for TableDisplayError {}

impl std::fmt::Display for TableDisplayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Table display error: {}", self.message)
    }
}

/// Display pod images in a formatted table
///
/// # Arguments
///
/// * `images` - List of pod images to display
/// * `output_format` - Format to use for displaying the images
///
/// # Returns
///
/// * `Result<()>` - Success or error
pub fn display_pod_images(images: &[PodImage], output_format: &OutputFormat) -> Result<()> {
    if images.is_empty() {
        warn!("No images found matching criteria");
        return Ok(());
    }

    let mut table = create_table()?;
    let header_row = create_header_row(output_format);
    table.add_row(header_row);

    for image in images {
        let row = create_image_row(image, output_format)?;
        table.add_row(row);
    }

    table.printstd();
    Ok(())
}

/// Create a new table with default formatting
///
/// # Returns
///
/// * `Result<Table>` - A new table instance or error
fn create_table() -> Result<Table> {
    let format = FormatBuilder::new()
        .column_separator(' ')
        .separator(
            prettytable::format::LinePosition::Title,
            prettytable::format::LineSeparator::new('-', '-', '-', '-'),
        )
        .padding(0, 1)
        .build();

    let mut table = Table::new();
    table.set_format(format);
    Ok(table)
}

/// Create a header row for the table based on output format
///
/// # Arguments
///
/// * `output_format` - Format to use for displaying the images
///
/// # Returns
///
/// * `Row` - A row containing the table headers
fn create_header_row(output_format: &OutputFormat) -> Row {
    let mut header_cells = vec![
        Cell::new("POD"),
        Cell::new("NAMESPACE"),
        Cell::new("CONTAINER"),
    ];

    if matches!(output_format, OutputFormat::Wide) {
        header_cells.push(Cell::new("REGISTRY"));
    }

    header_cells.extend_from_slice(&[Cell::new("IMAGE"), Cell::new("VERSION")]);

    if matches!(output_format, OutputFormat::Wide) {
        header_cells.extend_from_slice(&[Cell::new("DIGEST"), Cell::new("NODE")]);
    }

    Row::new(header_cells)
}

/// Create a row for a single pod image
///
/// # Arguments
///
/// * `image` - The pod image to create a row for
/// * `output_format` - Format to use for displaying the image
///
/// # Returns
///
/// * `Result<Row>` - A row containing the image information or error
fn create_image_row(image: &PodImage, output_format: &OutputFormat) -> Result<Row> {
    let mut cells = vec![
        Cell::new(&image.pod_name),
        Cell::new(&image.namespace),
        Cell::new(&image.container_name),
    ];

    if matches!(output_format, OutputFormat::Wide) {
        cells.push(Cell::new(&image.registry).style_spec("Fy"));
    }

    cells.extend_from_slice(&[
        Cell::new(&image.image_name),
        Cell::new(&image.image_version),
    ]);

    if matches!(output_format, OutputFormat::Wide) {
        cells.extend_from_slice(&[Cell::new(&image.digest), Cell::new(&image.node_name)]);
    }

    Ok(Row::new(cells))
}

/// Strips the registry prefix from an image name if it exists
///
/// # Arguments
///
/// * `image_name` - The full image name that may include a registry prefix
/// * `registry` - The registry to strip from the image name
///
/// # Returns
///
/// The image name without the registry prefix
pub fn strip_registry(image_name: &str, registry: &str) -> String {
    image_name
        .strip_prefix(&format!("{}/", registry))
        .unwrap_or(image_name)
        .to_string()
}

/// Display a list of container image registries in the specified format
///
/// # Arguments
///
/// * `registries` - List of registry URLs to display
/// * `_output_format` - Format to display the registries in (currently unused)
///
/// # Returns
///
/// * `Result<()>` - Success or error
pub fn display_registries(registries: &[String], _output_format: &OutputFormat) -> Result<()> {
    if registries.is_empty() {
        warn!("No registries found");
        return Ok(());
    }

    let mut table = create_table()?;
    table.add_row(Row::new(vec![Cell::new("CONTAINER REGISTRY")]));

    for registry in registries {
        table.add_row(Row::new(vec![Cell::new(registry)]));
    }

    table.printstd();
    Ok(())
}
