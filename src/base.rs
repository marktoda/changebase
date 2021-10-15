use crate::errors::BaseError;
use crate::opts::Base;
use std::u128;

impl Base {
    pub fn repr(&self) -> String {
        match *self {
            Base::Bin => "Binary".to_string(),
            Base::Oct => "Octal".to_string(),
            Base::Dec => "Decimal".to_string(),
            Base::Hex => "Hexadecimal".to_string(),
        }
    }

    pub fn validate(&self, value: String) -> Result<(), BaseError> {
        if match *self {
            Base::Bin => is_valid_bin(value),
            Base::Oct => is_valid_oct(value),
            Base::Dec => is_valid_dec(value),
            Base::Hex => is_valid_hex(value),
        } {
            Ok(())
        } else {
            Err(self.get_parse_error())
        }
    }

    pub fn get_parse_error(&self) -> BaseError {
        return match *self {
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

    pub fn to_u128(&self, value: String) -> Result<u128, BaseError> {
        self.validate(value.clone())?;

        match *self {
            Base::Bin => u128::from_str_radix(value.as_str(), 2),
            Base::Oct => u128::from_str_radix(value.as_str(), 8),
            Base::Dec => u128::from_str_radix(value.as_str(), 10),
            Base::Hex => u128::from_str_radix(value.trim_start_matches("0x"), 16),
        }
        .map_err(|_| self.get_parse_error())
    }

    pub fn from_u128(&self, value: u128) -> String {
        match *self {
            Base::Bin => format!("{:b}", value),
            Base::Oct => format!("{:o}", value),
            Base::Dec => format!("{}", value),
            Base::Hex => format!("{:x}", value),
        }
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
