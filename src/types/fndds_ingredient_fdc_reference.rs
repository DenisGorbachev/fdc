use serde::Deserialize;

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Eq, PartialEq, Hash, Copy, Clone, Debug)]
#[rkyv(derive(Eq, PartialEq, Hash))]
pub enum FnddsIngredientFdcReference {
    Food(u32),
    Missing,
    ZeroSentinel,
}

use FnddsIngredientFdcReference::*;

impl<'de> Deserialize<'de> for FnddsIngredientFdcReference {
    // Project style requires explicit error branches instead of the try operator.
    #[allow(clippy::question_mark)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let input = match Box::<str>::deserialize(deserializer) {
            Ok(value) => value,
            Err(error) => return Err(error),
        };
        if input.is_empty() {
            return Ok(Missing);
        }
        if input.as_ref() == "0" {
            return Ok(ZeroSentinel);
        }
        match input.parse::<u32>() {
            Ok(fdc_id) => Ok(Food(fdc_id)),
            Err(error) => Err(serde::de::Error::custom(error)),
        }
    }
}
