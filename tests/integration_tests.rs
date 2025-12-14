use std::process::Command;

/// Helper to run changebase CLI and capture output
fn run_changebase(args: &[&str]) -> (String, String, bool) {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--quiet")
        .arg("--")
        .args(args)
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let success = output.status.success();

    (stdout.trim().to_string(), stderr.trim().to_string(), success)
}

// ==================== Basic conversion tests ====================

mod basic_conversions {
    use super::*;

    #[test]
    fn decimal_to_hex() {
        let (stdout, _, success) = run_changebase(&["--id", "--oh", "255"]);
        assert!(success);
        assert_eq!(stdout, "ff");
    }

    #[test]
    fn decimal_to_binary() {
        let (stdout, _, success) = run_changebase(&["--id", "--ob", "42"]);
        assert!(success);
        assert_eq!(stdout, "101010");
    }

    #[test]
    fn decimal_to_octal() {
        let (stdout, _, success) = run_changebase(&["--id", "--oo", "64"]);
        assert!(success);
        assert_eq!(stdout, "100");
    }

    #[test]
    fn hex_to_decimal() {
        let (stdout, _, success) = run_changebase(&["--ih", "--od", "ff"]);
        assert!(success);
        assert_eq!(stdout, "255");
    }

    #[test]
    fn binary_to_decimal() {
        let (stdout, _, success) = run_changebase(&["--ib", "--od", "11111111"]);
        assert!(success);
        assert_eq!(stdout, "255");
    }

    #[test]
    fn octal_to_decimal() {
        let (stdout, _, success) = run_changebase(&["--io", "--od", "377"]);
        assert!(success);
        assert_eq!(stdout, "255");
    }
}

// ==================== Long flag tests ====================

mod long_flags {
    use super::*;

    #[test]
    fn input_output_flags() {
        let (stdout, _, success) = run_changebase(&["--input", "dec", "--output", "hex", "255"]);
        assert!(success);
        assert_eq!(stdout, "ff");
    }

    #[test]
    fn short_io_flags() {
        let (stdout, _, success) = run_changebase(&["-i", "dec", "-o", "hex", "255"]);
        assert!(success);
        assert_eq!(stdout, "ff");
    }

    #[test]
    fn case_insensitive_base_names() {
        let (stdout, _, success) = run_changebase(&["-i", "DEC", "-o", "HEX", "255"]);
        assert!(success);
        assert_eq!(stdout, "ff");
    }
}

// ==================== Hex prefix handling ====================

mod hex_prefix {
    use super::*;

    #[test]
    fn accepts_0x_prefix() {
        let (stdout, _, success) = run_changebase(&["--ih", "--od", "0xff"]);
        assert!(success);
        assert_eq!(stdout, "255");
    }

    #[test]
    fn accepts_0x_uppercase() {
        let (stdout, _, success) = run_changebase(&["--ih", "--od", "0xFF"]);
        assert!(success);
        assert_eq!(stdout, "255");
    }

    #[test]
    fn works_without_prefix() {
        let (stdout, _, success) = run_changebase(&["--ih", "--od", "ff"]);
        assert!(success);
        assert_eq!(stdout, "255");
    }
}

// ==================== Auto-detection tests ====================

mod auto_detection {
    use super::*;

    #[test]
    fn detects_hex_with_letters() {
        let (stdout, _, success) = run_changebase(&["--od", "abc"]);
        assert!(success);
        assert_eq!(stdout.lines().last().unwrap(), "2748");
    }

    #[test]
    fn detects_hex_with_0x_prefix() {
        let (stdout, _, success) = run_changebase(&["--od", "0xff"]);
        assert!(success);
        assert_eq!(stdout.lines().last().unwrap(), "255");
    }

    #[test]
    fn ambiguous_binary_detected_as_binary() {
        // "101" is valid for all bases but detected as binary
        let (stdout, _, success) = run_changebase(&["--od", "101"]);
        assert!(success);
        // Binary 101 = decimal 5
        assert_eq!(stdout.lines().last().unwrap(), "5");
    }

    #[test]
    fn ambiguous_octal_detected_as_octal() {
        // "777" is valid for oct/dec/hex but detected as octal (after binary check fails)
        let (stdout, _, success) = run_changebase(&["--od", "777"]);
        assert!(success);
        // Octal 777 = decimal 511
        assert_eq!(stdout.lines().last().unwrap(), "511");
    }
}

// ==================== Large number tests ====================

mod large_numbers {
    use super::*;

    #[test]
    fn handles_u64_max() {
        let (stdout, _, success) = run_changebase(&["--id", "--oh", "18446744073709551615"]);
        assert!(success);
        assert_eq!(stdout, "ffffffffffffffff");
    }

