#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct FoodNutrientSource {
    pub code: u16,
    pub description: Box<str>,
}
