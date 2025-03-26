use std::io::{self, BufRead};

fn convert_date_to_binary(date: &str) -> String {
    let mut ans = vec!['0'; 25];
    let mut ptr = 24;

    // Extract day
    let d: i32 = date[8..].parse().unwrap();
    if d == 0 {
        ptr -= 1;
        ans[ptr] = '0';
    } else {
        let mut d = d;
        while d > 0 {
            ptr -= 1;
            ans[ptr] = ((d & 1) as u8 + b'0') as char;
            d >>= 1;
        }
    }
    ptr -= 1;
    ans[ptr] = '-';

    // Extract month
    let m: i32 = date[5..8].parse().unwrap();
    if m == 0 {
        ptr -= 1;
        ans[ptr] = '0';
    } else {
        let mut m = m;
        while m > 0 {
            ptr -= 1;
            ans[ptr] = ((m & 1) as u8 + b'0') as char;
            m >>= 1;
        }
    }
    ptr -= 1;
    ans[ptr] = '-';

    // Extract year
    let y: i32 = date[..4].parse().unwrap();
    if y == 0 {
        ptr -= 1;
        ans[ptr] = '0';
    } else {
        let mut y = y;
        while y > 0 {
            ptr -= 1;
            ans[ptr] = ((y & 1) as u8 + b'0') as char;
            y >>= 1;
        }
    }

    // Convert to String and return
    ans[ptr..].iter().collect()
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let date = line?;
        let ans = convert_date_to_binary(&date);
        println!("{}", ans);
    }
    Ok(())
}