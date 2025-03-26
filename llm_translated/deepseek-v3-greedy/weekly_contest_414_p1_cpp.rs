use std::io;
use std::str::FromStr;

// Function to convert an integer to a binary string without leading zeros
fn to_binary(x: i32) -> String {
    format!("{:b}", x)
}

// Function to convert a date string to a binary formatted string
fn convert_date_to_binary(date: &str) -> String {
    let year = i32::from_str(&date[0..4]).unwrap();
    let month = i32::from_str(&date[5..7]).unwrap();
    let day = i32::from_str(&date[8..10]).unwrap();

    format!("{}-{}-{}", to_binary(year), to_binary(month), to_binary(day))
}

fn main() {
    // Read the input date from stdin
    let mut date = String::new();
    io::stdin().read_line(&mut date).expect("Failed to read line");
    let date = date.trim(); // Remove any trailing newline

    // Convert the date to binary format and print it
    let result = convert_date_to_binary(date);
    println!("{}", result);
}