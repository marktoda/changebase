use crate::base::detect_base;
use crate::errors::BaseError;
use clap::{Parser, Args, ValueEnum};

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Base {
    Bin,
    Oct,
    Dec,
    Hex,
}

impl Base {
    pub fn repr(&self) -> String {
        match *self {
            Base::Bin => "Binary".to_string(),
            Base::Oct => "Octal".to_string(),
            Base::Dec => "Decimal".to_string(),
            Base::Hex => "Hexadecimal".to_string(),
        }
    }
}

#[derive(Clone, Debug, Parser)]
#[command(name = "changebase", about = "numeric base converter")]
pub struct Opt {
    /// Input base to use. If not given, attempts to detect
    #[arg(long = "input", short = 'i', value_enum, ignore_case = true)]
    pub input: Option<Base>,

    /// Output base to use
    #[arg(long = "output", short = 'o', value_enum, ignore_case = true)]
    pub output: Option<Base>,

    pub value: String,

    #[command(flatten)]
    short_base_opts: ShortBaseOpts,

    /// add verbosity
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
            detect_base(self.value.clone())
                .map_err(|_| BaseError::ArgError {
                    message: "No input base specified",
                })
                .inspect(|b| println!("Detected base {}", b.repr()))
        }
    }

    pub fn get_output(&self) -> Result<Base, BaseError> {
        if let Some(base) = self.output {
            Ok(base)
        } else if self.short_base_opts.binary_output {
            Ok(Base::Bin)
        } else if self.short_base_opts.octal_output {
            Ok(Base::Oct)
        } else if self.short_base_opts.decimal_output {
            Ok(Base::Dec)
        } else if self.short_base_opts.hex_output {
            Ok(Base::Hex)
        } else {
            Err(BaseError::ArgError {
                message: "No output base specified",
            })
        }
    }
}
