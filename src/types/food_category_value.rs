use crate::WweiaFoodCategoryKey;

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Eq, PartialEq, Clone, Debug)]
pub enum FoodCategoryValue {
    WweiaFoodCategory(WweiaFoodCategoryKey),
    BrandedFoodCategory(Box<str>),
    Unrecognized(Box<str>),
}
