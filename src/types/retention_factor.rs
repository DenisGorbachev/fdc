pub type RetentionFactorKey = u16;

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct RetentionFactor {
    pub food_group_id: u16,
    pub description: Box<str>,
}
