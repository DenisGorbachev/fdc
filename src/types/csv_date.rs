use crate::parse_fdc_date;
use serde::Deserialize;
use time::Date;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub(super) struct CsvDate(Date);

impl CsvDate {
    pub(super) fn into_date(self) -> Date {
        self.0
    }
}

impl From<CsvDate> for Date {
    fn from(value: CsvDate) -> Self {
        value.into_date()
    }
}

impl<'de> Deserialize<'de> for CsvDate {
    // Project style requires explicit error branches instead of the try operator.
    #[allow(clippy::question_mark)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let input = match Box::<str>::deserialize(deserializer) {
            Ok(value) => value,
            Err(error) => return Err(error),
        };
        match parse_fdc_date(&input) {
            Ok(date) => Ok(Self(date)),
            Err(error) => Err(serde::de::Error::custom(error)),
        }
    }
}
