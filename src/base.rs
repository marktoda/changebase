use crate::errors::BaseError;
use crate::opts::Base;
use num::{bigint::BigUint, Num};

pub struct Value {
    value: BigUint,
}

impl Value {
    pub fn from(value: String, base: Base) -> Result<Value, BaseError> {
        // Strip prefix if present
        let stripped = strip_prefix(&value, base);
        Value::validate(base, stripped.clone())?;

        match base {
            Base::Bin => BigUint::from_str_radix(&stripped, 2),
            Base::Oct => BigUint::from_str_radix(&stripped, 8),
            Base::Dec => BigUint::from_str_radix(&stripped, 10),
            Base::Hex => BigUint::from_str_radix(&stripped, 16),
        }
        .map_err(|_| Value::get_parse_error(base))
        .map(|value| Value { value })
    }

    pub fn to_base(&self, base: Base) -> String {
        match base {
            Base::Bin => self.value.to_str_radix(2),
            Base::Oct => self.value.to_str_radix(8),
            Base::Dec => self.value.to_str_radix(10),
            Base::Hex => self.value.to_str_radix(16),
        }
    }

    fn validate(base: Base, value: String) -> Result<(), BaseError> {
        let valid = match base {
            Base::Bin => is_valid_bin(&value),
            Base::Oct => is_valid_oct(&value),
            Base::Dec => is_valid_dec(&value),
            Base::Hex => is_valid_hex(&value),
        };
        if valid {
            Ok(())
        } else {
            Err(Value::get_parse_error(base))
        }
    }

    fn get_parse_error(base: Base) -> BaseError {
        return match base {
            Base::Bin => BaseError::ParseError {
                message: "Binary: only include the digits 0 or 1.",
            },
            Base::Oct => BaseError::ParseError {
                message: "Octal: only enter the digits 0-7.",
            },
            Base::Dec => BaseError::ParseError {
                message: "Decimal: only enter the digits 0-9",
            },
            Base::Hex => BaseError::ParseError {
                message: "Hexaxecimal: only enter the digita 0-9 and a-f",
            },
        };
    }
}

/// Strip the prefix from a value for the given base
fn strip_prefix(value: &str, base: Base) -> String {
    let lower = value.to_lowercase();
    match base {
        Base::Bin => lower.strip_prefix("0b").unwrap_or(value).to_string(),
        Base::Oct => lower.strip_prefix("0o").unwrap_or(value).to_string(),
        Base::Hex => lower.strip_prefix("0x").unwrap_or(value).to_string(),
        Base::Dec => value.to_string(),
    }
}

fn is_valid_bin(value: &str) -> bool {
    !value.is_empty() && value.chars().all(|c| c == '0' || c == '1')
}

fn is_valid_oct(value: &str) -> bool {
    !value.is_empty() && value.chars().all(|c| ('0'..='7').contains(&c))
}

fn is_valid_dec(value: &str) -> bool {
    !value.is_empty() && value.chars().all(|c| c.is_ascii_digit())
}

fn is_valid_hex(value: &str) -> bool {
    !value.is_empty() && value.chars().all(|c| c.is_ascii_hexdigit())
}

