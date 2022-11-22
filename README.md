# csv-diff-splitter
A tool that analyses and produce diff files (new lines, removed lines, updated lines) for CSV files

This program will take at least a source `A` and a target `B` CSV file, and compare it. Then it will output 3 files :
- A `new` file containing lines in `B` that are not in `A`.
- A `deleted` file containing lines in `A` that are not in `B`.
- A `updated` file containing lines in `B` that are different in `A`.

To accomplish this, the program will use a hash of one or more columns to determine "ids", unique identifiers for each lines.
It will also use a list of "update columns" to check whever a line has changed from `A` to `B` using hash algorithms.

## Installation

Download and extract the archive corresponding to your operating system on the [Release page](https://github.com/owlnext-fr/csv-diff-splitter/releases).

Extract it and place it in a directory mentioned in your `PATH` environment variable.

Alternatively, you can place it wherever you want to use it in another project. Just make sure the binary `csv-diff-splitter` is executable.

## Usage

```txt
Split CSV files using comparison algorithms.

Usage: csv-diff-splitter [OPTIONS] <SOURCE_FILE> <TARGET_FILE>

Arguments:
  <SOURCE_FILE>  Source file to analyze
  <TARGET_FILE>  Target file to analyze

Options:
  -c, --config-file <CONFIG_FILE>  adds a configuration file to process source and target files
  -o, --output-path <OUTPUT_PATH>  path to generate the final CSVs
  -v, --verbose...                 More output per occurrence
  -q, --quiet...                   Less output per occurrence
  -h, --help                       Print help information
  -V, --version                    Print version information
```

Arguments (mandatory) :
- `SOURCE_FILE` is an absolute path to the first CSV file you want to compare.
- `TARGET_FILE` is an absolute path to the first CSV file you want to compare.

Options :
- `-c, --config-file <CONFIG_FILE>` specifies a JSON configuration file for the command to run. See [Configuration section](#configuration)
- `-o, --output-path <OUTPUT_PATH>` specifies a path to the output directory for final files. **Note:** This directory must exists and be writable, it will be checked before the program runs its core components.

## Configuration

The program uses a configuration to execute its core components :
- `id_index` are the indexes of columns representing values composing unique identifiers for each line. It can be only one value, or many values to produce a composite id. **Note:** By default, or if left empty, the id will be the whole line composed.
- `update_markers` are the indexes of columns representing values composing update markers for each line (e.g. values that indicates a change between the two CSV for the same identified line). It can be only one value, or many values to produce a composite update marker. **Note:** By default, or if left empty, the marker will be the whole line composed.
- `update_markers` are the indexes of columns representing values that will be printed on result CSVs. It can be only one value, or many values to produce a filtered CSV. **Note:** By default, or if left empty, the marker will be the whole line composed.
- `separator` specifies the separator by default for the two CSVs.
- `has_headers` specifies if both CSVs have a header line or not.

By default, the program use a "default" configuration like this :
```json
{
    "id_index": [],
    "update_markers": [], 
    "print_markers": [],
    "separator": "|",
    "has_headers": true
}
```

You can create your own configuration file and setting it up for the program using the `--config-file` option.