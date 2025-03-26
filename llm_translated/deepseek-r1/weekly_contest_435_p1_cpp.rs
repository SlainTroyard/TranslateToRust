use std::io;

fn max_difference(s: &str) -> i32 {
    let mut cnt = [0i32; 26];
    for c in s.chars() {
        let idx = (c as u8 - b'a') as usize;
        cnt[idx] += 1;
    }

    let mut max1 = 0;
    let mut min0 = i32::MAX;

    for &c in &cnt {
        if c % 2 == 1 {
            max1 = max1.max(c);
        } else if c != 0 {
            min0 = min0.min(c);
        }
    }

    max1 - min0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let s = input.split_whitespace().next().expect("No input provided");
    
    println!("{}", max_difference(s));
}