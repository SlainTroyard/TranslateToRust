use std::io::{self, BufRead};

const MOD: i64 = 1_000_000_007;

fn length_after_transformations(s: &str, t: i32) -> i32 {
    let mut cnt = [0i64; 26];
    // Initialize counts for each character in the input string
    for c in s.chars() {
        let idx = (c as u8 - b'a') as usize;
        cnt[idx] += 1;
    }
    // Apply each transformation step
    for _ in 0..t {
        let mut nxt = [0i32; 26];
        // Shift characters j -> j+1 for 0 <= j < 25
        for j in 0..25 {
            nxt[j + 1] = cnt[j] as i32;
        }
        // Handle 'z' transformation to 'a' and 'b'
        let add = cnt[25];
        nxt[0] = ((nxt[0] as i64 + add) % MOD) as i32;
        nxt[1] = ((nxt[1] as i64 + add) % MOD) as i32;
        // Update counts for the next iteration
        for (j, &val) in nxt.iter().enumerate() {
            cnt[j] = val as i64;
        }
    }
    // Calculate total count modulo MOD
    let total: i64 = cnt.iter().sum();
    (total % MOD) as i32
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.lock().read_line(&mut input).expect("Failed to read line");
    let mut parts = input.trim().split_whitespace();
    let s = parts.next().unwrap();
    let t: i32 = parts.next().unwrap().parse().unwrap();
    let result = length_after_transformations(s, t);
    println!("{}", result);
}