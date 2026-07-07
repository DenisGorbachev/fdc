use crate::CompactDecimal;

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct FnddsIngredientNutrientValue {
    pub ingredient_description: Box<str>,
    pub nutrient_value: CompactDecimal,
    pub nutrient_value_source: Box<str>,
    pub derivation_code: Option<Box<str>>,
    pub sr_add_mod_year: Option<u16>,
    pub foundation_year_acquired: u16,
}
