use crate::DateRkyv;
use rkyv::with::Map;
use time::Date;

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct MarketAcquisition {
    pub brand_description: Option<Box<str>>,
    #[rkyv(with = Map<DateRkyv>)]
    pub expiration_date: Option<Date>,
    pub label_weight: Option<Box<str>>,
    pub location: Option<Box<str>>,
    #[rkyv(with = Map<DateRkyv>)]
    pub acquisition_date: Option<Date>,
    pub sales_type: Option<Box<str>>,
    pub sample_lot_nbr: Option<Box<str>>,
    #[rkyv(with = Map<DateRkyv>)]
    pub sell_by_date: Option<Date>,
    pub store_city: Option<Box<str>>,
    pub store_name: Option<Box<str>>,
    pub store_state: Option<Box<str>>,
    pub upc_code: Option<Box<str>>,
    pub acquisition_number: Box<str>,
}
