pub type FoodCategoryKey = u16;

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct FoodCategory {
    pub code: u16,
    pub description: Box<str>,
}
