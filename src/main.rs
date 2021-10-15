use structopt::StructOpt;

mod opts;
use opts::Opt;
mod base;
mod errors;
use errors::BaseError;

fn main() {
    let opt = Opt::from_args();

    let result = convert_base(opt);
    if let Ok(val) = result {
        println!("{}", val);
    } else if let Err(e) = result {
        match e {
            BaseError::ParseError { message } => {
                eprintln!("Error parsing value: {}", message)
            },
            BaseError::ArgError { message } => {
                eprintln!("Invalid arguments: {}", message)
            }
        }
    }
}

fn convert_base(opt: Opt) -> Result<String, BaseError> {
    let input = opt.get_input()?;
    let output = opt.get_output()?;
    println!(
        "Converting {} from {} to {}",
        &opt.value,
        input.repr(),
        output.repr()
    );

    let num = input.to_u128(opt.value)?;
    Ok(output.from_u128(num))
}