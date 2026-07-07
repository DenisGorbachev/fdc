use rustc_hash::FxHashMap;
use std::error::Error;
use std::hash::Hash;
use thiserror::Error;

/// PRUNING: drops older values for duplicate keys because USDA publishes updated datasets and imports must use upserts.
pub fn collect_upserted_rows<K, V, E>(rows: impl IntoIterator<Item = Result<(K, V), E>>) -> Result<FxHashMap<K, V>, CollectUpsertedRowsError<E>>
where
    K: Eq + Hash,
    E: Error,
{
    use CollectUpsertedRowsError::*;
    rows.into_iter()
        .try_fold(FxHashMap::default(), |mut map, row| match row {
            Ok((key, value)) => {
                let _old_value = map.insert(key, value);
                Ok(map)
            }
            Err(source) => Err(ReadRowFailed {
                source,
            }),
        })
}

#[derive(Error, Debug)]
pub enum CollectUpsertedRowsError<E: Error> {
    #[error("failed to read a CSV row while importing upserted rows")]
    ReadRowFailed { source: E },
}
