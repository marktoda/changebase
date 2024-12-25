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
