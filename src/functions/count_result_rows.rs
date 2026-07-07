use std::error::Error;
use thiserror::Error;

pub fn count_result_rows<T, E>(rows: impl IntoIterator<Item = Result<T, E>>) -> Result<u64, CountResultRowsError<E>>
where
    E: Error,
{
    use CountResultRowsError::*;
    rows.into_iter().try_fold(0_u64, |count, row| match row {
        Ok(_row) => match count.checked_add(1) {
            Some(count) => Ok(count),
            None => Err(CountOverflowedInvalid),
        },
        Err(source) => Err(ReadRowFailed {
            source,
        }),
    })
}

#[derive(Error, Debug)]
pub enum CountResultRowsError<E: Error> {
    #[error("failed to read a row while counting rows")]
    ReadRowFailed { source: E },
    #[error("row count overflowed")]
    CountOverflowedInvalid,
}
