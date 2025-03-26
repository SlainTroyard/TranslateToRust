use std::io;

// Converts a date string in "YYYY-MM-DD" format to a binary string representation.
fn convert_date_to_binary(date: &str) -> String {
    // Split the date into year, month, and day components
    let parts: Vec<&str> = date.split('-').collect();
    let year = parts[0].parse::<u32>().unwrap();
    let month = parts[1].parse::<u32>().unwrap();
    let day = parts[2].parse::<u32>().unwrap();

    // Helper function to convert integer to binary string without leading zeros
    fn to_binary(n: u32) -> String {
        if n == 0 {
            return "0".to_string();
        }
        let mut bits = Vec::new();
        let mut num = n;
        while num > 0 {
            bits.push(if num & 1 == 1 { '1' } else { '0' });
            num >>= 1;
        }
        bits.iter().rev().collect::<String>()
    }

    // Format the binary components with hyphens
    format!("{}-{}-{}", to_binary(year), to_binary(month), to_binary(day))
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let date = input.trim();
    let result = convert_date_to_binary(date);
    println!("{}", result);
}