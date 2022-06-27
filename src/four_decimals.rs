//! Type representation of a u64 with 4 decimals.

use std::ops::{Add, Sub};

use serde::{Deserialize, Deserializer, de};
use serde::{Serialize, Serializer};

const DECIMAL_DIGITS: usize = 4;

/// Type representation of a u64 with fixed decimals.
#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd)]
pub struct FourDecimals {
    /// Integral part.
    pub integer: u64,
    /// Decimal part.
    pub decimal: u16,
}
impl Default for FourDecimals {
    fn default() -> Self {
        Self { integer: 0, decimal: 0 }
    }
}
impl Add for FourDecimals {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let mut integer = self.integer + other.integer;
        let mut decimal = self.decimal + other.decimal;
        if decimal >= 10000 {
            integer += 1;
            decimal -= 10000;
        }

        Self { integer, decimal }
    }
}
impl Sub for FourDecimals {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let mut integer = self.integer - other.integer;
        let decimal = if other.decimal > self.decimal {
            integer -= 1;
            10000 - (other.decimal - self.decimal)
        }
        else {
            self.decimal - other.decimal
        };

        Self { integer, decimal }
    }
}

impl<'de> Deserialize<'de> for FourDecimals {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // deserialize as string
        let s = String::deserialize(deserializer)?;

        // split into integer and decimal
        let parts: Vec<&str> = s.split(".").collect();
        let integer = parts.get(0).ok_or("Error parsing number.")
            .map_err(|e| de::Error::custom(e))?;
        let decimal = parts.get(1).unwrap_or(&"0");

        // check decimals length
        let decimal = String::from(*decimal);
        let decimal_digits = decimal.len();
        if decimal_digits > DECIMAL_DIGITS {
            return Err(de::Error::custom("Too many decimals."));
        }

        // parse numbers from strings
        let integer = integer.parse::<u64>()
            .map_err(|e| de::Error::custom(e))?;
        let mut decimal = decimal.parse::<u16>()
            .map_err(|e| de::Error::custom(e))?;

        for _ in 0..(DECIMAL_DIGITS - decimal_digits) {
            decimal *= 10;
        }

        Ok(FourDecimals {
            integer,
            decimal,
        })
    }
}

impl Serialize for FourDecimals {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}.{:0>4}", self.integer, self.decimal);
        serializer.serialize_str(&s)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    use std::io::Cursor;
    use csv::{ReaderBuilder, Writer};

    #[test]
    fn add_simple() {
        let a = FourDecimals { integer: 1, decimal: 2 };
        let b = FourDecimals { integer: 20, decimal: 10 };
        let r = FourDecimals { integer: 21, decimal: 12 };
        assert_eq!(a + b, r);
    }
    #[test]
    fn add_overflow() {
        let a = FourDecimals { integer: 1, decimal: 6000 };
        let b = FourDecimals { integer: 1, decimal: 9000 };
        let r = FourDecimals { integer: 3, decimal: 5000 };
        assert_eq!(a + b, r);
    }
    #[test]
    fn sub_simple() {
        let a = FourDecimals { integer: 21, decimal: 12 };
        let b = FourDecimals { integer: 20, decimal: 10 };
        let r = FourDecimals { integer: 1, decimal: 2 };
        assert_eq!(a - b, r);
    }
    #[test]
    fn sub_overflow() {
        let a = FourDecimals { integer: 3, decimal: 5000 };
        let b = FourDecimals { integer: 1, decimal: 9000 };
        let r = FourDecimals { integer: 1, decimal: 6000 };
        assert_eq!(a - b, r);
    }

    #[test]
    pub fn deserialize_integer() {
        let input = "1";

        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .from_reader(Cursor::new(input));
        let mut iter = rdr.deserialize();

        let result = iter.next().unwrap();
        let output: FourDecimals = result.unwrap();

        assert_eq!(output, FourDecimals { integer: 1, decimal: 0 })
    }
    #[test]
    pub fn deserialize_basic() {
        let input = "1.0";

        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .from_reader(Cursor::new(input));
        let mut iter = rdr.deserialize();

        let result = iter.next().unwrap();
        let output: FourDecimals = result.unwrap();

        assert_eq!(output, FourDecimals { integer: 1, decimal: 0 })
    }
    #[test]
    pub fn deserialize_trailing() {
        let input = "1.000";

        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .from_reader(Cursor::new(input));
        let mut iter = rdr.deserialize();

        let result = iter.next().unwrap();
        let output: FourDecimals = result.unwrap();

        assert_eq!(output, FourDecimals { integer: 1, decimal: 0 })
    }
    #[test]
    pub fn deserialize_pad() {
        let input = "1.120";

        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .from_reader(Cursor::new(input));
        let mut iter = rdr.deserialize();

        let result = iter.next().unwrap();
        let output: FourDecimals = result.unwrap();

        assert_eq!(output, FourDecimals { integer: 1, decimal: 1200 })
    }
    #[test]
    pub fn deserialize_complex() {
        let input = "12345.1234";

        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .from_reader(Cursor::new(input));
        let mut iter = rdr.deserialize();

        let result = iter.next().unwrap();
        let output: FourDecimals = result.unwrap();

        assert_eq!(output, FourDecimals { integer: 12345, decimal: 1234 })
    }
    #[test]
    pub fn deserialize_long() {
        let input = "12345.123456789";

        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .from_reader(Cursor::new(input));
        let mut iter = rdr.deserialize();

        let result = iter.next().unwrap();
        let result: std::result::Result<FourDecimals, csv::Error> = result;

        assert!(result.is_err());
    }

    #[test]
    pub fn serialize_basic() {
        let input = FourDecimals { integer: 1, decimal: 0 };
        let output = Vec::new();

        let mut wtr = Writer::from_writer(output);
        wtr.serialize(input).unwrap();
        wtr.flush().unwrap();
        let output = wtr.into_inner().unwrap();

        assert_eq!(String::from_utf8_lossy(&output), "1.0000\n");
    }
    #[test]
    pub fn serialize_pad() {
        let input = FourDecimals { integer: 1, decimal: 12 };
        let output = Vec::new();

        let mut wtr = Writer::from_writer(output);
        wtr.serialize(input).unwrap();
        wtr.flush().unwrap();
        let output = wtr.into_inner().unwrap();

        assert_eq!(String::from_utf8_lossy(&output), "1.0012\n");
    }
    #[test]
    pub fn serialize_complex() {
        let input = FourDecimals { integer: 12345, decimal: 1234 };
        let output = Vec::new();

        let mut wtr = Writer::from_writer(output);
        wtr.serialize(input).unwrap();
        wtr.flush().unwrap();
        let output = wtr.into_inner().unwrap();

        assert_eq!(String::from_utf8_lossy(&output), "12345.1234\n");
    }
}
