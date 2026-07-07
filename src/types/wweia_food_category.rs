pub type WweiaFoodCategoryKey = u16;

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct WweiaFoodCategory {
    pub wweia_food_category_description: Box<str>,
}
