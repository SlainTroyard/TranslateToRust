use std::io::{self, BufRead, Write};

/// Convert an integer to its binary string representation without leading zeros.
/// If the input x is 0, return "0".
fn to_binary(x: i32) -> String {
    if x == 0 {
        "0".to_string()
    } else {
        format!("{:b}", x)
    }
}

/// Convert a date string in the format "YYYY-MM-DD" to a binary representation
/// where each component (year, month, day) is converted to its binary form
/// (without leading zeros) and joined by '-'.
/// For example, "2019-11-26" -> "11111100011-1011-11010".
fn convert_date_to_binary(date: &str) -> Result<String, String> {
    // Validate that the date string has at least the expected length.
    if date.len() < 10 {
        return Err("Invalid date format".to_string());
    }
    
    // Extract and convert the year, month, and day substrings.
    let year_str = &date[0..4];
    let month_str = &date[5..7];
    let day_str = &date[8..10];

    // Parse the substrings as integers.
    let year = year_str.parse::<i32>().map_err(|e| format!("Failed to parse year: {}", e))?;
    let month = month_str.parse::<i32>().map_err(|e| format!("Failed to parse month: {}", e))?;
    let day = day_str.parse::<i32>().map_err(|e| format!("Failed to parse day: {}", e))?;

    // Convert integers to binary strings.
    let year_bin = to_binary(year);
    let month_bin = to_binary(month);
    let day_bin = to_binary(day);

    // Concatenate with '-' separator.
    Ok(format!("{}-{}-{}", year_bin, month_bin, day_bin))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up standard input and output.
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    // Read the full input from stdin. The original C++ code uses 'cin >> date'
    // which extracts one token based on whitespace. We mimic that behavior.
    let mut input = String::new();
    stdin.lock().read_line(&mut input)?;
    let date = input.trim(); // Remove any extra whitespace/newlines

    // Get the converted date or print an error message if it fails.
    match convert_date_to_binary(date) {
        Ok(result) => writeln!(stdout, "{}", result)?,
        Err(e) => writeln!(stdout, "Error: {}", e)?,
    }

    Ok(())
}