use core::fmt::{self, Display, Formatter};
use core::num::{ParseIntError, TryFromIntError};
use core::ops::Not;
use core::str::FromStr;
use serde::Deserialize;
use std::iter::repeat_n;
use thiserror::Error;

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub struct CompactDecimal {
    mantissa: i64,
    scale: u8,
}

impl CompactDecimal {
    pub fn new(mantissa: i64, scale: u8) -> Self {
        Self {
            mantissa,
            scale,
        }
    }

    pub fn mantissa(&self) -> i64 {
        self.mantissa
    }

    pub fn scale(&self) -> u8 {
        self.scale
    }
}

impl FromStr for CompactDecimal {
    type Err = CompactDecimalFromStrError;

    // Project style requires explicit error branches instead of the try operator.
    #[allow(clippy::question_mark)]
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        use CompactDecimalFromStrError::*;
        let (is_negative, unsigned) = match input.strip_prefix('-') {
            Some(unsigned) => (true, unsigned),
            None => match input.strip_prefix('+') {
                Some(unsigned) => (false, unsigned),
                None => (false, input),
            },
        };

        let mut parts = unsigned.split(['e', 'E']);
        let coefficient = match parts.next() {
            Some(coefficient) => coefficient,
            None => {
                return Err(CoefficientMissingInvalid {
                    input: input.to_owned(),
                });
            }
        };
        let exponent_part = parts.next();
        if parts.next().is_some() {
            return Err(ExponentRepeatedInvalid {
                input: input.to_owned(),
            });
        }

        let output = match parse_coefficient(coefficient, input) {
            Ok(value) => value,
            Err(error) => return Err(error),
        };
        let (mantissa, scale) = output;
        let mantissa = if is_negative {
            match mantissa.checked_neg() {
                Some(mantissa) => mantissa,
                None => {
                    return Err(MantissaOverflowedInvalid {
                        input: input.to_owned(),
                    });
                }
            }
        } else {
            mantissa
        };
        let exponent = match exponent_part {
            Some(exponent) => match exponent.parse::<i32>() {
                Ok(exponent) => exponent,
                Err(source) => {
                    return Err(ExponentParseFailed {
                        source,
                        input: input.to_owned(),
                    });
                }
            },
            None => 0,
        };
        apply_exponent(mantissa, scale, exponent, input)
    }
}

impl<'de> Deserialize<'de> for CompactDecimal {
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
        match input.parse::<Self>() {
            Ok(value) => Ok(value),
            Err(error) => Err(serde::de::Error::custom(error)),
        }
    }
}

impl Display for CompactDecimal {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}e-{}", self.mantissa, self.scale)
    }
}

// Project style requires explicit error branches instead of the try operator.
#[allow(clippy::question_mark)]
fn parse_coefficient(coefficient: &str, input: &str) -> Result<(i64, u8), CompactDecimalFromStrError> {
    use CompactDecimalFromStrError::*;
    let result = coefficient
        .chars()
        .try_fold((0_i64, 0_u8, false, false), |(mantissa, scale, seen_digit, seen_decimal_point), character| {
            if character == '.' {
                if seen_decimal_point {
                    return Err(DecimalPointRepeatedInvalid {
                        input: input.to_owned(),
                    });
                }
                return Ok((mantissa, scale, seen_digit, true));
            }
            let digit = match character.to_digit(10) {
                Some(digit) => digit,
                None => {
                    return Err(CharacterInvalid {
                        input: input.to_owned(),
                        character,
                    });
                }
            };
            let mantissa = match mantissa.checked_mul(10) {
                Some(mantissa) => mantissa,
                None => {
                    return Err(MantissaOverflowedInvalid {
                        input: input.to_owned(),
                    });
                }
            };
            let mantissa = match mantissa.checked_add(i64::from(digit)) {
                Some(mantissa) => mantissa,
                None => {
                    return Err(MantissaOverflowedInvalid {
                        input: input.to_owned(),
                    });
                }
            };
            let scale = if seen_decimal_point {
                match scale.checked_add(1) {
                    Some(scale) => scale,
                    None => {
                        return Err(ScaleOverflowedInvalid {
                            input: input.to_owned(),
                        });
                    }
                }
            } else {
                scale
            };
            Ok((mantissa, scale, true, seen_decimal_point))
        });
    let output = match result {
        Ok(value) => value,
        Err(error) => return Err(error),
    };
    let (mantissa, scale, seen_digit, _seen_decimal_point) = output;
    if seen_digit.not() {
        return Err(DigitsMissingInvalid {
            input: input.to_owned(),
        });
    }
    Ok((mantissa, scale))
}

