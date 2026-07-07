use crate::{CompactDecimal, DateRkyv};
use rkyv::with::Map;
use time::Date;

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct BrandedFood {
    pub brand_owner: Option<Box<str>>,
    pub brand_name: Option<Box<str>>,
    pub subbrand_name: Option<Box<str>>,
    pub gtin_upc: Box<str>,
    pub ingredients: Option<Box<str>>,
    pub not_a_significant_source_of: Option<Box<str>>,
    pub serving_size: Option<CompactDecimal>,
    pub serving_size_unit: Option<Box<str>>,
    pub household_serving_fulltext: Option<Box<str>>,
    pub branded_food_category: Option<Box<str>>,
    pub data_source: Box<str>,
    pub package_weight: Option<Box<str>>,
    #[rkyv(with = Map<DateRkyv>)]
    pub modified_date: Option<Date>,
    #[rkyv(with = DateRkyv)]
    pub available_date: Date,
    pub market_country: Box<str>,
    #[rkyv(with = Map<DateRkyv>)]
    pub discontinued_date: Option<Date>,
    pub preparation_state_code: Option<Box<str>>,
    pub trade_channel: Option<Box<str>>,
    pub short_description: Option<Box<str>>,
    pub material_code: Option<u32>,
}
