use core::fmt::{self, Display, Formatter};
use serde::Deserialize;

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Eq, PartialEq, Clone, Debug)]
pub enum FoodDataType {
    AgriculturalAcquisition,
    BrandedFood,
    ExperimentalFood,
    FoundationFood,
    MarketAcquisition,
    SampleFood,
    SrLegacyFood,
    SubSampleFood,
    SurveyFnddsFood,
    Other(Box<str>),
}

use FoodDataType::*;

impl FoodDataType {
    pub fn as_str(&self) -> &str {
        match self {
            AgriculturalAcquisition => "agricultural_acquisition",
            BrandedFood => "branded_food",
            ExperimentalFood => "experimental_food",
            FoundationFood => "foundation_food",
            MarketAcquisition => "market_acquisition",
            SampleFood => "sample_food",
            SrLegacyFood => "sr_legacy_food",
            SubSampleFood => "sub_sample_food",
            SurveyFnddsFood => "survey_fndds_food",
            Other(value) => value,
        }
    }
}

impl From<Box<str>> for FoodDataType {
    fn from(value: Box<str>) -> Self {
        match value.as_ref() {
            "agricultural_acquisition" => AgriculturalAcquisition,
            "branded_food" => BrandedFood,
            "experimental_food" => ExperimentalFood,
            "foundation_food" => FoundationFood,
            // USDA publishes this controlled vocabulary value misspelled in `food.csv`.
            "market_acquistion" | "market_acquisition" => MarketAcquisition,
            "sample_food" => SampleFood,
            "sr_legacy_food" => SrLegacyFood,
            "sub_sample_food" => SubSampleFood,
            "survey_fndds_food" => SurveyFnddsFood,
            _ => Other(value),
        }
    }
}

impl Display for FoodDataType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<'de> Deserialize<'de> for FoodDataType {
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
        Ok(Self::from(input))
    }
}
