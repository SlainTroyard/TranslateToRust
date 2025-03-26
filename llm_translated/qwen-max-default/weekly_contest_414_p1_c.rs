use std::io::{self, Write};

fn convert_date_to_binary(date: &str) -> String {
    let mut ans = vec!['0'; 25];
    let mut ptr = 24;

    // Parse and convert the day
    let d = date[8..10].parse::<i32>().unwrap_or(0);
    if d == 0 {
        ans[ptr] = '0';
        ptr -= 1;
    } else {
        while d > 0 {
            ans[ptr] = (d & 1) as u8 as char + '0';
            ptr -= 1;
            d >>= 1;
        }
    }
    ans[ptr] = '-';
    ptr -= 1;

    // Parse and convert the month
    let m = date[5..7].parse::<i32>().unwrap_or(0);
    if m == 0 {
        ans[ptr] = '0';
        ptr -= 1;
    } else {
        while m > 0 {
            ans[ptr] = (m & 1) as u8 as char + '0';
            ptr -= 1;
            m >>= 1;
        }
    }
    ans[ptr] = '-';
    ptr -= 1;

    // Parse and convert the year
    let y = date[0..4].parse::<i32>().unwrap_or(0);
    if y == 0 {
        ans[ptr] = '0';
        ptr -= 1;
    } else {
        while y > 0 {
            ans[ptr] = (y & 1) as u8 as char + '0';
            ptr -= 1;
            y >>= 1;
        }
    }

    // Trim the leading zeros and create the final string
    let len = 25 - ptr - 1;
    ans[ptr..ptr + len].iter().collect()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let date = input.trim();

    let binary_date = convert_date_to_binary(date);
    println!("{}", binary_date);
}