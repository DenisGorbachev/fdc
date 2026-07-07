#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct FoodAttributeType {
    pub name: Box<str>,
    pub description: Box<str>,
}
