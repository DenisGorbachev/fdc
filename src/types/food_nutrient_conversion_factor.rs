#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct FoodNutrientConversionFactor {
    pub fdc_id: u32,
}
