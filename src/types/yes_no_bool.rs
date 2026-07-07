use serde::Deserialize;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct YesNoBool(bool);

impl From<YesNoBool> for bool {
    fn from(value: YesNoBool) -> Self {
        value.0
    }
}

impl<'de> Deserialize<'de> for YesNoBool {
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
        match input.as_ref() {
            "Y" => Ok(Self(true)),
            "N" => Ok(Self(false)),
            _ => Err(serde::de::Error::custom(format!("expected 'Y' or 'N', got '{input}'"))),
        }
    }
}
