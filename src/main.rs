use clap::Parser;

mod opts;
use opts::Opt;
mod base;
use base::Value;
mod errors;
use errors::BaseError;

fn main() {
    let opt = Opt::parse();

    let result = convert_base(opt);
    match result {
        Ok(val) => println!("{}", val),
        Err(e) => {
            match e {
                BaseError::ParseError { message } => {
                    eprintln!("Error parsing value: {}", message)
                }
                BaseError::ArgError { message } => {
                    eprintln!("Invalid arguments: {}", message)
                }
            }
            std::process::exit(1);
        }
    }
}

fn convert_base(opt: Opt) -> Result<String, BaseError> {
    let input = opt.get_input()?;
    let output = opt.get_output()?;
    if opt.verbose {
        println!(
            "Converting {} from {} to {}",
            &opt.value,
            input.repr(),
            output.repr()
        );
    }

    let num = Value::from(opt.value, input)?;
    Ok(num.to_base(output))
}
