use serde::Deserialize;

#[derive(Default, Deserialize, Debug)]
pub struct ConfigOptions {
    id_index: u32,
    update_markers: Vec<u32>,
    print_markers: Vec<u32>
}