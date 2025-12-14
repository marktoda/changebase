use crate::errors::BaseError;
use crate::opts::Base;
use num::{bigint::BigUint, Num};

pub struct Value {
    value: BigUint,
}

impl Value {
    pub fn from(value: String, base: Base) -> Result<Value, BaseError> {
        Value::validate(base.clone(), value.clone())?;

        match base {
            Base::Bin => BigUint::from_str_radix(value.as_str(), 2),
            Base::Oct => BigUint::from_str_radix(value.as_str(), 8),
            Base::Dec => BigUint::from_str_radix(value.as_str(), 10),
            Base::Hex => BigUint::from_str_radix(value.trim_start_matches("0x"), 16),
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
        if match base {
            Base::Bin => is_valid_bin(value),
            Base::Oct => is_valid_oct(value),
            Base::Dec => is_valid_dec(value),
            Base::Hex => is_valid_hex(value),
        } {
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

fn is_valid_bin(value: String) -> bool {
    for c in value.chars() {
        if !(c == '0' || c == '1') {
            return false;
        }
    }
    return true;
}

fn is_valid_oct(value: String) -> bool {
    for c in value.chars() {
        if !("01234567".contains(c)) {
            return false;
        }
    }
    return true;
}

fn is_valid_dec(value: String) -> bool {
    for c in value.chars() {
        if !("0123456789".contains(c)) {
            return false;
        }
    }
    return true;
}

fn is_valid_hex(value: String) -> bool {
    for c in value.to_lowercase().chars() {
        if !("0123456789abcdefx".contains(c)) {
            return false;
        }
    }
    return true;
}

pub fn detect_base(value: String) -> Result<Base, BaseError> {
    if is_valid_bin(value.clone()) {
        return Ok(Base::Bin);
    };
    if is_valid_oct(value.clone()) {
        return Ok(Base::Oct);
    };
    if is_valid_dec(value.clone()) {
        return Ok(Base::Dec);
    };
    if is_valid_hex(value) {
        return Ok(Base::Hex);
    };

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

        #[test]
        fn detects_binary_only_zeros_and_ones() {
            let result = detect_base("101010".to_string()).unwrap();
            assert!(matches!(result, Base::Bin));
        }

        #[test]
        fn detects_octal_with_digits_2_to_7() {
            let result = detect_base("234567".to_string()).unwrap();
            assert!(matches!(result, Base::Oct));
        }

        #[test]
        fn detects_decimal_with_digits_8_or_9() {
            let result = detect_base("12389".to_string()).unwrap();
            assert!(matches!(result, Base::Dec));
        }

        #[test]
        fn detects_hex_with_letters() {
            let result = detect_base("abc123".to_string()).unwrap();
            assert!(matches!(result, Base::Hex));
        }

        #[test]
        fn detects_hex_with_0x_prefix() {
            let result = detect_base("0xff".to_string()).unwrap();
            assert!(matches!(result, Base::Hex));
        }

        #[test]
        fn ambiguous_101_detected_as_binary() {
            // This documents the current behavior - "101" is detected as binary
            let result = detect_base("101".to_string()).unwrap();
            assert!(matches!(result, Base::Bin));
        }

        #[test]
        fn ambiguous_777_detected_as_octal() {
            // "777" with only 0-7 digits is detected as octal (after failing binary)
            let result = detect_base("777".to_string()).unwrap();
            assert!(matches!(result, Base::Oct));
        }

        #[test]
        fn ambiguous_999_detected_as_decimal() {
            // "999" with 8/9 is detected as decimal
            let result = detect_base("999".to_string()).unwrap();
            assert!(matches!(result, Base::Dec));
        }

        #[test]
        fn fails_on_invalid_characters() {
            let result = detect_base("xyz".to_string());
            assert!(result.is_err());
        }

        #[test]
        fn fails_on_special_characters() {
            let result = detect_base("12+34".to_string());
            assert!(result.is_err());
        }

        #[test]
        fn empty_string_detected_as_binary() {
            // Empty string passes all validations (vacuous truth)
            // This documents current behavior
            let result = detect_base("".to_string());
            assert!(result.is_ok());
        }
    }

    // ==================== Validation function tests ====================

    mod validation_tests {
        use super::*;

        #[test]
        fn is_valid_bin_accepts_valid() {
            assert!(is_valid_bin("0".to_string()));
            assert!(is_valid_bin("1".to_string()));
            assert!(is_valid_bin("010101".to_string()));
        }

        #[test]
        fn is_valid_bin_rejects_invalid() {
            assert!(!is_valid_bin("2".to_string()));
            assert!(!is_valid_bin("a".to_string()));
            assert!(!is_valid_bin("01201".to_string()));
        }

        #[test]
        fn is_valid_oct_accepts_valid() {
            assert!(is_valid_oct("01234567".to_string()));
            assert!(is_valid_oct("777".to_string()));
        }

        #[test]
        fn is_valid_oct_rejects_invalid() {
            assert!(!is_valid_oct("8".to_string()));
            assert!(!is_valid_oct("9".to_string()));
            assert!(!is_valid_oct("a".to_string()));
        }

        #[test]
        fn is_valid_dec_accepts_valid() {
            assert!(is_valid_dec("0123456789".to_string()));
            assert!(is_valid_dec("999".to_string()));
        }

        #[test]
        fn is_valid_dec_rejects_invalid() {
            assert!(!is_valid_dec("a".to_string()));
            assert!(!is_valid_dec("12a34".to_string()));
        }

        #[test]
        fn is_valid_hex_accepts_valid() {
            assert!(is_valid_hex("0123456789abcdef".to_string()));
            assert!(is_valid_hex("ABCDEF".to_string()));
            assert!(is_valid_hex("0xff".to_string()));
            assert!(is_valid_hex("0xFF".to_string()));
        }

        #[test]
        fn is_valid_hex_rejects_invalid() {
            assert!(!is_valid_hex("g".to_string()));
            assert!(!is_valid_hex("xyz".to_string()));
        }
    }
}
