//! Command-line argument parsing and base definitions.
//!
//! This module defines the CLI interface using clap, including all flags and
//! options for specifying input/output bases.

use crate::base::detect_base;
use crate::errors::BaseError;
use clap::{Args, Parser, ValueEnum};

/// Supported numeric bases for conversion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum Base {
    /// Binary (base 2)
    Bin,
    /// Octal (base 8)
    Oct,
    /// Decimal (base 10)
    Dec,
    /// Hexadecimal (base 16)
    Hex,
}

/// All supported bases in display order (binary, octal, decimal, hex).
pub const ALL_BASES: [Base; 4] = [Base::Bin, Base::Oct, Base::Dec, Base::Hex];

impl Base {
    /// Returns the full name of the base (e.g., "Hexadecimal").
    pub fn repr(&self) -> String {
        match *self {
            Base::Bin => "Binary".to_string(),
            Base::Oct => "Octal".to_string(),
            Base::Dec => "Decimal".to_string(),
            Base::Hex => "Hexadecimal".to_string(),
        }
    }

    /// Returns the short label for display (e.g., "hex").
    pub fn short_label(&self) -> &'static str {
        match *self {
            Base::Bin => "bin",
            Base::Oct => "oct",
            Base::Dec => "dec",
            Base::Hex => "hex",
        }
    }
}

/// Command-line options for changebase.
///
/// Supports multiple ways to specify input/output bases:
/// - Long flags: `--input dec --output hex`
/// - Short flags: `-i dec -o hex`
/// - Shorthand flags: `--id --oh`
#[derive(Clone, Debug, Parser)]
#[command(name = "changebase", about = "A fast CLI tool for converting numbers between bases")]
pub struct Opt {
    /// Input base to use. If not given, attempts to auto-detect.
    #[arg(long = "input", short = 'i', value_enum, ignore_case = true)]
    pub input: Option<Base>,

    /// Output base to use. If not given, shows all bases.
    #[arg(long = "output", short = 'o', value_enum, ignore_case = true)]
    pub output: Option<Base>,

    /// The value to convert
    pub value: String,

    #[command(flatten)]
    short_base_opts: ShortBaseOpts,

    /// Enable verbose output showing conversion details
    #[arg(short)]
    pub verbose: bool,
}

#[derive(Clone, Debug, Args)]
struct ShortBaseOpts {
    /// use binary as input base
    #[arg(long = "ib")]
    pub binary_input: bool,

    /// use octal as input base
    #[arg(long = "io")]
    pub octal_input: bool,

    /// use decimal as input base
    #[arg(long = "id")]
    pub decimal_input: bool,

    /// use hex as input base
    #[arg(long = "ih")]
    pub hex_input: bool,

    /// use binary as output base
    #[arg(long = "ob")]
    pub binary_output: bool,

    /// use octal as output base
    #[arg(long = "oo")]
    pub octal_output: bool,

    /// use decimal as output base
    #[arg(long = "od")]
    pub decimal_output: bool,

    /// use hex as output base
    #[arg(long = "oh")]
    pub hex_output: bool,
}

impl Opt {
    /// Determines the input base from CLI arguments or auto-detection.
    ///
    /// Priority order:
    /// 1. Explicit `--input` / `-i` flag
    /// 2. Shorthand flags (`--ib`, `--io`, `--id`, `--ih`)
    /// 3. Auto-detection from value content/prefix
    ///
    /// When auto-detecting, prints the detected base to stdout.
    pub fn get_input(&self) -> Result<Base, BaseError> {
        if let Some(base) = self.input {
            Ok(base)
        } else if self.short_base_opts.binary_input {
            Ok(Base::Bin)
        } else if self.short_base_opts.octal_input {
            Ok(Base::Oct)
        } else if self.short_base_opts.decimal_input {
            Ok(Base::Dec)
        } else if self.short_base_opts.hex_input {
            Ok(Base::Hex)
        } else {
            detect_base(&self.value).inspect(|b| println!("Detected base {}", b.repr()))
        }
    }

