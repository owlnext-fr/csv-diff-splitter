use serde::Deserialize;

#[derive(Default, Deserialize, Debug)]
/// A struct representing configuration options for the CSV split and comparison.
pub struct ConfigOptions {
    /// The column index identifying each line (i.e. a unique identifier).
    pub id_index: u32,
    /// Each column indexes regarding data indicating an update in the line.
    pub update_markers: Vec<u32>,
    /// Each column indexes to print in the result CSV.
    pub print_markers: Vec<u32>
}