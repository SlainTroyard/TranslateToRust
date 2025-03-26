use std::io::{self, Read};

fn convert_date_to_binary(date: &str) -> String {
    let mut ans = vec![0u8; 25];
    let mut ptr = 24;

    // Convert day
    let d: u32 = date[8..10].parse().unwrap();
    if d == 0 {
        ans[ptr - 1] = b'0';
        ptr -= 1;
    } else {
        let mut temp = d;
        while temp > 0 {
            ans[ptr - 1] = b'0' + (temp & 1) as u8;
            ptr -= 1;
            temp >>= 1;
        }
    }
    ans[ptr - 1] = b'-';
    ptr -= 1;

    // Convert month
    let m: u32 = date[5..7].parse().unwrap();
    if m == 0 {
        ans[ptr - 1] = b'0';
        ptr -= 1;
    } else {
        let mut temp = m;
        while temp > 0 {
            ans[ptr - 1] = b'0' + (temp & 1) as u8;
            ptr -= 1;
            temp >>= 1;
        }
    }
    ans[ptr - 1] = b'-';
    ptr -= 1;

    // Convert year
    let y: u32 = date[0..4].parse().unwrap();
    if y == 0 {
        ans[ptr - 1] = b'0';
        ptr -= 1;
    } else {
        let mut temp = y;
        while temp > 0 {
            ans[ptr - 1] = b'0' + (temp & 1) as u8;
            ptr -= 1;
            temp >>= 1;
        }
    }

    // Convert to String
    String::from_utf8(ans[ptr..].to_vec()).unwrap()
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    for line in input.lines() {
        let ans = convert_date_to_binary(line);
        println!("{}", ans);
    }

    Ok(())
}