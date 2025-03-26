use std::io;

fn max_difference(s: String) -> i32 {
    let mut cnt = [0; 26];
    for c in s.chars() {
        if c >= 'a' && c <= 'z' {
            let idx = (c as u8 - 'a' as u8) as usize;
            cnt[idx] += 1;
        }
    }
    let mut c1 = 0;
    let mut c2 = 100;
    for i in 0..26 {
        let count = cnt[i];
        if count % 2 == 1 {
            if count > c1 {
                c1 = count;
            }
        } else if count != 0 {
            if count < c2 {
                c2 = count;
            }
        }
    }
    c1 - c2
}

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read input");
    let s = s.trim().to_string();
    println!("{}", max_difference(s));
}