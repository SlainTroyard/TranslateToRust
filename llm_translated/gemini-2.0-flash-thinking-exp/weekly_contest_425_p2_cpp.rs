use std::io;
use std::collections::HashMap;

struct Solution {}

impl Solution {
    fn is_possible_to_rearrange(&self, s: String, t: String, k: i32) -> bool {
        let n = s.len();
        if k <= 0 || n as i32 % k != 0 { // Handle invalid k or non-divisible n by k (though problem likely assumes valid input)
            return false; // Or handle as per problem constraints if specified
        }
        let size = n / k as usize;
        let mut mp: HashMap<String, i32> = HashMap::new();

        // Count substrings in s
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
        for value in mp.values() {
            if *value != 0 {
                return false;
            }
        }

        true
    }
}

fn main() {
    let solution = Solution {};

    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    let s = s.trim();

    let mut t = String::new();
    io::stdin().read_line(&mut t).expect("Failed to read line");
    let t = t.trim();

    let mut k_str = String::new();
    io::stdin().read_line(&mut k_str).expect("Failed to read line");
    let k: i32 = k_str.trim().parse().expect("Failed to parse k as i32");

    let result = solution.is_possible_to_rearrange(s.to_string(), t.to_string(), k);

    if result {
        println!("true");
    } else {
        println!("false");
    }
}