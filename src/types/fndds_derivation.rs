#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct FnddsDerivation {
    pub derivation_description: Box<str>,
}
