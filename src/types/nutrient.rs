use crate::CompactDecimal;

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct Nutrient {
    pub name: Box<str>,
    pub unit_name: Box<str>,
    pub nutrient_nbr: Option<Box<str>>,
    pub rank: Option<CompactDecimal>,
}
