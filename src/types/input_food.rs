use crate::{CompactDecimal, RetentionFactorKey};

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct InputFood {
    pub fdc_id: u32,
    pub fdc_id_of_input_food: Option<u32>,
    pub seq_num: u16,
    pub amount: CompactDecimal,
    pub sr_code: u32,
    pub sr_description: Box<str>,
    pub unit: Option<Box<str>>,
    pub portion_code: u32,
    pub portion_description: Box<str>,
    pub gram_weight: CompactDecimal,
    pub retention_code: Option<RetentionFactorKey>,
}
