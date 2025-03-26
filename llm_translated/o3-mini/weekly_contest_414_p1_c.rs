use std::error::Error;
use std::io::{self, BufRead, Write};

/// Converts the input date string (in the format "YYYY-MM-DD")
/// into a binary representation in the format "year_in_binary-month_in_binary-day_in_binary".
///
/// This function mimics the exact logic of the original C code:
/// - It uses atoi on the string at offsets 0, 5, and 8 to convert year, month, and day respectively.
///   Note: atoi stops parsing at the first nondigit so this works with the hyphens in the input.
/// - If a parsed number is 0, the binary representation is "0"; otherwise, the binary
///   representation is produced with no leading zeros.
/// - The final output order is: binary(year) + "-" + binary(month) + "-" + binary(day)
fn convert_date_to_binary(date: &str) -> String {
    // Helper closure to mimic the C conversion from string to integer.
    // It scans the string from the beginning picking up digits.
    fn atoi(s: &str) -> i32 {
        let mut num = 0;
        for c in s.chars() {
            if let Some(digit) = c.to_digit(10) {
                num = num * 10 + digit as i32;
            } else {
                break; // stop at first non-digit
            }
        }
        num
    }

    // Mimic C pointer arithmetic for conversion.
    // The C code does:
    //   d = atoi(date + 8)
    //   m = atoi(date + 5)
    //   y = atoi(date)
    //
    // We extract substrings starting at these positions. Note: 
    // We don't assume that the string length is exactly 10; we simply
    // use the offsets as in the original code.
    let d = if date.len() > 8 { atoi(&date[8..]) } else { 0 };
    let m = if date.len() > 5 { atoi(&date[5..]) } else { 0 };
    let y = atoi(date);

    // Convert an integer to a binary string (without leading zeros).
    // If the integer is zero, return "0".
    fn to_binary(n: i32) -> String {
        if n == 0 {
            "0".to_string()
        } else {
            // The built-in formatting produces no leading zeros.
            format!("{:b}", n)
        }
    }

    // According to the original logic, the C code appends parts in reverse order,
    // but then memmoves the final result so that the output is:
    // binary(year) + '-' + binary(month) + '-' + binary(day)
    let binary_year = to_binary(y);
    let binary_month = to_binary(m);
    let binary_day = to_binary(d);

    // Build the final result string with the same separators.
    format!("{}-{}-{}", binary_year, binary_month, binary_day)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Use a buffered reader for stdin
    let stdin = io::stdin();
    let mut lock = stdin.lock();

    // Read input token by token.
    // The original C code uses scanf("%s", date) which reads a single whitespace-delimited token.
    // We use split_whitespace() to mimic this exact behavior.
    let mut input = String::new();
    lock.read_line(&mut input)?;
    // trim to remove any trailing newline (scanf stops at whitespace anyway)
    let token = input.trim().split_whitespace().next().unwrap_or("");

    // Compute the conversion
    let result = convert_date_to_binary(token);

    // Print the result to stdout with a newline, exactly as the original C code does.
    writeln!(io::stdout(), "{}", result)?;

    Ok(())
}