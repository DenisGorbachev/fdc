use crate::DateRkyv;
use time::Date;

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct AgriculturalSample {
    #[rkyv(with = DateRkyv)]
    pub acquisition_date: Date,
    pub market_class: Box<str>,
    pub treatment: Box<str>,
    pub state: Box<str>,
}
