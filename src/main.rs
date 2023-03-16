use itertools::Itertools;

#[derive(Debug)]
enum RomanNumeralError {
    InvalidChar,
    MiscError,
}

impl From<&str> for RomanNumeralError {
    fn from(_: &str) -> Self {
        Self::MiscError
    }
}

fn parse_roman_numeral_char(c: char) -> Result<i64, RomanNumeralError> {
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

fn parse_roman_numeral(s: &str) -> Result<i64, RomanNumeralError> {
    assert!(s.is_ascii());

    let mut res = 0;

    for (current, next) in s.chars().tuple_windows() {
        let current = parse_roman_numeral_char(current)?;
        let next = parse_roman_numeral_char(next)?;
        res += if current < next {
            // if the next value is greater, we subtract the current value
            -current
        } else {
            current
        };
    }

    // last character always has its value added
    let last = parse_roman_numeral_char(s.chars().next_back().ok_or("empty string")?)?;
    Ok(res + last)
}

fn main() {
    let mut buf = String::new();
    let _ = std::io::stdin().read_line(&mut buf);
    println!("{:?}", parse_roman_numeral(buf.trim()));
}

#[cfg(test)]
mod tests {
    use crate::parse_roman_numeral;

    #[test]
    fn decreasing_digit_numerals() {
        assert_eq!(parse_roman_numeral("MMXXIII").unwrap(), 2023);
        assert_eq!(parse_roman_numeral("MDCLXVI").unwrap(), 1666);
    }

    #[test]
    fn increasing_digit_numerals() {
        assert_eq!(parse_roman_numeral("IV").unwrap(), 4);
        assert_eq!(parse_roman_numeral("IX").unwrap(), 9);
    }

    #[test]
    fn general_numerals() {
        assert_eq!(parse_roman_numeral("MCMLXXXIV").unwrap(), 1984);
        assert_eq!(parse_roman_numeral("MCCXCIII").unwrap(), 1293);
    }

    #[test]
    #[should_panic]
    fn empty_string() {
        let _ = parse_roman_numeral("").unwrap();
    }

    #[test]
    fn invalid_characters() {
        for chr in "abefghjknopqrstuwyz".chars() {
            let _ = parse_roman_numeral(&format!("{chr}"))
                .expect_err(format!("invalid character '{chr}'").as_str());
        }
        for chr in ":.,;?! \t\n\r".chars() {
            let _ = parse_roman_numeral(&format!("{chr}"))
                .expect_err(format!("invalid character '{chr}'").as_str());
        }
    }

    #[test]
    fn valid_characters() {
        for chr in "cdilmvx".chars() {
            let _ = parse_roman_numeral(&format!("{chr}"))
                .unwrap_or_else(|err| panic!("valid character `{chr}` (error: {err:?})"));
        }
    }
}
