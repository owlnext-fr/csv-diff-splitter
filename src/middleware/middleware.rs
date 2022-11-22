use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
    path::PathBuf,
};

use crate::{config::config_options::ConfigOptions, csv_utils::loader::load_data_from_csv};
use chrono::{Datelike, Timelike, Utc};
use csv::StringRecord;
use ulid;

/// processes the middleware program.
pub fn process(
    source_path: PathBuf,
    target_path: PathBuf,
    config: ConfigOptions,
    output_path: PathBuf,
) {
    // load CSV data with configuration

    let source_content = load_data_from_csv(
        source_path.as_path(),
        config.separator.as_str(),
        config.has_headers,
    );
    let target_content = load_data_from_csv(
        target_path.as_path(),
        config.separator.as_str(),
        config.has_headers,
    );

    debug!("Source lines: {}", source_content.len());
    debug!("Target lines: {}", target_content.len());

    // map<id values hashed, update values hashed>
    let mut source_hashed_lines: HashMap<u64, u64> = HashMap::new();
    let mut target_hashed_lines: HashMap<u64, u64> = HashMap::new();

    // map<id values hashed, CSV line representation pointer>
    let mut source_lines: HashMap<u64, &StringRecord> = HashMap::new();
    let mut target_lines: HashMap<u64, &StringRecord> = HashMap::new();

    // vectors of algos results
    let mut new_lines: Vec<&StringRecord> = vec![];
    let mut deleted_lines: Vec<&StringRecord> = vec![];
    let mut updated_lines: Vec<&StringRecord> = vec![];

    // treating source data
    for line in source_content.iter() {
        let mut hasher = DefaultHasher::new();

        if config.id_index.is_empty() {
            for val in line.into_iter() {
                val.hash(&mut hasher);
            }
        } else {
            for idx in config.id_index.iter() {
                line.get(*idx as usize).unwrap_or("").hash(&mut hasher);
            }
        }

        let key = hasher.finish();
        source_lines.insert(key, line);

        let mut hasher = DefaultHasher::new();

        if config.update_markers.is_empty() {
            for val in line.into_iter() {
                val.hash(&mut hasher);
            }
        } else {
            for idx in config.update_markers.iter() {
                line.get(*idx as usize).unwrap_or("").hash(&mut hasher);
            }
        }

        source_hashed_lines.insert(key, hasher.finish());
    }

    // treating target data
    for line in target_content.iter() {
        let mut hasher = DefaultHasher::new();

        if config.id_index.is_empty() {
            for val in line.into_iter() {
                val.hash(&mut hasher);
            }
        } else {
            for idx in config.id_index.iter() {
                line.get(*idx as usize).unwrap_or("").hash(&mut hasher);
            }
        }

        let key = hasher.finish();
        target_lines.insert(key, line);

        let mut hasher = DefaultHasher::new();

        if config.update_markers.is_empty() {
            for val in line.into_iter() {
                val.hash(&mut hasher);
            }
        } else {
            for idx in config.update_markers.iter() {
                line.get(*idx as usize).unwrap_or("").hash(&mut hasher);
            }
        }

        target_hashed_lines.insert(key, hasher.finish());
    }

    debug!("Data computed, starting diff...");

    // process new lines
    for (key, _) in target_hashed_lines.iter() {
        let found = source_lines.get(key);

        if found.is_none() {
            new_lines.push(target_lines.get(key).unwrap());
        }
    }
    debug!("New lines found: {}", new_lines.len());

    // process deleted lines
    for (key, _) in source_hashed_lines.iter() {
        let found = target_lines.get(key);

        if found.is_none() {
            deleted_lines.push(source_lines.get(key).unwrap());
        }
    }

    debug!("Deleted lines found: {}", deleted_lines.len());

    // process updated lines
    for (key, val) in source_hashed_lines.iter() {
        let found_line: Option<&u64> = target_hashed_lines.get(key);

        if let Some(line) = found_line {
            if *line != *val {
                updated_lines.push(target_lines.get(key).unwrap());
            }
        }
    }

    debug!("Updated lines found: {}", updated_lines.len());

    // generate outpout CSVs
    let now = Utc::now();
    let uuid = ulid::Ulid::new();

    let now_str = format!(
        "{}{}{}_{}{}{}_{}",
        now.year(),
        now.month(),
        now.day(),
        now.hour(),
        now.minute(),
        now.second(),
        uuid
    );

    let new_csv_path = output_path.join(format!("{}_new.csv", now_str));
    let deleted_csv_path = output_path.join(format!("{}_deleted.csv", now_str));
    let updated_csv_path = output_path.join(format!("{}_updated.csv", now_str));

    write_csv(&new_csv_path, new_lines, &config);

    debug!(
        "New lines CSV written : {}",
        new_csv_path.as_os_str().to_string_lossy()
    );

    write_csv(&deleted_csv_path, deleted_lines, &config);

    debug!(
        "Deleted lines CSV written : {}",
        deleted_csv_path.as_os_str().to_string_lossy()
    );

    write_csv(&updated_csv_path, updated_lines, &config);

    debug!(
        "Updated lines CSV written : {}",
        updated_csv_path.as_os_str().to_string_lossy()
    );
}

/// write a CSV given a path, datas and a configuration.
fn write_csv(path: &PathBuf, items: Vec<&StringRecord>, config: &ConfigOptions) {
    let mut wtr = csv::WriterBuilder::new()
        .delimiter(*config.separator.as_bytes().get(0).unwrap())
        .from_path(path)
        .unwrap();

    for line in items.iter() {
        if config.print_markers.is_empty() {
            wtr.write_record(*line).unwrap();
        } else {
            let mut line_to_print: Vec<String> = vec![];

            for index in config.print_markers.iter() {
                line_to_print.push(line.get(*index as usize).unwrap().to_string());
            }

            wtr.write_record(line_to_print).unwrap();
        }
    }

    wtr.flush().unwrap();
}
