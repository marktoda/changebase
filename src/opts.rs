use clap::arg_enum;
use structopt::StructOpt;
use crate::errors::BaseError;

arg_enum! {
    #[derive(Debug, Clone)]
    pub enum Base {
        Bin,
        Oct,
        Dec,
        Hex,
    }
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

#[derive(Clone, Debug, StructOpt)]
#[structopt(name = "base", about = "numeric base converter")]
pub struct Opt {
    /// Input base to use
    #[structopt(
        long = "input",
        short = "in",
        possible_values = &Base::variants(),
        case_insensitive = true,
    )]
    pub input: Option<Base>,

    /// Output base to use
    #[structopt(
        long = "output",
        short = "out",
        possible_values = &Base::variants(),
        case_insensitive = true,
    )]
    pub output: Option<Base>,

    pub value: String,

    #[structopt(flatten)]
    short_base_opts: ShortBaseOpts,

    /// add verbosity
    #[structopt(short)]
    pub verbose: bool,
}


#[derive(Clone, Debug, StructOpt)]
struct ShortBaseOpts {
    /// use binary as input base
    #[structopt(
        long = "ib",
    )]
    pub binary_input: bool,

    /// use octal as input base
    #[structopt(
        long = "io",
    )]
    pub octal_input: bool,

    /// use decimal as input base
    #[structopt(
        long = "id",
    )]
    pub decimal_input: bool,

    /// use hex as input base
    #[structopt(
        long = "ih",
    )]
    pub hex_input: bool,

    /// use binary as output base
    #[structopt(
        long = "ob",
    )]
    pub binary_output: bool,

    /// use octal as output base
    #[structopt(
        long = "oo",
    )]
    pub octal_output: bool,

    /// use decimal as output base
    #[structopt(
        long = "od",
    )]
    pub decimal_output: bool,

    /// use hex as output base
    #[structopt(
        long = "oh",
    )]
    pub hex_output: bool,
}

impl Opt {
    pub fn get_input(&self) -> Result<Base, BaseError> {
        if self.input.is_some() {
            Ok(self.input.clone().unwrap())
        } else if self.short_base_opts.binary_input {
            Ok(Base::Bin)
        } else if self.short_base_opts.octal_input {
            Ok(Base::Oct)
        } else if self.short_base_opts.decimal_input {
            Ok(Base::Dec)
        } else if self.short_base_opts.hex_input {
            Ok(Base::Hex)
        } else {
            Err(BaseError::ArgError { message: "No input base specified" })
        }
    }

    pub fn get_output(&self) -> Result<Base, BaseError> {
        if self.output.is_some() {
            Ok(self.output.clone().unwrap())
        } else if self.short_base_opts.binary_output {
            Ok(Base::Bin)
        } else if self.short_base_opts.octal_output {
            Ok(Base::Oct)
        } else if self.short_base_opts.decimal_output {
            Ok(Base::Dec)
        } else if self.short_base_opts.hex_output {
            Ok(Base::Hex)
        } else {
            Err(BaseError::ArgError { message: "No output base specified" })
        }
    }
}
