use thiserror::Error;
use time::Date;
use time::macros::format_description;

pub fn parse_fdc_date(input: &str) -> Result<Date, ParseFdcDateError> {
    use ParseFdcDateError::*;
    let iso_format = format_description!("[year]-[month]-[day]");
    let slash_format = format_description!("[month padding:none]/[day padding:none]/[year]");
    match Date::parse(input, iso_format) {
        Ok(date) => Ok(date),
        Err(iso_source) => match Date::parse(input, slash_format) {
            Ok(date) => Ok(date),
            Err(source) => Err(ParseSlashDateFailed {
                source,
                input: input.to_owned(),
                iso_source,
            }),
        },
    }
}

#[derive(Error, Debug)]
pub enum ParseFdcDateError {
    #[error("failed to parse FDC date '{input}'")]
    ParseSlashDateFailed { source: time::error::Parse, input: String, iso_source: time::error::Parse },
}
