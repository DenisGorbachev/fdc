#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct Microbe {
    pub fdc_id: u32,
    pub method: Box<str>,
    pub microbe_code: Box<str>,
    pub min_value: u32,
    pub max_value: Option<u32>,
    pub uom: Option<Box<str>>,
}