    #[test]
    fn handles_larger_than_u64() {
        let (stdout, _, success) = run_changebase(&["--id", "--oh", "18446744073709551616"]);
        assert!(success);
        assert_eq!(stdout, "10000000000000000");
    }

    #[test]
    fn handles_256_bit_number() {
        // 2^256 - 1 in hex
        let hex = "f".repeat(64);
        let (stdout, _, success) = run_changebase(&["--ih", "--ob", &hex]);
        assert!(success);
        // Should be 256 ones
        assert_eq!(stdout.len(), 256);
        assert!(stdout.chars().all(|c| c == '1'));
    }
}

// ==================== Error handling tests ====================

mod error_handling {
    use super::*;

    #[test]
    fn rejects_invalid_binary_digit() {
        let (_, stderr, success) = run_changebase(&["--ib", "--od", "102"]);
        assert!(!success);
        assert!(stderr.contains("Error") || stderr.contains("error"));
    }

    #[test]
    fn rejects_invalid_octal_digit() {
        let (_, stderr, success) = run_changebase(&["--io", "--od", "78"]);
        assert!(!success);
        assert!(stderr.contains("Error") || stderr.contains("error"));
    }

    #[test]
    fn rejects_invalid_decimal_digit() {
        let (_, stderr, success) = run_changebase(&["--id", "--oh", "12a"]);
        assert!(!success);
        assert!(stderr.contains("Error") || stderr.contains("error"));
    }

    #[test]
    fn rejects_invalid_hex_digit() {
        let (_, stderr, success) = run_changebase(&["--ih", "--od", "xyz"]);
        assert!(!success);
        assert!(stderr.contains("Error") || stderr.contains("error"));
    }

    #[test]
    fn error_when_no_output_base() {
        let (_, stderr, success) = run_changebase(&["--id", "255"]);
        assert!(!success);
        assert!(stderr.contains("output") || stderr.contains("Invalid"));
    }
}

// ==================== Verbose mode tests ====================

mod verbose_mode {
    use super::*;

    #[test]
    fn verbose_shows_conversion_info() {
        let (stdout, _, success) = run_changebase(&["-v", "--id", "--oh", "255"]);
        assert!(success);
        assert!(stdout.contains("Converting"));
        assert!(stdout.contains("Decimal"));
        assert!(stdout.contains("Hexadecimal"));
        assert!(stdout.contains("ff"));
    }
}

// ==================== Edge cases ====================

mod edge_cases {
    use super::*;

    #[test]
    fn zero_conversion() {
        let (stdout, _, success) = run_changebase(&["--id", "--oh", "0"]);
        assert!(success);
        assert_eq!(stdout, "0");
    }

    #[test]
    fn single_digit_conversion() {
        let (stdout, _, success) = run_changebase(&["--id", "--ob", "1"]);
        assert!(success);
        assert_eq!(stdout, "1");
    }

    #[test]
    fn leading_zeros_stripped() {
        let (stdout, _, success) = run_changebase(&["--id", "--ob", "8"]);
        assert!(success);
        // Should be "1000" not "0001000"
        assert_eq!(stdout, "1000");
    }

    #[test]
    fn same_base_identity() {
        let (stdout, _, success) = run_changebase(&["--id", "--od", "12345"]);
        assert!(success);
        assert_eq!(stdout, "12345");
    }
}

// ==================== Common use cases ====================

mod common_use_cases {
    use super::*;

    #[test]
    fn ip_address_octet() {
        // Common: convert byte values
        let (stdout, _, success) = run_changebase(&["--id", "--oh", "192"]);
        assert!(success);
        assert_eq!(stdout, "c0");
    }

    #[test]
    fn color_code() {
        // Common: RGB color values
        let (stdout, _, success) = run_changebase(&["--ih", "--od", "ff"]);
        assert!(success);
        assert_eq!(stdout, "255");
    }

    #[test]
    fn bit_pattern() {
        // Common: checking bit patterns
        let (stdout, _, success) = run_changebase(&["--id", "--ob", "255"]);
        assert!(success);
        assert_eq!(stdout, "11111111");
    }

    #[test]
    fn permissions_octal() {
        // Common: Unix permissions
        let (stdout, _, success) = run_changebase(&["--io", "--ob", "755"]);
        assert!(success);
        assert_eq!(stdout, "111101101");
    }

    #[test]
    fn memory_address() {
        // Common: memory addresses
        let (stdout, _, success) = run_changebase(&["--ih", "--od", "deadbeef"]);
        assert!(success);
        assert_eq!(stdout, "3735928559");
    }
}
