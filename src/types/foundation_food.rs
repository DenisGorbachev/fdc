#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct FoundationFood {
    pub ndb_number: u32,
    pub footnote: Option<Box<str>>,
}
