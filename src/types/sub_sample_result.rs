use crate::CompactDecimal;

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct SubSampleResult {
    pub adjusted_amount: Option<CompactDecimal>,
    pub lab_method_id: u16,
    pub nutrient_name: Box<str>,
}
