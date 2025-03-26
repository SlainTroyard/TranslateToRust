// Problem: Weekly Contest 414 Problem 1
use std::io::{self, Read};

fn convert_date_to_binary(date: &str) -> String {
    // Allocate a buffer of 25 bytes, initialized to 0
    let mut buffer = vec![0u8; 25];
    let mut ptr = 24; // Start from the end of the buffer

    // Parse and convert day to binary
    let d = date[8..].parse::<i32>().unwrap_or(0);
    if d == 0 {
        ptr -= 1;
        buffer[ptr] = b'0';
    } else {
        let mut day = d;
        while day > 0 {
            ptr -= 1;
            buffer[ptr] = (day & 1) as u8 + b'0';
            day >>= 1;
        }
    }
    
    // Add dash separator
    ptr -= 1;
    buffer[ptr] = b'-';

    // Parse and convert month to binary
    let m = date[5..7].parse::<i32>().unwrap_or(0);
    if m == 0 {
        ptr -= 1;
        buffer[ptr] = b'0';
    } else {
        let mut month = m;
        while month > 0 {
            ptr -= 1;
            buffer[ptr] = (month & 1) as u8 + b'0';
            month >>= 1;
        }
    }
    
    // Add dash separator
    ptr -= 1;
    buffer[ptr] = b'-';

    // Parse and convert year to binary
    let y = date[0..4].parse::<i32>().unwrap_or(0);
    if y == 0 {
        ptr -= 1;
        buffer[ptr] = b'0';
    } else {
        let mut year = y;
        while year > 0 {
            ptr -= 1;
            buffer[ptr] = (year & 1) as u8 + b'0';
            year >>= 1;
        }
    }

    // Move the result to the beginning of the string
    let len = 25 - ptr;
    let result = &buffer[ptr..ptr+len];
    
    // Convert to String with proper UTF-8 handling
    String::from_utf8(result.to_vec()).unwrap()
}

fn main() {
    // Read input from stdin
    let mut date = String::new();
    io::stdin().read_line(&mut date).expect("Failed to read line");
    let date = date.trim();
    
    // Convert the date and print the result
    let ans = convert_date_to_binary(date);
    println!("{}", ans);
}