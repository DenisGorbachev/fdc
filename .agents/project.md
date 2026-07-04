# USDA FDC

## Aliases

### Current data dir

A dir at `.cache/FoodData_Central_csv_2026-04-30`

## Facts

### F001

[Current data dir](#current-data-dir) size is 3.1G.

### F002

USDA will publish updated datasets in the future.

### F003

Import must use upserts.

Reasons: F002

### F003

The count of data rows in a CSV file is equal to total count of rows - 1.

## Concepts

### `fdc` package

A Rust package.

- Must have dependencies:
  - `csv`
  - `time`
- Must contain the types for USDA FDC data.
  - Must contain only the [data structs](#data-struct), not [row structs](#row-struct)
  - Must use the most precise data types
  - Must not contain data that can be calculated
    - Examples:
      - `min_year_acquired`

### struct DataReader

- Must have fields:
  - `inner: ZipArchive<R>`
- Must have one [file iterator method](#file-iterator-method) per file in the archive.
- Must have impls:
  - `From<File>`

### fn must_import

A test for importing data.

- Must read the [current data dir](#current-data-dir)
- Must assert that the count of items in each iterator is equal to the count of rows in each file - 1
  - Reasons: F003

### File iterator method

- Must have a name that starts with `iter` and ends with the file name
- Must return an `impl Iterator` whose `Item` is a [row tuple](#row-tuple)

### Row struct

A Rust struct that contains at least one field for the row identifier and zero or more fields for data.

### Row tuple

A tuple with the following structure:

- First element is the row identifier.
- Second element is the row data struct.

### Data struct

A Rust struct that doesn't contain a row identifier.
