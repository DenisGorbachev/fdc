use crate::DateRkyv;
use time::Date;

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct FoodUpdateLogEntry {
    pub description: Box<str>,
    #[rkyv(with = DateRkyv)]
    pub last_updated: Date,
}
