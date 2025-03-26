use std::io::{self, Write};

// Function to convert a date string to a binary representation
fn convert_date_to_binary(date: &str) -> String {
    let mut ans = vec!['0'; 25];
    let mut ptr = 24;

    // Extract day, month, and year from the date string
    let day = date[8..10].parse::<i32>().unwrap();
    let month = date[5..7].parse::<i32>().unwrap();
    let year = date[0..4].parse::<i32>().unwrap();

    // Convert day to binary and store in the result
    if day == 0 {
        ans[ptr] = '0';
        ptr -= 1;
    } else {
        let mut d = day;
        while d != 0 {
            ans[ptr] = if d & 1 == 1 { '1' } else { '0' };
            ptr -= 1;
            d >>= 1;
        }
    }
    ans[ptr] = '-';
    ptr -= 1;

    // Convert month to binary and store in the result
    if month == 0 {
        ans[ptr] = '0';
        ptr -= 1;
    } else {
        let mut m = month;
        while m != 0 {
            ans[ptr] = if m & 1 == 1 { '1' } else { '0' };
            ptr -= 1;
            m >>= 1;
        }
    }
    ans[ptr] = '-';
    ptr -= 1;

    // Convert year to binary and store in the result
    if year == 0 {
        ans[ptr] = '0';
        ptr -= 1;
    } else {
        let mut y = year;
        while y != 0 {
            ans[ptr] = if y & 1 == 1 { '1' } else { '0' };
            ptr -= 1;
            y >>= 1;
        }
    }

    // Calculate the length of the valid binary string
    let len = 25 - (ptr + 1);
    let result: String = ans[ptr + 1..].iter().collect();
    result
}

fn main() {
    // Read the input date string
    let mut date = String::new();
    io::stdin().read_line(&mut date).expect("Failed to read input");
    let date = date.trim();

    // Convert the date to binary representation
    let ans = convert_date_to_binary(date);

    // Print the result
    println!("{}", ans);
}