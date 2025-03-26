use std::io;

fn convert_date_to_binary(date: String) -> String {
    // Split the date into year, month, and day components
    let parts: Vec<&str> = date.split('-').collect();
    // Convert each part to an unsigned 32-bit integer
    let year = parts[0].parse::<u32>().expect("Invalid year");
    let month = parts[1].parse::<u32>().expect("Invalid month");
    let day = parts[2].parse::<u32>().expect("Invalid day");
    
    // Convert each integer to its binary string representation
    format!(
        "{}-{}-{}",
        format!("{:b}", year),
        format!("{:b}", month),
        format!("{:b}", day)
    )
}

fn main() {
    // Read the input date string
    let mut date = String::new();
    io::stdin()
        .read_line(&mut date)
        .expect("Failed to read input");
    // Trim any trailing newline or whitespace
    let date = date.trim().to_string();
    
    // Convert the date to binary format and print the result
    let result = convert_date_to_binary(date);
    println!("{}", result);
}