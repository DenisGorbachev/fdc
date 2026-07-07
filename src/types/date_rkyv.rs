use crate::DateParts;
use rkyv::rancor::{Fallible, Source};
use rkyv::with::{ArchiveWith, DeserializeWith, SerializeWith};
use rkyv::{Archive, Archived, Deserialize, Place, Resolver, Serialize};
use thiserror::Error;
use time::Date;

pub struct DateRkyv;

impl ArchiveWith<Date> for DateRkyv {
    type Archived = Archived<DateParts>;
    type Resolver = Resolver<DateParts>;

    fn resolve_with(field: &Date, resolver: Self::Resolver, out: Place<Self::Archived>) {
        let parts = DateParts::from_date(*field);
        parts.resolve(resolver, out);
    }
}

impl<S> SerializeWith<Date, S> for DateRkyv
where
    S: Fallible + ?Sized,
    DateParts: Serialize<S>,
{
    fn serialize_with(field: &Date, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        let parts = DateParts::from_date(*field);
        parts.serialize(serializer)
    }
}

impl<D> DeserializeWith<Archived<DateParts>, Date, D> for DateRkyv
where
    D: Fallible + ?Sized,
    D::Error: Source,
    Archived<DateParts>: Deserialize<DateParts, D>,
{
    // Project style requires explicit error branches instead of the try operator.
    #[allow(clippy::question_mark)]
    fn deserialize_with(field: &Archived<DateParts>, deserializer: &mut D) -> Result<Date, D::Error> {
        let parts = match field.deserialize(deserializer) {
            Ok(parts) => parts,
            Err(error) => return Err(error),
        };
        match Date::from_ordinal_date(parts.year(), parts.ordinal()) {
            Ok(date) => Ok(date),
            Err(source) => Err(D::Error::new(DateRkyvDeserializeError::DateInvalid {
                source,
                year: parts.year(),
                ordinal: parts.ordinal(),
            })),
        }
    }
}

#[derive(Error, Debug)]
pub enum DateRkyvDeserializeError {
    #[error("archived date is invalid: year {year}, ordinal {ordinal}")]
    DateInvalid { source: time::error::ComponentRange, year: i32, ordinal: u16 },
}
