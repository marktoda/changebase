# changebase

A CLI tool for changing the base of numbers. 

```
> changebase -h

numeric base converter

USAGE:
    changebase [FLAGS] [OPTIONS] <value>

FLAGS:
        --ib         use binary as input base
        --ob         use binary as output base
        --id         use decimal as input base
        --od         use decimal as output base
        --ih         use hex as input base
        --oh         use hex as output base
        --io         use octal as input base
        --oo         use octal as output base
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --input <input>      Input base to use [possible values: Bin, Oct, Dec, Hex]
    -o, --output <output>    Output base to use [possible values: Bin, Oct, Dec, Hex]

ARGS:
    <value> 

```
