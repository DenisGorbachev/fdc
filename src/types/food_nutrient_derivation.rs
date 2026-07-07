#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct FoodNutrientDerivation {
    pub code: Box<str>,
    pub description: Box<str>,
}
