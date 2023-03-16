use std::str::FromStr;

use itertools::Itertools;

#[derive(Debug)]
pub enum RomanNumeralError {
    InvalidChar,
    InvalidValue,
    MiscError,
}

#[derive(Debug, Default)]
pub struct RomanNumeral {
    value: i64,
}

fn char_to_value(c: char) -> Result<i64, RomanNumeralError> {
    match c.to_ascii_lowercase() {
        'm' => Ok(1000),
        'd' => Ok(500),
        'c' => Ok(100),
        'l' => Ok(50),
        'x' => Ok(10),
        'v' => Ok(5),
        'i' => Ok(1),
        _ => Err(RomanNumeralError::InvalidChar),
    }
}

fn greatest_str_leq_than_n(v: i64) -> (&'static str, i64) {
    match v {
        1000.. => ("M", 1000),
        900.. => ("CM", 900),
        500.. => ("D", 500),
        400.. => ("CD", 400),
        100.. => ("C", 100),
        90.. => ("XC", 90),
        50.. => ("L", 50),
        40.. => ("XL", 40),
        10.. => ("X", 10),
        9.. => ("IX", 10),
        5.. => ("V", 5),
        4.. => ("IV", 5),
        1.. => ("I", 1),
        _ => ("", 0),
    }
}

impl RomanNumeral {
    #[allow(dead_code)]
    fn new() -> RomanNumeral {
        RomanNumeral::default()
    }

    #[allow(dead_code)]
    fn with_value(v: i64) -> RomanNumeral {
        RomanNumeral { value: v }
    }

    fn to_int(&self) -> i64 {
        self.value
    }
}

impl FromStr for RomanNumeral {
    type Err = RomanNumeralError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut res = 0;

        for (current, next) in s.chars().tuple_windows() {
            let current = char_to_value(current)?;
            let next = char_to_value(next)?;
            res += if current < next {
                // if the next value is greater, we subtract the current value
                -current
            } else {
                current
            };
        }

        // last character always has its value added
        let last = char_to_value(s.chars().next_back().ok_or("empty string")?)?;
        Ok(RomanNumeral { value: res + last })
    }
}

impl ToString for RomanNumeral {
    fn to_string(&self) -> String {
        let mut result = String::with_capacity(self.value as usize / 1000);
        let mut val = self.value;

        while val > 0 {
            let (s, v) = greatest_str_leq_than_n(val);
            val -= v;
            result.push_str(s);
        }

        result
    }
}

impl From<&str> for RomanNumeralError {
    fn from(_: &str) -> Self {
        RomanNumeralError::MiscError
    }
}

fn getline() -> String {
    let mut buf = String::new();
    let _ = std::io::stdin().read_line(&mut buf);
    buf
}

fn main() {
    loop {
        let r = RomanNumeral::from_str(getline().trim());
        if let Ok(i) = r {
            println!("value: {:?}", i.to_int());
            println!("converted back: {:?}", i.to_string());
        } else {
            println!("error: {:?}", r.err().unwrap());
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::RomanNumeral;

    #[test]
    fn decreasing_digit_numerals() {
        assert_eq!(RomanNumeral::from_str("MMXXIII").unwrap().to_int(), 2023);
        assert_eq!(RomanNumeral::from_str("MDCLXVI").unwrap().to_int(), 1666);
    }

    #[test]
    fn increasing_digit_numerals() {
        assert_eq!(RomanNumeral::from_str("IV").unwrap().to_int(), 4);
        assert_eq!(RomanNumeral::from_str("IX").unwrap().to_int(), 9);
    }

    #[test]
    fn general_numerals() {
        assert_eq!(RomanNumeral::from_str("MCMLXXXIV").unwrap().to_int(), 1984);
        assert_eq!(RomanNumeral::from_str("MCCXCIII").unwrap().to_int(), 1293);
    }

    #[test]
    #[should_panic]
    fn empty_string() {
        let _ = RomanNumeral::from_str("").unwrap();
    }

    #[test]
    fn invalid_characters() {
        for chr in "abefghjknopqrstuwyz".chars() {
            let _ = RomanNumeral::from_str(&format!("{chr}"))
                .expect_err(format!("invalid character '{chr}'").as_str());
        }
        for chr in ":.,;?! \t\n\r".chars() {
            let _ = RomanNumeral::from_str(&format!("{chr}"))
                .expect_err(format!("invalid character '{chr}'").as_str());
        }
    }

    #[test]
    fn valid_characters() {
        for chr in "cdilmvx".chars() {
            let _ = RomanNumeral::from_str(&format!("{chr}"))
                .unwrap_or_else(|err| panic!("valid character `{chr}` (error: {err:?})"));
        }
    }

    #[test]
    fn convert_back_and_forth() {
        for i in 1..10_000 {
            assert_eq!(
                i,
                RomanNumeral::from_str(&RomanNumeral::with_value(i).to_string())
                    .unwrap()
                    .to_int(),
                "result of to_string(): `{}`",
                RomanNumeral::with_value(i).to_string(),
            );
        }
    }
}
