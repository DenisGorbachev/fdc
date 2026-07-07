#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct FoodAttribute {
    pub fdc_id: u32,
    pub seq_num: Option<u16>,
    pub food_attribute_type_id: Option<u16>,
    pub name: Option<Box<str>>,
    pub value: Option<Box<str>>,
}
