use std::io::{self, Write};

fn convert_date_to_binary(date: &str) -> String {
    let mut ans = vec![' '; 25];
    let mut ptr = 24;

    // Parse the day and convert to binary
    let d = date[8..10].parse::<i32>().unwrap_or(0);
    if d == 0 {
        ans[ptr - 1] = '0';
    } else {
        let mut d = d;
        while d > 0 {
            ans[ptr - 1] = (d & 1) as u8 as char + '0';
            d >>= 1;
            ptr -= 1;
        }
    }
    ans[ptr - 1] = '-';
    ptr -= 1;

    // Parse the month and convert to binary
    let m = date[5..7].parse::<i32>().unwrap_or(0);
    if m == 0 {
        ans[ptr - 1] = '0';
    } else {
        let mut m = m;
        while m > 0 {
            ans[ptr - 1] = (m & 1) as u8 as char + '0';
            m >>= 1;
            ptr -= 1;
        }
    }
    ans[ptr - 1] = '-';
    ptr -= 1;

    // Parse the year and convert to binary
    let y = date[0..4].parse::<i32>().unwrap_or(0);
    if y == 0 {
        ans[ptr - 1] = '0';
    } else {
        let mut y = y;
        while y > 0 {
            ans[ptr - 1] = (y & 1) as u8 as char + '0';
            y >>= 1;
            ptr -= 1;
        }
    }

    // Trim the leading spaces and return the result
    ans[ptr..].iter().collect()
}

fn main() {
    let mut date = String::new();
    println!("Enter a date (YYYY-MM-DD):");
    io::stdin().read_line(&mut date).expect("Failed to read line");
    let date = date.trim(); // Remove any trailing newline or whitespace

    let ans = convert_date_to_binary(date);
    println!("{}", ans);
}