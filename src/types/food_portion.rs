use crate::CompactDecimal;

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct FoodPortion {
    pub fdc_id: Option<u32>,
    pub seq_num: Option<u16>,
    pub amount: Option<CompactDecimal>,
    pub measure_unit_id: Option<u16>,
    pub portion_description: Option<Box<str>>,
    pub modifier: Option<Box<str>>,
    pub gram_weight: CompactDecimal,
    pub data_points: Option<u16>,
    pub footnote: Option<Box<str>>,
    pub min_year_acquired: Option<u16>,
}
