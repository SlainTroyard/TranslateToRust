use std::io;
use std::io::Write;

fn convert_date_to_binary(date: &str) -> String {
    let mut ans = vec!['\0'; 25];
    let mut ptr = 24;

    let d = date[8..].parse::<i32>().unwrap_or(0);
    if d == 0 {
        ptr -= 1;
        ans[ptr] = '0';
    } else {
        let mut temp_d = d;
        while temp_d > 0 {
            ptr -= 1;
            ans[ptr] = ((temp_d & 1) as u8 + b'0') as char;
            temp_d >>= 1;
        }
    }
    ptr -= 1;
    ans[ptr] = '-';

    let m = date[5..7].parse::<i32>().unwrap_or(0);
    if m == 0 {
        ptr -= 1;
        ans[ptr] = '0';
    } else {
        let mut temp_m = m;
        while temp_m > 0 {
            ptr -= 1;
            ans[ptr] = ((temp_m & 1) as u8 + b'0') as char;
            temp_m >>= 1;
        }
    }
    ptr -= 1;
    ans[ptr] = '-';

    let y = date[0..4].parse::<i32>().unwrap_or(0);
    if y == 0 {
        ptr -= 1;
        ans[ptr] = '0';
    } else {
        let mut temp_y = y;
        while temp_y > 0 {
            ptr -= 1;
            ans[ptr] = ((temp_y & 1) as u8 + b'0') as char;
            temp_y >>= 1;
        }
    }

    let len = 25 - ptr;
    ans[ptr..25].iter().collect::<String>()
}

fn main() -> io::Result<()> {
    let mut date = String::new();
    io::stdin().read_line(&mut date)?;
    let date = date.trim();

    let ans = convert_date_to_binary(date);
    println!("{}", ans);

    Ok(())
}