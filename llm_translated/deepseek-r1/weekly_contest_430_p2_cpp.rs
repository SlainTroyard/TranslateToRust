use std::io::{self, Read};

fn answer_string(s: &str, k: i32) -> String {
    let k = k as usize;
    let s_bytes = s.as_bytes();
    let n = s_bytes.len();

    if k == 1 {
        return s.to_string();
    }

    let mut max_slice = &s_bytes[..0]; // Start with empty slice

    for i in 0..n {
        let max_val = std::cmp::max(k - 1, i);
        let len = n.saturating_sub(max_val);
        let end = i + len;
        let current = &s_bytes[i..end];
        if current > max_slice {
            max_slice = current;
        }
    }

    String::from_utf8(max_slice.to_vec()).expect("Invalid UTF-8")
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut parts = input.split_whitespace();
    let s = parts.next().expect("No string provided");
    let k: i32 = parts.next().expect("No k provided").parse().expect("Invalid k");

    let result = answer_string(s, k);
    println!("{}", result);
}