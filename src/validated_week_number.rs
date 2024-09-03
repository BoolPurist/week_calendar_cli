use std::str::FromStr;

use thiserror::Error;

const MINIMUM: u32 = 1;
const MAXIMUM: u32 = 53;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct ValidatedWeekNumber(u32);

#[cfg(test)]
impl From<u32> for ValidatedWeekNumber {
    fn from(value: u32) -> Self {
        Self::new(value).unwrap()
    }
}

#[derive(Debug, Error)]
pub enum InvalidFormatWeekNumber {
    #[error("Must be a positive number")]
    InvalidFormat,
    #[error("{0}")]
    InvalidRange(#[from] InvalidWeekNumber),
}
impl FromStr for ValidatedWeekNumber {
    type Err = InvalidFormatWeekNumber;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let number: u32 = s
            .parse()
            .map_err(|_| InvalidFormatWeekNumber::InvalidFormat)?;
        let parsed = Self::new(number)?;
        Ok(parsed)
    }
}

impl From<ValidatedWeekNumber> for u32 {
    fn from(value: ValidatedWeekNumber) -> Self {
        value.0
    }
}

#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
#[error("A valid week number is between {} and {}", MINIMUM, MAXIMUM)]
pub struct InvalidWeekNumber;
pub type WeekNumberValidatationResult = Result<ValidatedWeekNumber, InvalidWeekNumber>;

impl ValidatedWeekNumber {
    pub fn new(number: u32) -> WeekNumberValidatationResult {
        if !(MINIMUM..=MAXIMUM).contains(&number) {
            Err(InvalidWeekNumber)
        } else {
            Ok(Self(number))
        }
    }
}

#[cfg(test)]
mod testing {
    use super::*;
    use crate::validated_week_number::ValidatedWeekNumber;
    const ERROR: WeekNumberValidatationResult = Err(InvalidWeekNumber);
    #[test]
    fn ensure_valid_week_number() {
        fn assert_case(input: u32, expected: WeekNumberValidatationResult) {
            let actual = ValidatedWeekNumber::new(input);
            assert_eq!(expected, actual, "Input: {}", input);
        }

        assert_case(0, ERROR);
        assert_case(54, ERROR);
        assert_case(1, Ok(ValidatedWeekNumber(1)));
        assert_case(53, Ok(ValidatedWeekNumber(53)));
    }

    #[test]
    fn correct_error_message() {
        const EXPECTED: &str = "A valid week number is between 1 and 53";
        assert_eq!(EXPECTED, ERROR.unwrap_err().to_string());
    }
}
