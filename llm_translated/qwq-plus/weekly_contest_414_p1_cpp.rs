use std::io;

/// Converts a date string in "YYYY-MM-DD" format to a binary string with each component converted to binary.
///
/// # Arguments
///
/// * `date` - A string slice representing the date in "YYYY-MM-DD" format.
///
/// # Returns
///
/// A `String` representing the binary conversion of the date in the format "YYYY_bin-MM_bin-DD_bin".
///
/// # Panics
///
/// Panics if the input date is not in the correct format or contains invalid numeric components.
fn convert_date_to_binary(date: &str) -> String {
    // Extract year, month, and day components from the date string
    let year_str = &date[0..4];
    let month_str = &date[5..7];
    let day_str = &date[8..10];

    // Parse each component into integers, using expect for error handling
    let year = year_str.parse::<i32>().expect("Invalid year component");
    let month = month_str.parse::<i32>().expect("Invalid month component");
    let day = day_str.parse::<i32>().expect("Invalid day component");

    // Closure to convert an integer to its binary string representation without leading zeros
    let to_bin = |x: i32| format!("{:b}", x);

    // Combine the binary strings with hyphens
    format!("{}-{}-{}", to_bin(year), to_bin(month), to_bin(day))
}

fn main() {
    // Read the input date from standard input
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input from stdin");

    // Trim whitespace and process the input
    let trimmed_input = input.trim();
    // Convert and print the result
    println!("{}", convert_date_to_binary(trimmed_input));
}