    /// Determines the output base from CLI arguments.
    ///
    /// Returns `None` if no output base specified, indicating all bases
    /// should be displayed.
    ///
    /// Priority order:
    /// 1. Explicit `--output` / `-o` flag
    /// 2. Shorthand flags (`--ob`, `--oo`, `--od`, `--oh`)
    /// 3. `None` (show all bases)
    pub fn get_output(&self) -> Option<Base> {
        if let Some(base) = self.output {
            Some(base)
        } else if self.short_base_opts.binary_output {
            Some(Base::Bin)
        } else if self.short_base_opts.octal_output {
            Some(Base::Oct)
        } else if self.short_base_opts.decimal_output {
            Some(Base::Dec)
        } else if self.short_base_opts.hex_output {
            Some(Base::Hex)
        } else {
            None // Show all bases
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to create an Opt struct for testing
    fn make_opt(
        input: Option<Base>,
        output: Option<Base>,
        value: &str,
        ib: bool,
        io: bool,
        id: bool,
        ih: bool,
        ob: bool,
        oo: bool,
        od: bool,
        oh: bool,
        verbose: bool,
    ) -> Opt {
        Opt {
            input,
            output,
            value: value.to_string(),
            short_base_opts: ShortBaseOpts {
                binary_input: ib,
                octal_input: io,
                decimal_input: id,
                hex_input: ih,
                binary_output: ob,
                octal_output: oo,
                decimal_output: od,
                hex_output: oh,
            },
            verbose,
        }
    }

    // Simplified helper for common cases
    fn make_simple_opt(input: Option<Base>, output: Option<Base>, value: &str) -> Opt {
        make_opt(
            input, output, value, false, false, false, false, false, false, false, false, false,
        )
    }

    // ==================== Base::repr tests ====================

    mod base_repr {
        use super::*;

        #[test]
        fn binary_repr() {
            assert_eq!(Base::Bin.repr(), "Binary");
        }

        #[test]
        fn octal_repr() {
            assert_eq!(Base::Oct.repr(), "Octal");
        }

        #[test]
        fn decimal_repr() {
            assert_eq!(Base::Dec.repr(), "Decimal");
        }

        #[test]
        fn hex_repr() {
            assert_eq!(Base::Hex.repr(), "Hexadecimal");
        }
    }

    // ==================== get_input tests ====================

    mod get_input {
        use super::*;

        #[test]
        fn returns_explicit_input_base() {
            let opt = make_simple_opt(Some(Base::Dec), Some(Base::Hex), "255");
            assert!(matches!(opt.get_input().unwrap(), Base::Dec));
        }

        #[test]
        fn explicit_input_takes_precedence_over_shorthand() {
            // Even with --ib set, --input dec should win
            let opt = make_opt(
                Some(Base::Dec),
                None,
                "255",
                true,
                false,
                false,
                false, // ib=true
                false,
                false,
                false,
                false,
                false,
            );
            assert!(matches!(opt.get_input().unwrap(), Base::Dec));
        }

        #[test]
        fn shorthand_ib_returns_binary() {
            let opt = make_opt(
                None, None, "1010", true, false, false, false, false, false, false, false, false,
            );
            assert!(matches!(opt.get_input().unwrap(), Base::Bin));
        }

        #[test]
        fn shorthand_io_returns_octal() {
            let opt = make_opt(
                None, None, "777", false, true, false, false, false, false, false, false, false,
            );
            assert!(matches!(opt.get_input().unwrap(), Base::Oct));
        }

        #[test]
        fn shorthand_id_returns_decimal() {
            let opt = make_opt(
                None, None, "255", false, false, true, false, false, false, false, false, false,
            );
            assert!(matches!(opt.get_input().unwrap(), Base::Dec));
        }

        #[test]
        fn shorthand_ih_returns_hex() {
            let opt = make_opt(
                None, None, "ff", false, false, false, true, false, false, false, false, false,
            );
            assert!(matches!(opt.get_input().unwrap(), Base::Hex));
        }

        #[test]
        fn auto_detects_binary_with_prefix() {
            // Binary now requires 0b prefix for auto-detection
            let opt = make_simple_opt(None, Some(Base::Dec), "0b1010");
            assert!(matches!(opt.get_input().unwrap(), Base::Bin));
        }

        #[test]
        fn pure_digits_default_to_decimal() {
            // "1010" without prefix now defaults to decimal
            let opt = make_simple_opt(None, Some(Base::Dec), "1010");
            assert!(matches!(opt.get_input().unwrap(), Base::Dec));
        }

        #[test]
        fn auto_detects_hex_with_letters() {
            let opt = make_simple_opt(None, Some(Base::Dec), "abc");
            assert!(matches!(opt.get_input().unwrap(), Base::Hex));
        }

        #[test]
        fn auto_detects_hex_with_0x_prefix() {
            let opt = make_simple_opt(None, Some(Base::Dec), "0xff");
            assert!(matches!(opt.get_input().unwrap(), Base::Hex));
        }

        #[test]
        fn shorthand_precedence_ib_over_io() {
            // First true shorthand wins (binary before octal)
            let opt = make_opt(
                None, None, "777", true, true, false, false, // both ib and io
                false, false, false, false, false,
            );
            assert!(matches!(opt.get_input().unwrap(), Base::Bin));
        }
    }

    // ==================== get_output tests ====================

    mod get_output {
        use super::*;

        #[test]
        fn returns_explicit_output_base() {
            let opt = make_simple_opt(Some(Base::Dec), Some(Base::Hex), "255");
            assert_eq!(opt.get_output(), Some(Base::Hex));
        }

        #[test]
        fn explicit_output_takes_precedence_over_shorthand() {
            let opt = make_opt(
                None,
                Some(Base::Hex),
                "255",
                false,
                false,
                false,
                false,
                true,
                false,
                false,
                false, // ob=true
                false,
            );
            assert_eq!(opt.get_output(), Some(Base::Hex));
        }

        #[test]
        fn shorthand_ob_returns_binary() {
            let opt = make_opt(
                None, None, "255", false, false, false, false, true, false, false, false, false,
            );
            assert_eq!(opt.get_output(), Some(Base::Bin));
        }

        #[test]
        fn shorthand_oo_returns_octal() {
            let opt = make_opt(
                None, None, "255", false, false, false, false, false, true, false, false, false,
            );
            assert_eq!(opt.get_output(), Some(Base::Oct));
        }

        #[test]
        fn shorthand_od_returns_decimal() {
            let opt = make_opt(
                None, None, "ff", false, false, false, false, false, false, true, false, false,
            );
            assert_eq!(opt.get_output(), Some(Base::Dec));
        }

        #[test]
        fn shorthand_oh_returns_hex() {
            let opt = make_opt(
                None, None, "255", false, false, false, false, false, false, false, true, false,
            );
            assert_eq!(opt.get_output(), Some(Base::Hex));
        }

        #[test]
        fn returns_none_when_no_output_specified() {
            let opt = make_simple_opt(Some(Base::Dec), None, "255");
            assert_eq!(opt.get_output(), None);
        }

        #[test]
        fn shorthand_precedence_ob_over_oo() {
            let opt = make_opt(
                None, None, "255", false, false, false, false, true, true, false,
                false, // both ob and oo
                false,
            );
            assert_eq!(opt.get_output(), Some(Base::Bin));
        }
    }

    // ==================== CLI parsing tests ====================

    mod cli_parsing {
        use super::*;

        #[test]
        fn parses_long_input_flag() {
            let opt =
                Opt::try_parse_from(["changebase", "--input", "dec", "--output", "hex", "255"])
                    .unwrap();
            assert!(matches!(opt.input, Some(Base::Dec)));
        }

        #[test]
        fn parses_short_input_flag() {
            let opt = Opt::try_parse_from(["changebase", "-i", "dec", "-o", "hex", "255"]).unwrap();
            assert!(matches!(opt.input, Some(Base::Dec)));
        }

        #[test]
        fn parses_case_insensitive_base() {
            let opt = Opt::try_parse_from(["changebase", "-i", "DEC", "-o", "HEX", "255"]).unwrap();
            assert!(matches!(opt.input, Some(Base::Dec)));
            assert!(matches!(opt.output, Some(Base::Hex)));
        }

        #[test]
        fn parses_shorthand_flags() {
            let opt = Opt::try_parse_from(["changebase", "--id", "--oh", "255"]).unwrap();
            assert!(opt.short_base_opts.decimal_input);
            assert!(opt.short_base_opts.hex_output);
        }

        #[test]
        fn parses_verbose_flag() {
            let opt = Opt::try_parse_from(["changebase", "-v", "--id", "--oh", "255"]).unwrap();
            assert!(opt.verbose);
        }

        #[test]
        fn parses_value_argument() {
            let opt = Opt::try_parse_from(["changebase", "--id", "--oh", "12345"]).unwrap();
            assert_eq!(opt.value, "12345");
        }

        #[test]
        fn parses_hex_value_with_prefix() {
            let opt = Opt::try_parse_from(["changebase", "--ih", "--od", "0xff"]).unwrap();
            assert_eq!(opt.value, "0xff");
        }

        #[test]
        fn rejects_invalid_base() {
            let result = Opt::try_parse_from(["changebase", "-i", "invalid", "-o", "hex", "255"]);
            assert!(result.is_err());
        }

        #[test]
        fn rejects_missing_value() {
            let result = Opt::try_parse_from(["changebase", "--id", "--oh"]);
            assert!(result.is_err());
        }
    }
}
