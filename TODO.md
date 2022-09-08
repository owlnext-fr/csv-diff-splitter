# TODO

## Specification

This software is meant to be a `CLI` that takes as arguments two CSV files (`source` and `target`), and produces three files in result :
- One with new lines in `target` regarding `source`.
- One with removed lines in `target` regarding `source`.
- One with updated lines in `target` regarding `source`.

This CLI app will take an optional `JSON` configuration file specifying:
- The column index for the line identifier (i.e. a line identifier).
- Colums (indexes of) to compare one line to another.
- Colums (indexes of) to keep for the final files.

Some instructions:
- The CLI must be autonomous, no `.env` files or external configuration.
- The CLI must handle verbosity levels.
- The CLI must, if possible, use multithreads.
- The CLI will be exported as a binary.
- The CLI will be exported as a .DEB package.

## Rundown

- Handle CLI command ✅
    - source file
    - target file
    - option JSON file for configuration
    - handle verbosity
        - logger
        - stopwatch
    - args validation

- CSV files
    - Read CSV
    - Write CSV

- Handling confugration & options ✅
    - Load JSON file
        - Deserializing data
        - scheme:
            - identifier
            - update_markers
            - print_markers
    - Default configuration

- Middlewares
    - async/await | async tasks
    - loop over CSV data
        - detect new lines
            - if target.identifier not in source -> new
        - detect deleted lines
            - if source.identifier not in target -> deleted
        - detect updated lines
            - filter update_markers on each line
            - create array with identifier -> hash(values)
            - if source[i].hash != target[i].hash -> updated !

## Tasks


