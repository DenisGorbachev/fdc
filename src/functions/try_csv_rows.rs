use errgonomic::handle;
use serde::Deserialize;
use std::fs::File;
use std::path::{Path, PathBuf};
use thiserror::Error;

pub fn try_csv_rows<T>(dir: &Path, file_name: &'static str) -> Result<csv::DeserializeRecordsIntoIter<File, T>, TryCsvRowsError>
where
    T: for<'de> Deserialize<'de>,
{
    use TryCsvRowsError::*;
    let path = dir.join(file_name);
    let reader = handle!(csv::Reader::from_path(&path), OpenCsvFailed, path);
    Ok(reader.into_deserialize())
}

#[derive(Error, Debug)]
pub enum TryCsvRowsError {
    #[error("failed to open CSV file '{path}'")]
    OpenCsvFailed { source: csv::Error, path: PathBuf },
}

#[derive(Error, Debug)]
pub enum CsvRowError {
    #[error("failed to read CSV row from '{path}'")]
    ReadRowFailed { source: csv::Error, path: PathBuf },
}
