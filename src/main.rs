//! changebase - A fast CLI tool for converting numbers between bases.
//!
//! Supports binary, octal, decimal, and hexadecimal with smart auto-detection
//! and arbitrary-precision arithmetic.

use clap::Parser;

mod base;
mod errors;
mod opts;

use base::Value;
use errors::BaseError;
use opts::{Base, Opt, ALL_BASES};

fn main() {
    let opt = Opt::parse();

    let result = convert_base(&opt);
    match result {
        Ok(output) => print!("{}", output),
        Err(e) => {
            match e {
                BaseError::ParseError { message } => {
                    eprintln!("Error parsing value: {}", message)
                }
            }
            std::process::exit(1);
        }
    }
}

/// Performs the base conversion based on CLI options.
///
/// Returns the formatted output string, or an error if parsing fails.
fn convert_base(opt: &Opt) -> Result<String, BaseError> {
    let input = opt.get_input()?;
    let output = opt.get_output();

    let num = Value::from(opt.value.clone(), input)?;

    match output {
        Some(base) => {
            if opt.verbose {
                println!(
                    "Converting {} from {} to {}",
                    &opt.value,
                    input.repr(),
                    base.repr()
                );
            }
            Ok(format!("{}\n", num.to_base(base)))
        }
        None => {
            if opt.verbose {
                println!("Converting {} from {}", &opt.value, input.repr());
            }
            Ok(format_all_bases(&num, input))
        }
    }
}

/// Formats a number in all supported bases.
///
/// The input base is marked with an asterisk (*) for easy identification.
fn format_all_bases(num: &Value, input_base: Base) -> String {
    let mut output = String::new();
    for base in ALL_BASES {
        let marker = if base == input_base { " *" } else { "" };
        output.push_str(&format!(
            "{}: {}{}\n",
            base.short_label(),
            num.to_base(base),
            marker
        ));
    }
    output
}
