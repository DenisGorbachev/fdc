use crate::DateRkyv;
use time::Date;

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct SurveyFnddsFood {
    pub food_code: u32,
    pub wweia_category_code: u16,
    #[rkyv(with = DateRkyv)]
    pub start_date: Date,
    #[rkyv(with = DateRkyv)]
    pub end_date: Date,
}
