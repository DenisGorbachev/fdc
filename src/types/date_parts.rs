use time::Date;

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Eq, PartialEq, Hash, Copy, Clone, Debug)]
#[rkyv(derive(Eq, PartialEq, Hash))]
pub struct DateParts {
    year: i32,
    ordinal: u16,
}

impl DateParts {
    pub fn from_date(date: Date) -> Self {
        Self {
            year: date.year(),
            ordinal: date.ordinal(),
        }
    }

    pub fn year(&self) -> i32 {
        self.year
    }

    pub fn ordinal(&self) -> u16 {
        self.ordinal
    }
}

impl From<Date> for DateParts {
    fn from(date: Date) -> Self {
        Self::from_date(date)
    }
}
