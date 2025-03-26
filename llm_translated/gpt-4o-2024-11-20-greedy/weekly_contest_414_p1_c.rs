use std::io::{self, Write};

fn convert_date_to_binary(date: &str) -> String {
    let mut ans = vec!['0'; 25]; // Preallocate space for the binary string
    let mut ptr = 24; // Start from the end of the buffer

    // Parse the day (last two characters of the date string)
    let d = date[8..].parse::<i32>().unwrap_or(0);
    if d == 0 {
        ans[ptr] = '0';
        ptr -= 1;
    } else {
        let mut temp = d;
        while temp > 0 {
            ans[ptr] = ((temp & 1) as u8 + b'0') as char;
            temp >>= 1;
            ptr -= 1;
        }
    }
    ans[ptr] = '-';
    ptr -= 1;

    // Parse the month (characters at positions 5 and 6)
    let m = date[5..7].parse::<i32>().unwrap_or(0);
    if m == 0 {
        ans[ptr] = '0';
        ptr -= 1;
    } else {
        let mut temp = m;
        while temp > 0 {
            ans[ptr] = ((temp & 1) as u8 + b'0') as char;
            temp >>= 1;
            ptr -= 1;
        }
    }
    ans[ptr] = '-';
    ptr -= 1;

    // Parse the year (first four characters of the date string)
    let y = date[..4].parse::<i32>().unwrap_or(0);
    if y == 0 {
        ans[ptr] = '0';
        ptr -= 1;
    } else {
        let mut temp = y;
        while temp > 0 {
            ans[ptr] = ((temp & 1) as u8 + b'0') as char;
            temp >>= 1;
            ptr -= 1;
        }
    }

    // Create the final string by slicing the buffer
    let result = ans[ptr + 1..].iter().collect::<String>();
    result
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let date = input.trim(); // Remove any trailing newline or whitespace

    // Ensure the input is exactly 10 characters long (YYYY-MM-DD format)
    if date.len() != 10 {
        eprintln!("Invalid input format. Expected YYYY-MM-DD.");
        return;
    }

    let ans = convert_date_to_binary(date);
    println!("{}", ans);
}