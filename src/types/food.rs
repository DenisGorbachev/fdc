use crate::{DateRkyv, FoodCategoryKey, FoodCategoryValue, FoodDataType};
use time::Date;

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct Food {
    pub data_type: FoodDataType,
    pub description: Option<Box<str>>,
    pub food_category_id: Option<FoodCategoryKey>,
    pub food_category: Option<FoodCategoryValue>,
    #[rkyv(with = DateRkyv)]
    pub publication_date: Date,
}
