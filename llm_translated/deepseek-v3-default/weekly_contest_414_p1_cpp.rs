use std::io;
use std::str::FromStr;

fn main() {
    // Read the input date from stdin
    let mut date = String::new();
    io::stdin().read_line(&mut date).expect("Failed to read line");
    
    // Remove any trailing newline characters
    let date = date.trim();
    
    // Convert the date to binary format
    let result = convert_date_to_binary(date);
    
    // Print the result to stdout
    println!("{}", result);
}

fn convert_date_to_binary(date: &str) -> String {
    // Helper function to convert an integer to binary string without leading zeros
    let bin = |x: i32| -> String {
        format!("{:b}", x)
    };
    
    // Extract year, month, and day from the date string
    let year = i32::from_str(&date[0..4]).unwrap();
    let month = i32::from_str(&date[5..7]).unwrap();
    let day = i32::from_str(&date[8..10]).unwrap();
    
    // Convert each part to binary and concatenate with hyphens
    format!("{}-{}-{}", bin(year), bin(month), bin(day))
}