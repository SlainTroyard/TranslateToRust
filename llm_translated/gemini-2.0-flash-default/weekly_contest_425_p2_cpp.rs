use std::collections::HashMap;
use std::io;

struct Solution {}

impl Solution {
    fn is_possible_to_rearrange(s: String, t: String, k: i32) -> bool {
        let n = s.len();
        let mut mp: HashMap<String, i32> = HashMap::new();
        let size = n / k as usize;

        // Count occurrences of substrings in s
        for i in (0..n).step_by(size) {
            let sub = s[i..i + size].to_string();
            *mp.entry(sub).or_insert(0) += 1;
        }

        // Decrement counts for substrings in t
        for i in (0..n).step_by(size) {
            let sub = t[i..i + size].to_string();
            *mp.entry(sub).or_insert(0) -= 1;
        }

        // Check if all counts are zero
        for (_key, value) in &mp {
            if *value != 0 {
                return false;
            }
        }

        true
    }
}

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    let s = s.trim().to_string();

    let mut t = String::new();
    io::stdin().read_line(&mut t).expect("Failed to read line");
    let t = t.trim().to_string();

    let mut k_str = String::new();
    io::stdin().read_line(&mut k_str).expect("Failed to read line");
    let k: i32 = k_str.trim().parse().expect("Failed to parse integer");

    let solution = Solution {};
    let result = solution.is_possible_to_rearrange(s, t, k);

    if result {
        println!("true");
    } else {
        println!("false");
    }
}