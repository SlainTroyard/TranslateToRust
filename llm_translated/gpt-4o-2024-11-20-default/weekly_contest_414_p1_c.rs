use std::io::{self, Write};

fn convert_date_to_binary(date: &str) -> String {
    let mut ans = String::with_capacity(25);

    // Process the day (2 characters starting from the 9th position)
    let d: i32 = date[8..10].parse().unwrap_or(0);
    if d == 0 {
        ans.insert(0, '0');
    } else {
        let mut ptr = String::new();
        let mut d = d;
        while d != 0 {
            ptr.insert(0, char::from((d & 1) as u8 + b'0'));
            d >>= 1;
        }
        ans.insert_str(0, &ptr);
    }
    ans.insert(0, '-');

    // Process the month (2 characters starting from the 6th position)
    let m: i32 = date[5..7].parse().unwrap_or(0);
    if m == 0 {
        ans.insert(0, '0');
    } else {
        let mut ptr = String::new();
        let mut m = m;
        while m != 0 {
            ptr.insert(0, char::from((m & 1) as u8 + b'0'));
            m >>= 1;
        }
        ans.insert_str(0, &ptr);
    }
    ans.insert(0, '-');

    // Process the year (4 characters starting from the 0th position)
    let y: i32 = date[0..4].parse().unwrap_or(0);
    if y == 0 {
        ans.insert(0, '0');
    } else {
        let mut ptr = String::new();
        let mut y = y;
        while y != 0 {
            ptr.insert(0, char::from((y & 1) as u8 + b'0'));
            y >>= 1;
        }
        ans.insert_str(0, &ptr);
    }

    ans
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let date = input.trim(); // Remove newline characters from the input

    let ans = convert_date_to_binary(date);
    println!("{}", ans);
}