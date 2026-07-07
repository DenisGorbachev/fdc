#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct SrLegacyFood {
    pub ndb_number: u32,
}
