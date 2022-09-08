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

## Rundown

## Tasks


