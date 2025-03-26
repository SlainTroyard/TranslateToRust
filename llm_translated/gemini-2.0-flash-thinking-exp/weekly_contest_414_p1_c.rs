use std::io;
use std::process;

fn convert_date_to_binary(date: &str) -> String {
    let parts: Vec<&str> = date.split('-').collect();
    if parts.len() != 3 {
        eprintln!("Invalid date format: {}", date);
        process::exit(1); // Exit with an error code
    }

    let year_str = parts[0];
    let month_str = parts[1];
    let day_str = parts[2];

    let year = year_str.parse::<i32>().unwrap();
    let month = month_str.parse::<i32>().unwrap();
    let day = day_str.parse::<i32>().unwrap();

    let year_bin = int_to_binary_string(year);
    let month_bin = int_to_binary_string(month);
    let day_bin = int_to_binary_string(day);

    format!("{}-{}-{}", year_bin, month_bin, day_bin)
}

fn int_to_binary_string(mut n: i32) -> String {
    if n == 0 {
        return "0".to_string();
    }
    let mut binary_string = String::new();
    while n > 0 {
        binary_string.insert(0, if n % 2 == 0 { '0' } else { '1' });
        n /= 2;
    }
    binary_string
}

fn main() {
    let mut date = String::new();
    io::stdin().read_line(&mut date).expect("Failed to read line");
    let date = date.trim(); // Remove trailing newline

    let ans = convert_date_to_binary(date);
    println!("{}", ans);
}