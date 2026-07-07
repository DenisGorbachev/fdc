use crate::CompactDecimal;

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct FoodProteinConversionFactor {
    pub value: CompactDecimal,
}
