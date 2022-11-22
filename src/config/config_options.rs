use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
/// A struct representing configuration options for the CSV split and comparison.
pub struct ConfigOptions {
    /// The column indexes identifying each line (i.e. a unique identifier).
    pub id_index: Vec<u32>,
    /// Each column indexes regarding data indicating an update in the line.
    pub update_markers: Vec<u32>,
    /// Each column indexes to print in the result CSV.
    pub print_markers: Vec<u32>,
    /// Separator between each columns.
    pub separator: String,
    /// Either the CSV has headers or not.
    pub has_headers: bool,
}

impl Default for ConfigOptions {
    fn default() -> Self {
        Self {
            id_index: vec![0],
            update_markers: vec![],
            print_markers: vec![],
            separator: ",".to_owned(),
            has_headers: false,
        }
    }
}
