use std::io::{self, Read};

fn max_difference(s: &str) -> i32 {
    let mut cnt = [0; 26];
    let mut c1 = 0;
    let mut c2 = 100;

    for c in s.chars() {
        cnt[(c as u8 - b'a') as usize] += 1;
    }

    for i in 0..26 {
        if cnt[i] & 1 != 0 {
            c1 = cnt[i].max(c1);
        } else if cnt[i] != 0 {
            c2 = cnt[i].min(c2);
        }
    }

    c1 as i32 - c2 as i32
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Process each line of input
    for line in input.lines() {
        let result = max_difference(line);
        println!("{}", result);
    }

    Ok(())
}