// Project style requires explicit error branches instead of the try operator.
#[allow(clippy::question_mark)]
fn apply_exponent(mantissa: i64, scale: u8, exponent: i32, input: &str) -> Result<CompactDecimal, CompactDecimalFromStrError> {
    use CompactDecimalFromStrError::*;
    if exponent.is_positive() {
        let exponent = match u32::try_from(exponent) {
            Ok(exponent) => exponent,
            Err(source) => {
                return Err(ExponentConvertFailed {
                    source,
                    input: input.to_owned(),
                });
            }
        };
        let scale_u32 = u32::from(scale);
        if exponent <= scale_u32 {
            let scale = match scale_u32
                .checked_sub(exponent)
                .and_then(|scale| u8::try_from(scale).ok())
            {
                Some(scale) => scale,
                None => {
                    return Err(ScaleOverflowedInvalid {
                        input: input.to_owned(),
                    });
                }
            };
            Ok(CompactDecimal::new(mantissa, scale))
        } else {
            let power = match exponent.checked_sub(scale_u32) {
                Some(power) => power,
                None => {
                    return Err(ScaleOverflowedInvalid {
                        input: input.to_owned(),
                    });
                }
            };
            let output_mantissa = match multiply_by_power_of_ten(mantissa, power, input) {
                Ok(value) => value,
                Err(error) => return Err(error),
            };
            Ok(CompactDecimal::new(output_mantissa, 0))
        }
    } else if exponent.is_negative() {
        let exponent = exponent.unsigned_abs();
        let scale = match u32::from(scale)
            .checked_add(exponent)
            .and_then(|scale| u8::try_from(scale).ok())
        {
            Some(scale) => scale,
            None => {
                return Err(ScaleOverflowedInvalid {
                    input: input.to_owned(),
                });
            }
        };
        Ok(CompactDecimal::new(mantissa, scale))
    } else {
        Ok(CompactDecimal::new(mantissa, scale))
    }
}

fn multiply_by_power_of_ten(mantissa: i64, power: u32, input: &str) -> Result<i64, CompactDecimalFromStrError> {
    use CompactDecimalFromStrError::*;
    let iterations = match usize::try_from(power) {
        Ok(iterations) => iterations,
        Err(source) => {
            return Err(PowerConvertFailed {
                source,
                input: input.to_owned(),
            });
        }
    };
    repeat_n((), iterations).try_fold(mantissa, |mantissa, ()| match mantissa.checked_mul(10) {
        Some(mantissa) => Ok(mantissa),
        None => Err(MantissaOverflowedInvalid {
            input: input.to_owned(),
        }),
    })
}

#[derive(Error, Debug)]
pub enum CompactDecimalFromStrError {
    #[error("decimal coefficient is missing in '{input}'")]
    CoefficientMissingInvalid { input: String },
    #[error("decimal contains multiple exponent markers in '{input}'")]
    ExponentRepeatedInvalid { input: String },
    #[error("failed to parse decimal exponent in '{input}'")]
    ExponentParseFailed { source: ParseIntError, input: String },
    #[error("failed to convert decimal exponent in '{input}'")]
    ExponentConvertFailed { source: TryFromIntError, input: String },
    #[error("failed to convert decimal power in '{input}'")]
    PowerConvertFailed { source: TryFromIntError, input: String },
    #[error("decimal contains multiple points in '{input}'")]
    DecimalPointRepeatedInvalid { input: String },
    #[error("decimal contains invalid character '{character}' in '{input}'")]
    CharacterInvalid { input: String, character: char },
    #[error("decimal mantissa overflowed in '{input}'")]
    MantissaOverflowedInvalid { input: String },
    #[error("decimal scale overflowed in '{input}'")]
    ScaleOverflowedInvalid { input: String },
    #[error("decimal digits are missing in '{input}'")]
    DigitsMissingInvalid { input: String },
}
