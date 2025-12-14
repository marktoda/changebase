# changebase

A fast, lightweight CLI tool for converting numbers between bases (binary, octal, decimal, hexadecimal).

## Features

- **Smart auto-detection**: Automatically detects input base from prefixes (`0b`, `0o`, `0x`) or content
- **Show all bases**: When no output base is specified, displays the number in all four bases
- **Arbitrary precision**: Handles numbers larger than 64-bit using BigUint
- **Flexible input**: Supports both long flags (`--input dec`) and shorthand (`--id`)
- **Prefix support**: Accepts standard prefixes (`0b1010`, `0o777`, `0xff`)

## Installation

```sh
cargo install changebase
```

Or build from source:

```sh
git clone https://github.com/marktoda/changebase
cd changebase
cargo build --release
```

## Usage

### Basic conversion

```sh
# Convert decimal 255 to hexadecimal
changebase --id --oh 255
# Output: ff

# Convert hex to decimal
changebase --ih --od ff
# Output: 255

# Convert binary to decimal
changebase --ib --od 11111111
# Output: 255
```

### Show all bases (default)

When no output base is specified, changebase displays the number in all bases.
The input base is marked with an asterisk (`*`):

```sh
changebase --id 255
# Output:
# bin: 11111111
# oct: 377
# dec: 255 *
# hex: ff

changebase --ih ff
# Output:
# bin: 11111111
# oct: 377
# dec: 255
# hex: ff *
```

### Auto-detection

If no input base is specified, changebase auto-detects based on:

1. **Prefix**: `0b` → binary, `0o` → octal, `0x` → hex
2. **Content**: Contains `a-f` → hex
3. **Default**: Pure digits → decimal

```sh
# Hex prefix detected
changebase --od 0xff
# Detected base Hexadecimal
# 255

# Hex letters detected
changebase --od deadbeef
# Detected base Hexadecimal
# 3735928559

# Pure digits default to decimal
changebase --oh 255
# Detected base Decimal
# ff

# Use prefixes for binary/octal without flags
changebase --od 0b1010
# Detected base Binary
# 10
```

### Using long flags

```sh
# Long form flags
changebase --input dec --output hex 255
# Output: ff

# Short form flags
changebase -i dec -o hex 255
# Output: ff
```

### Verbose mode

```sh
changebase -v --id --oh 255
# Converting 255 from Decimal to Hexadecimal
# ff
```

## Flag Reference

### Input base flags

| Flag | Long form | Description |
|------|-----------|-------------|
| `--ib` | `--input bin` or `-i bin` | Binary input |
| `--io` | `--input oct` or `-i oct` | Octal input |
| `--id` | `--input dec` or `-i dec` | Decimal input |
| `--ih` | `--input hex` or `-i hex` | Hexadecimal input |

### Output base flags

| Flag | Long form | Description |
|------|-----------|-------------|
| `--ob` | `--output bin` or `-o bin` | Binary output |
| `--oo` | `--output oct` or `-o oct` | Octal output |
| `--od` | `--output dec` or `-o dec` | Decimal output |
| `--oh` | `--output hex` or `-o hex` | Hexadecimal output |

### Other flags

| Flag | Description |
|------|-------------|
| `-v` | Verbose mode - show conversion details |
| `-h, --help` | Show help information |
| `-V, --version` | Show version |

## Examples

### Common use cases

```sh
# Check bit pattern of a byte
changebase --id --ob 255
# Output: 11111111

# Convert RGB color value
changebase --ih --od ff
# Output: 255

# Unix file permissions
changebase --io --ob 755
# Output: 111101101

# Memory address
changebase --ih --od deadbeef
# Output: 3735928559
```

### Large numbers

changebase supports arbitrary-precision arithmetic:

```sh
# 64-bit maximum
changebase --id --oh 18446744073709551615
# Output: ffffffffffffffff

# Beyond 64-bit
changebase --id --oh 18446744073709551616
# Output: 10000000000000000
```

## License

GPL-3.0
