use std::fmt::Display;

use serde::{de::Visitor, Deserialize, Serialize};

#[derive(Debug)]
pub enum Error {
    TooLong,
    ContainsNonDigits,
    ElfProef,
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::TooLong => write!(f, "Provided BSN is too long, must be 8 to 9 chars"),
            Error::ContainsNonDigits => write!(f, "Provided BSN contained non digit chars"),
            Error::ElfProef => write!(f, "Provided BSN doesn't comply with elfproef"),
        }
    }
}

/// A valid BSN (burgerservicenummer), a Dutch
/// personal identification number that is similar
/// to the US Social Security Number.
/// More info (Dutch): https://www.rvig.nl/bsn
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Bsn {
    inner: String,
}

impl Bsn {
    /// Try to create a new BSN. Returns `Err` if the passed string
    /// does not represent a valid BSN
    pub fn try_from_string<B: ToString>(bsn: B) -> Result<Self, Error> {
        let bsn_str = bsn.to_string();
        Self::validate(&bsn_str)?;
        Ok(Self { inner: bsn_str })
    }

    /// Check whether the passed string represents a valid BSN.
    //  Returns `Err` if the passed string does not represent a valid BSN
    pub fn validate(bsn: &str) -> Result<(), Error> {
        // Remove any whitespace and check if it's all digits
        let cleaned = bsn.trim();

        // Must be 8 or 9 digits
        if cleaned.len() < 8 || cleaned.len() > 9 {
            return Err(Error::TooLong);
        }

        // Must contain only digits
        if !cleaned.chars().all(|c| c.is_ascii_digit()) {
            return Err(Error::ContainsNonDigits);
        }

        // Convert to 9-digit format (pad with 0 if 8 digits)
        let padded = if cleaned.len() == 8 {
            format!("0{}", cleaned)
        } else {
            cleaned.to_string()
        };

        // Apply the 11-check (elfproef)
        let digits: Vec<u32> = padded.chars().map(|c| c.to_digit(10).unwrap()).collect();

        // Calculate: (9×A) + (8×B) + (7×C) + (6×D) + (5×E) + (4×F) + (3×G) + (2×H) + (-1×I)
        let sum = (9 * digits[0])
            + (8 * digits[1])
            + (7 * digits[2])
            + (6 * digits[3])
            + (5 * digits[4])
            + (4 * digits[5])
            + (3 * digits[6])
            + (2 * digits[7])
            - digits[8]; // Note: -1 × I

        if sum % 11 == 0 {
            Ok(())
        } else {
            Err(Error::ElfProef)
        }
    }
}

impl Serialize for Bsn {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.inner)
    }
}

impl<'de> Deserialize<'de> for Bsn {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        /// A vistitor for deserializing strings into `Bns`
        struct BsnVisitor;

        impl<'d> Visitor<'d> for BsnVisitor {
            type Value = Bsn;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "A string representing a valid BSN")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Bsn::try_from_string(value)
                    .map_err(|e| E::custom(format!("Invalid BSN format {}", e)))
            }

            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                self.visit_str(&value)
            }
        }

        deserializer.deserialize_str(BsnVisitor)
    }
}

#[cfg(test)]
mod tests {
    use crate::Bsn;

    #[test]
    fn test_validation() {
        let bsns = include_str!("../valid_bsns.in").lines();
        bsns.for_each(|bsn| {
            assert!(
                Bsn::validate(bsn).is_ok(),
                "BSN {bsn} is valid, but did not pass validation"
            )
        });

        let bsns = include_str!("../invalid_bsns.in").lines();
        bsns.for_each(|bsn| {
            assert!(
                Bsn::validate(bsn).is_err(),
                "BSN {bsn} invalid, but passed validation"
            )
        });
    }

    #[test]
    fn test_serde() {
        let json = serde_json::to_string(&Bsn::try_from_string("999998456").unwrap()).unwrap();
        assert_eq!(json, "\"999998456\"");
        let bsn: Bsn = serde_json::from_str("\"999998456\"").unwrap();
        assert_eq!(bsn, Bsn::try_from_string("999998456".to_string()).unwrap());

        serde_json::from_str::<Bsn>("\"1112223333\"").unwrap_err();
    }
}
