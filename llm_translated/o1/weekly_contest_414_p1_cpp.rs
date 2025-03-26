use std::error::Error;
use std::io::{self, BufRead};

// Converts an integer to its 32-bit binary form, then removes leading zeros
// (matching the C++ bitset<32> + s.substr(s.find('1')) logic exactly).
fn bin(x: i32) -> String {
    let s = format!("{:032b}", x);
    match s.find('1') {
        Some(pos) => s[pos..].to_string(),
        // If x == 0, in C++ bitset<32>(0).to_string() is all zeros, leading to no '1' found,
        // which results in an empty substring. We'll replicate that behavior here.
        None => String::new(),
    }
}

// Converts a date string of format "YYYY-MM-DD" into its binary representation.
fn convert_date_to_binary(date: &str) -> Result<String, Box<dyn Error>> {
    // Extract the year, month, and day from the date string
    let year = date[0..4].parse::<i32>()?;
    let month = date[5..7].parse::<i32>()?;
    let day = date[8..10].parse::<i32>()?;

    // Build the binary date string
    let binary_date = format!(
        "{}-{}-{}",
        bin(year),
        bin(month),
        bin(day)
    );

    Ok(binary_date)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read a single token (the date) from standard input
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // In C++, "cin >> date" reads one space-delimited token. We replicate by
    // taking the next line (or token-like input if it ends with newline).
    if let Some(Ok(date)) = lines.next() {
        let solution = convert_date_to_binary(&date)?;
        // Print the result exactly as in C++
        println!("{}", solution);
    }

    Ok(())
}