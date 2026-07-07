use errgonomic::handle;
use std::path::PathBuf;
use thiserror::Error;

pub fn count_csv_data_rows(path: PathBuf) -> Result<u64, CountCsvDataRowsError> {
    use CountCsvDataRowsError::*;
    let mut reader = handle!(csv::Reader::from_path(&path), OpenCsvFailed, path);
    reader
        .records()
        .try_fold(0_u64, |count, result| match result {
            Ok(_record) => match count.checked_add(1) {
                Some(count) => Ok(count),
                None => Err(CountOverflowed {
                    path: path.clone(),
                }),
            },
            Err(source) => Err(ReadRecordFailed {
                source,
                path: path.clone(),
            }),
        })
}

#[derive(Error, Debug)]
pub enum CountCsvDataRowsError {
    #[error("failed to open CSV file '{path}'")]
    OpenCsvFailed { source: csv::Error, path: PathBuf },
    #[error("failed to read CSV record from '{path}'")]
    ReadRecordFailed { source: csv::Error, path: PathBuf },
    #[error("CSV row count overflowed for '{path}'")]
    CountOverflowed { path: PathBuf },
}
