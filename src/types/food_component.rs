use crate::CompactDecimal;

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct FoodComponent {
    pub fdc_id: u32,
    pub name: Box<str>,
    pub pct_weight: Option<CompactDecimal>,
    pub is_refuse: bool,
    pub gram_weight: CompactDecimal,
    pub data_points: u16,
    pub min_year_acquired: Option<u16>,
}
