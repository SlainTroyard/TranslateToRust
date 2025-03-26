use std::collections::HashMap;
use std::io;

struct Solution;

impl Solution {
    pub fn is_possible_to_rearrange(s: String, t: String, k: i32) -> bool {
        let n = s.len();
        let size = n / k as usize;
        let mut map: HashMap<Vec<u8>, i32> = HashMap::new();

        // Process string s and add counts
        let s_bytes = s.as_bytes();
        for i in (0..n).step_by(size) {
            let end = i + size;
            let chunk = &s_bytes[i..end.min(n)];
            let key: Vec<u8> = chunk.to_vec();
            *map.entry(key).or_insert(0) += 1;
        }

        // Process string t and subtract counts
        let t_bytes = t.as_bytes();
        for i in (0..n).step_by(size) {
            let end = i + size;
            let chunk = &t_bytes[i..end.min(n)];
            let key: Vec<u8> = chunk.to_vec();
            *map.entry(key).or_insert(0) -= 1;
        }

        // Check if all counts are zero
        for (&_, count) in &map {
            if *count != 0 {
                return false;
            }
        }

        true
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let tokens: Vec<&str> = input.split_whitespace().collect();
    if tokens.len() < 3 {
        eprintln!("Invalid input");
        return;
    }
    let s = tokens[0].to_string();
    let t = tokens[1].to_string();
    let k = tokens[2].parse().expect("Invalid k");
    let solution = Solution;
    let result = solution.is_possible_to_rearrange(s, t, k);
    println!("{}", if result { "true" } else { "false" });
}