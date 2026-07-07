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

### F004

The count of data rows in a CSV file is equal to total count of rows - 1.

## Concepts

### `fdc` package

A Rust package.

- Must have dependencies:
  - `csv`
  - `time`
  - `rkyv`
- Must contain the types for USDA FDC data.
  - Must contain only the [data structs](#data-struct), not [row structs](#row-struct)
  - Must use the most precise data types
  - Must use enums for [controlled-vocabulary fields](#controlled-vocabulary-field)
  - Must use collection key types for [collection reference fields](#collection-reference-field)
  - Must normalize the USDA `food.data_type` value `market_acquistion` to `market_acquisition`

### examples/rkyv.rs

- Must accept args:
  - `dir: PathBuf` (a dir with data files)
- Must read the files in `dir` into `Database`
- Must output the database in `rkyv` format to `stdout`

### struct Database

- Must contain all FDC data
- Must have one field per collection
- Must use `FxHashMap` for collection fields
- Must have derives:
  - `rkyv::Serialize`
  - `rkyv::Deserialize`
- Must have impls:
  - `TryFrom<&Path>`

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

### Controlled-vocabulary field

A field whose values identify a finite domain concept defined by USDA/FDC semantics.

Examples:

- Record type
- Status
- Method kind
- Source kind
- Category code

Notes:

- A field is not a controlled-vocabulary field only because the current CSV has few distinct values.
- If future USDA datasets may add values, the enum must preserve unrecognized values with an `Unknown(Box<str>)` or `Other(Box<str>)` variant.
- Use an exhaustive enum without an unknown/other variant only when unrecognized values are invalid and must fail import.

### Collection reference field

A field whose value identifies a row in a `Database` collection.

Requirements:

- Must use the same Rust type as the key of the referenced collection.
- Must be wrapped in `Option` only when the source field may be blank or absent.
- Must not be stored as free text unless the referenced collection key is text.
- If the source column is overloaded, split it into separate semantically named fields instead of using a stringly typed reference.
- Must preserve unrecognized or non-reference source values in a separate semantically correct field instead of dropping them.

### Food

A specifically prepared food.

Examples:

- Fish, tilapia, raw
- Fish, tilapia, cooked, dry heat

### Food record

Information about the [food](#food).

Notes:

- A single food can have multiple food records.

### Identifier

One of:

- [FDC ID](#fdc-id)
- [NDB Number](#ndb-number)
- [GTIN/UPC](#gtinupc)
- [Food Code](#food-code)

### FDC ID

Identifier of a [food record](#food-record).

### NDB Number

Identifier of a [food](#food) in "Foundation" and "SR Legacy" datasets.

### GTIN/UPC

Identifier of a [food](#food) in "Branded" dataset.

Notes:

- The content of this field is either a GTIN or a UPC.

### Food Code

Identifier of a [food](#food) in "FNDDS" dataset.