/// Detect the base of a value using prefix-based detection.
///
/// Detection rules:
/// 1. `0b` prefix → Binary
/// 2. `0o` prefix → Octal
/// 3. `0x` prefix → Hexadecimal
/// 4. Contains a-f letters → Hexadecimal
/// 5. Otherwise → Decimal (the most common case)
pub fn detect_base(value: &str) -> Result<Base, BaseError> {
    let lower = value.to_lowercase();

    // Check for explicit prefixes first
    if lower.starts_with("0b") {
        let stripped = &lower[2..];
        if is_valid_bin(stripped) {
            return Ok(Base::Bin);
        } else {
            return Err(BaseError::ParseError {
                message: "Invalid binary number after 0b prefix",
            });
        }
    }

    if lower.starts_with("0o") {
        let stripped = &lower[2..];
        if is_valid_oct(stripped) {
            return Ok(Base::Oct);
        } else {
            return Err(BaseError::ParseError {
                message: "Invalid octal number after 0o prefix",
            });
        }
    }

    if lower.starts_with("0x") {
        let stripped = &lower[2..];
        if is_valid_hex(stripped) {
            return Ok(Base::Hex);
        } else {
            return Err(BaseError::ParseError {
                message: "Invalid hexadecimal number after 0x prefix",
            });
        }
    }

    // No prefix - check content
    if value.is_empty() {
        return Err(BaseError::ParseError {
            message: "Empty input",
        });
    }

    // If it contains hex letters (a-f), it must be hex
    if lower.chars().any(|c| ('a'..='f').contains(&c)) {
        if is_valid_hex(&lower) {
            return Ok(Base::Hex);
        } else {
            return Err(BaseError::ParseError {
                message: "Invalid hexadecimal number",
            });
        }
    }

    // Default to decimal for pure numeric input
    if is_valid_dec(value) {
        return Ok(Base::Dec);
    }

    Err(BaseError::ParseError {
        message: "Unable to detect base",
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== Value::from tests ====================

    mod from_binary {
        use super::*;

        #[test]
        fn parses_simple_binary() {
            let val = Value::from("1010".to_string(), Base::Bin).unwrap();
            assert_eq!(val.to_base(Base::Dec), "10");
        }

        #[test]
        fn parses_all_zeros() {
            let val = Value::from("0000".to_string(), Base::Bin).unwrap();
            assert_eq!(val.to_base(Base::Dec), "0");
        }

        #[test]
        fn parses_all_ones() {
            let val = Value::from("11111111".to_string(), Base::Bin).unwrap();
            assert_eq!(val.to_base(Base::Dec), "255");
        }

        #[test]
        fn parses_single_bit() {
            let val = Value::from("1".to_string(), Base::Bin).unwrap();
            assert_eq!(val.to_base(Base::Dec), "1");
        }

        #[test]
        fn rejects_invalid_digits() {
            let result = Value::from("1021".to_string(), Base::Bin);
            assert!(result.is_err());
        }

        #[test]
        fn rejects_hex_chars() {
            let result = Value::from("1a01".to_string(), Base::Bin);
            assert!(result.is_err());
        }
    }

    mod from_octal {
        use super::*;

        #[test]
        fn parses_simple_octal() {
            let val = Value::from("77".to_string(), Base::Oct).unwrap();
            assert_eq!(val.to_base(Base::Dec), "63");
        }

        #[test]
        fn parses_zero() {
            let val = Value::from("0".to_string(), Base::Oct).unwrap();
            assert_eq!(val.to_base(Base::Dec), "0");
        }

        #[test]
        fn parses_all_valid_digits() {
            let val = Value::from("01234567".to_string(), Base::Oct).unwrap();
            assert_eq!(val.to_base(Base::Dec), "342391");
        }

        #[test]
        fn rejects_digit_8() {
            let result = Value::from("78".to_string(), Base::Oct);
            assert!(result.is_err());
        }

        #[test]
        fn rejects_digit_9() {
            let result = Value::from("79".to_string(), Base::Oct);
            assert!(result.is_err());
        }
    }

    mod from_decimal {
        use super::*;

        #[test]
        fn parses_simple_decimal() {
            let val = Value::from("255".to_string(), Base::Dec).unwrap();
            assert_eq!(val.to_base(Base::Hex), "ff");
        }

        #[test]
        fn parses_zero() {
            let val = Value::from("0".to_string(), Base::Dec).unwrap();
            assert_eq!(val.to_base(Base::Bin), "0");
        }

        #[test]
        fn parses_large_number() {
            let val = Value::from("1000000".to_string(), Base::Dec).unwrap();
            assert_eq!(val.to_base(Base::Hex), "f4240");
        }

        #[test]
        fn rejects_hex_chars() {
            let result = Value::from("12a".to_string(), Base::Dec);
            assert!(result.is_err());
        }

        #[test]
        fn rejects_letters() {
            let result = Value::from("abc".to_string(), Base::Dec);
            assert!(result.is_err());
        }
    }

    mod from_hex {
        use super::*;

        #[test]
        fn parses_lowercase_hex() {
            let val = Value::from("ff".to_string(), Base::Hex).unwrap();
            assert_eq!(val.to_base(Base::Dec), "255");
        }

        #[test]
        fn parses_uppercase_hex() {
            let val = Value::from("FF".to_string(), Base::Hex).unwrap();
            assert_eq!(val.to_base(Base::Dec), "255");
        }

        #[test]
        fn parses_mixed_case_hex() {
            let val = Value::from("FfAa".to_string(), Base::Hex).unwrap();
            assert_eq!(val.to_base(Base::Dec), "65450");
        }

        #[test]
        fn parses_with_0x_prefix() {
            let val = Value::from("0xff".to_string(), Base::Hex).unwrap();
            assert_eq!(val.to_base(Base::Dec), "255");
        }

        #[test]
        fn parses_with_0x_prefix_uppercase() {
            let val = Value::from("0xFF".to_string(), Base::Hex).unwrap();
            assert_eq!(val.to_base(Base::Dec), "255");
        }

        #[test]
        fn parses_all_valid_digits() {
            let val = Value::from("0123456789abcdef".to_string(), Base::Hex).unwrap();
            assert_eq!(val.to_base(Base::Dec), "81985529216486895");
        }

        #[test]
        fn rejects_invalid_hex_char() {
            let result = Value::from("fg".to_string(), Base::Hex);
            assert!(result.is_err());
        }
    }

    // ==================== Value::to_base tests ====================

    mod to_base_conversions {
        use super::*;

        #[test]
        fn decimal_to_binary() {
            let val = Value::from("42".to_string(), Base::Dec).unwrap();
            assert_eq!(val.to_base(Base::Bin), "101010");
        }

        #[test]
        fn decimal_to_octal() {
            let val = Value::from("64".to_string(), Base::Dec).unwrap();
            assert_eq!(val.to_base(Base::Oct), "100");
        }

        #[test]
        fn decimal_to_hex() {
            let val = Value::from("255".to_string(), Base::Dec).unwrap();
            assert_eq!(val.to_base(Base::Hex), "ff");
        }

        #[test]
        fn binary_to_decimal() {
            let val = Value::from("11111111".to_string(), Base::Bin).unwrap();
            assert_eq!(val.to_base(Base::Dec), "255");
        }

        #[test]
        fn binary_to_hex() {
            let val = Value::from("11110000".to_string(), Base::Bin).unwrap();
            assert_eq!(val.to_base(Base::Hex), "f0");
        }

        #[test]
        fn binary_to_octal() {
            let val = Value::from("111111".to_string(), Base::Bin).unwrap();
            assert_eq!(val.to_base(Base::Oct), "77");
        }

        #[test]
        fn hex_to_binary() {
            let val = Value::from("a5".to_string(), Base::Hex).unwrap();
            assert_eq!(val.to_base(Base::Bin), "10100101");
        }

        #[test]
        fn hex_to_decimal() {
            let val = Value::from("100".to_string(), Base::Hex).unwrap();
            assert_eq!(val.to_base(Base::Dec), "256");
        }

        #[test]
        fn hex_to_octal() {
            let val = Value::from("ff".to_string(), Base::Hex).unwrap();
            assert_eq!(val.to_base(Base::Oct), "377");
        }

        #[test]
        fn octal_to_binary() {
            let val = Value::from("7".to_string(), Base::Oct).unwrap();
            assert_eq!(val.to_base(Base::Bin), "111");
        }

        #[test]
        fn octal_to_decimal() {
            let val = Value::from("100".to_string(), Base::Oct).unwrap();
            assert_eq!(val.to_base(Base::Dec), "64");
        }

        #[test]
        fn octal_to_hex() {
            let val = Value::from("377".to_string(), Base::Oct).unwrap();
            assert_eq!(val.to_base(Base::Hex), "ff");
        }

        #[test]
        fn same_base_identity() {
            let val = Value::from("12345".to_string(), Base::Dec).unwrap();
            assert_eq!(val.to_base(Base::Dec), "12345");
        }
    }

    // ==================== Large number tests ====================

    mod large_numbers {
        use super::*;

        #[test]
        fn handles_u64_max() {
            let val = Value::from("18446744073709551615".to_string(), Base::Dec).unwrap();
            assert_eq!(val.to_base(Base::Hex), "ffffffffffffffff");
        }

        #[test]
        fn handles_larger_than_u64() {
            let val = Value::from("18446744073709551616".to_string(), Base::Dec).unwrap();
            assert_eq!(val.to_base(Base::Hex), "10000000000000000");
        }

        #[test]
        fn handles_very_large_binary() {
            let binary = "1".repeat(128);
            let val = Value::from(binary, Base::Bin).unwrap();
            assert_eq!(val.to_base(Base::Hex), "ffffffffffffffffffffffffffffffff");
        }

        #[test]
        fn handles_256_bit_hex() {
            let hex = "f".repeat(64);
            let val = Value::from(hex, Base::Hex).unwrap();
            let binary_result = val.to_base(Base::Bin);
            assert_eq!(binary_result.len(), 256);
            assert!(binary_result.chars().all(|c| c == '1'));
        }
    }

    // ==================== Edge cases ====================

    mod edge_cases {
        use super::*;

        #[test]
        fn zero_in_all_bases() {
            let bases = [Base::Bin, Base::Oct, Base::Dec, Base::Hex];
            for base in bases {
                let val = Value::from("0".to_string(), base).unwrap();
                assert_eq!(val.to_base(Base::Dec), "0");
            }
        }

        #[test]
        fn leading_zeros_binary() {
            let val = Value::from("00001010".to_string(), Base::Bin).unwrap();
            assert_eq!(val.to_base(Base::Dec), "10");
            // Leading zeros stripped in output
            assert_eq!(val.to_base(Base::Bin), "1010");
        }

        #[test]
        fn leading_zeros_decimal() {
            let val = Value::from("00255".to_string(), Base::Dec).unwrap();
            assert_eq!(val.to_base(Base::Hex), "ff");
        }

        #[test]
        fn single_digit_conversions() {
            for i in 0..10 {
                let val = Value::from(i.to_string(), Base::Dec).unwrap();
                assert_eq!(val.to_base(Base::Dec), i.to_string());
            }
        }
    }

    // ==================== detect_base tests ====================

    mod detect_base_tests {
        use super::*;

        // === Prefix-based detection ===

        #[test]
        fn detects_binary_with_0b_prefix() {
            let result = detect_base("0b1010").unwrap();
            assert_eq!(result, Base::Bin);
        }

        #[test]
        fn detects_binary_with_0b_prefix_uppercase() {
            let result = detect_base("0B1010").unwrap();
            assert_eq!(result, Base::Bin);
        }

        #[test]
        fn detects_octal_with_0o_prefix() {
            let result = detect_base("0o777").unwrap();
            assert_eq!(result, Base::Oct);
        }

        #[test]
        fn detects_octal_with_0o_prefix_uppercase() {
            let result = detect_base("0O755").unwrap();
            assert_eq!(result, Base::Oct);
        }

        #[test]
        fn detects_hex_with_0x_prefix() {
            let result = detect_base("0xff").unwrap();
            assert_eq!(result, Base::Hex);
        }

        #[test]
        fn detects_hex_with_0x_prefix_uppercase() {
            let result = detect_base("0XFF").unwrap();
            assert_eq!(result, Base::Hex);
        }

        // === Content-based detection ===

        #[test]
        fn detects_hex_with_letters_no_prefix() {
            let result = detect_base("abc123").unwrap();
            assert_eq!(result, Base::Hex);
        }

        #[test]
        fn detects_hex_with_only_letters() {
            let result = detect_base("deadbeef").unwrap();
            assert_eq!(result, Base::Hex);
        }

        // === Decimal default (key behavior change) ===

        #[test]
        fn pure_digits_default_to_decimal() {
            // Previously detected as binary, now decimal
            let result = detect_base("101").unwrap();
            assert_eq!(result, Base::Dec);
        }

        #[test]
        fn octal_looking_numbers_default_to_decimal() {
            // Previously detected as octal, now decimal
            let result = detect_base("777").unwrap();
            assert_eq!(result, Base::Dec);
        }

        #[test]
        fn regular_decimal_detected() {
            let result = detect_base("12389").unwrap();
            assert_eq!(result, Base::Dec);
        }

        #[test]
        fn large_decimal_detected() {
            let result = detect_base("999999999").unwrap();
            assert_eq!(result, Base::Dec);
        }

        // === Error cases ===

        #[test]
        fn fails_on_invalid_characters() {
            let result = detect_base("xyz");
            assert!(result.is_err());
        }

        #[test]
        fn fails_on_special_characters() {
            let result = detect_base("12+34");
            assert!(result.is_err());
        }

        #[test]
        fn fails_on_empty_string() {
            let result = detect_base("");
            assert!(result.is_err());
        }

        #[test]
        fn fails_on_invalid_binary_after_0b() {
            let result = detect_base("0b123");
            assert!(result.is_err());
        }

        #[test]
        fn fails_on_invalid_octal_after_0o() {
            let result = detect_base("0o789");
            assert!(result.is_err());
        }

        #[test]
        fn fails_on_invalid_hex_after_0x() {
            let result = detect_base("0xghij");
            assert!(result.is_err());
        }
    }

    // ==================== Validation function tests ====================

    mod validation_tests {
        use super::*;

        #[test]
        fn is_valid_bin_accepts_valid() {
            assert!(is_valid_bin("0"));
            assert!(is_valid_bin("1"));
            assert!(is_valid_bin("010101"));
        }

        #[test]
        fn is_valid_bin_rejects_invalid() {
            assert!(!is_valid_bin("2"));
            assert!(!is_valid_bin("a"));
            assert!(!is_valid_bin("01201"));
        }

        #[test]
        fn is_valid_bin_rejects_empty() {
            assert!(!is_valid_bin(""));
        }

        #[test]
        fn is_valid_oct_accepts_valid() {
            assert!(is_valid_oct("01234567"));
            assert!(is_valid_oct("777"));
        }

        #[test]
        fn is_valid_oct_rejects_invalid() {
            assert!(!is_valid_oct("8"));
            assert!(!is_valid_oct("9"));
            assert!(!is_valid_oct("a"));
        }

        #[test]
        fn is_valid_oct_rejects_empty() {
            assert!(!is_valid_oct(""));
        }

        #[test]
        fn is_valid_dec_accepts_valid() {
            assert!(is_valid_dec("0123456789"));
            assert!(is_valid_dec("999"));
        }

        #[test]
        fn is_valid_dec_rejects_invalid() {
            assert!(!is_valid_dec("a"));
            assert!(!is_valid_dec("12a34"));
        }

        #[test]
        fn is_valid_dec_rejects_empty() {
            assert!(!is_valid_dec(""));
        }

        #[test]
        fn is_valid_hex_accepts_valid() {
            assert!(is_valid_hex("0123456789abcdef"));
            assert!(is_valid_hex("ABCDEF"));
            assert!(is_valid_hex("ff"));
        }

        #[test]
        fn is_valid_hex_rejects_invalid() {
            assert!(!is_valid_hex("g"));
            assert!(!is_valid_hex("xyz"));
        }

        #[test]
        fn is_valid_hex_rejects_empty() {
            assert!(!is_valid_hex(""));
        }
    }

    // ==================== Prefix stripping tests ====================

    mod prefix_tests {
        use super::*;

        #[test]
        fn strips_0b_prefix_for_binary() {
            assert_eq!(strip_prefix("0b1010", Base::Bin), "1010");
            assert_eq!(strip_prefix("0B1010", Base::Bin), "1010");
        }

        #[test]
        fn strips_0o_prefix_for_octal() {
            assert_eq!(strip_prefix("0o777", Base::Oct), "777");
            assert_eq!(strip_prefix("0O755", Base::Oct), "755");
        }

        #[test]
        fn strips_0x_prefix_for_hex() {
            assert_eq!(strip_prefix("0xff", Base::Hex), "ff");
            assert_eq!(strip_prefix("0XFF", Base::Hex), "ff");
        }

        #[test]
        fn no_strip_for_decimal() {
            assert_eq!(strip_prefix("123", Base::Dec), "123");
        }

        #[test]
        fn no_strip_when_no_prefix() {
            assert_eq!(strip_prefix("1010", Base::Bin), "1010");
            assert_eq!(strip_prefix("777", Base::Oct), "777");
            assert_eq!(strip_prefix("ff", Base::Hex), "ff");
        }
    }

    // ==================== Value::from with prefix tests ====================

    mod from_with_prefix {
        use super::*;

        #[test]
        fn parses_binary_with_0b_prefix() {
            let val = Value::from("0b1010".to_string(), Base::Bin).unwrap();
            assert_eq!(val.to_base(Base::Dec), "10");
        }

        #[test]
        fn parses_octal_with_0o_prefix() {
            let val = Value::from("0o777".to_string(), Base::Oct).unwrap();
            assert_eq!(val.to_base(Base::Dec), "511");
        }

        #[test]
        fn parses_hex_with_0x_prefix() {
            let val = Value::from("0xff".to_string(), Base::Hex).unwrap();
            assert_eq!(val.to_base(Base::Dec), "255");
        }
    }
}
