use crate::CompactDecimal;

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct FoodNutrient {
    pub fdc_id: u32,
    pub nutrient_id: u16,
    pub amount: CompactDecimal,
    pub data_points: Option<u16>,
    pub derivation_id: Option<u16>,
    pub min: Option<CompactDecimal>,
    pub max: Option<CompactDecimal>,
    pub median: Option<CompactDecimal>,
    pub loq: Option<CompactDecimal>,
    pub footnote: Option<Box<str>>,
    pub min_year_acquired: Option<u16>,
    pub percent_daily_value: Option<CompactDecimal>,
}
