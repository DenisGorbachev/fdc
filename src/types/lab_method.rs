#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct LabMethod {
    pub description: Box<str>,
    pub technique: Box<str>,
}
