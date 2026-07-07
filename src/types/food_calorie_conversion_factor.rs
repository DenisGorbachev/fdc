use crate::CompactDecimal;

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct FoodCalorieConversionFactor {
    pub protein_value: Option<CompactDecimal>,
    pub fat_value: Option<CompactDecimal>,
    pub carbohydrate_value: Option<CompactDecimal>,
}
