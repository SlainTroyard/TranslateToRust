use std::io::{self, Write};

// Function to convert a date string to a binary representation
fn convert_date_to_binary(date: &str) -> String {
    let mut ans = vec!['0'; 25]; // Initialize a vector with 25 '0's
    let mut ptr = 24; // Pointer to the end of the vector

    // Extract day, month, and year from the date string
    let day = date[8..10].parse::<i32>().unwrap();
    let month = date[5..7].parse::<i32>().unwrap();
    let year = date[0..4].parse::<i32>().unwrap();

    // Convert day to binary and store in the vector
    if day == 0 {
        ans[ptr] = '0';
        ptr -= 1;
    } else {
        let mut d = day;
        while d != 0 {
            ans[ptr] = ((d & 1) as u8 + b'0') as char;
            ptr -= 1;
            d >>= 1;
        }
    }
    ans[ptr] = '-';
    ptr -= 1;

    // Convert month to binary and store in the vector
    if month == 0 {
        ans[ptr] = '0';
        ptr -= 1;
    } else {
        let mut m = month;
        while m != 0 {
            ans[ptr] = ((m & 1) as u8 + b'0') as char;
            ptr -= 1;
            m >>= 1;
        }
    }
    ans[ptr] = '-';
    ptr -= 1;

    // Convert year to binary and store in the vector
    if year == 0 {
        ans[ptr] = '0';
        ptr -= 1;
    } else {
        let mut y = year;
        while y != 0 {
            ans[ptr] = ((y & 1) as u8 + b'0') as char;
            ptr -= 1;
            y >>= 1;
        }
    }

    // Calculate the length of the result and move it to the beginning of the vector
    let len = 25 - ptr - 1;
    let result: String = ans[ptr + 1..].iter().collect();
    result
}

fn main() {
    // Read the date string from stdin
    let mut date = String::new();
    io::stdin().read_line(&mut date).expect("Failed to read line");
    let date = date.trim();

    // Convert the date to binary and print the result
    let ans = convert_date_to_binary(date);
    println!("{}", ans);
}