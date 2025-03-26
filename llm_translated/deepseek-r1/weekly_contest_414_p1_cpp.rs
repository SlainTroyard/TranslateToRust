use std::io;

fn main() {
    // Read input line and extract the first token, matching C++'s cin >> date
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let date = input.split_whitespace().next().expect("No date provided");
    
    // Process the date and print the result
    let result = convert_date_to_binary(date);
    println!("{}", result);
}

/// Converts a date string in "YYYY-MM-DD" format to binary representation without leading zeros
fn convert_date_to_binary(date: &str) -> String {
    // Extract date components using fixed indices (assuming valid format)
    let year_str = &date[0..4];
    let month_str = &date[5..7];
    let day_str = &date[8..10];

    // Parse components to integers and convert to binary strings
    let year = year_str.parse::<u32>().unwrap();
    let month = month_str.parse::<u32>().unwrap();
    let day = day_str.parse::<u32>().unwrap();

    // Format binary components with hyphens
    format!(
        "{}-{}-{}",
        format!("{:b}", year),
        format!("{:b}", month),
        format!("{:b}", day)
    )
}