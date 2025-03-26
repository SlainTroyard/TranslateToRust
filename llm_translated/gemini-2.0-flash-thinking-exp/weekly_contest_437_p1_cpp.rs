use std::io;

fn has_special_substring(s: String, k: i32) -> bool {
    let n = s.len();
    if k <= 0 {
        return false; // k should be positive
    }
    if n == 0 {
        return false; // empty string cannot have substring of length k > 0
    }

    let chars: Vec<char> = s.chars().collect();
    let mut cnt = 0;
    for i in 0..n {
        cnt += 1;
        if i == n - 1 || chars[i] != chars[i + 1] {
            if cnt == k {
                return true;
            }
            cnt = 0;
        }
    }
    false
}

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    let s = s.trim();

    let mut k_str = String::new();
    io::stdin().read_line(&mut k_str).expect("Failed to read line");
    let k: i32 = k_str.trim().parse().expect("Failed to parse k as integer");

    let result = has_special_substring(s.to_string(), k);
    if result {
        println!("1");
    } else {
        println!("0");
    }
}