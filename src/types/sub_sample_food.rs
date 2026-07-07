#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct SubSampleFood {
    pub fdc_id_of_sample_food: u32,
}
