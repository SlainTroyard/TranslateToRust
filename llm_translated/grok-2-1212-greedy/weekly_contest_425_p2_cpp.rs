use std::collections::HashMap;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn is_possible_to_rearrange(s: &str, t: &str, k: i32) -> bool {
        let n = s.len();
        let mut mp: HashMap<String, i32> = HashMap::new();
        let size = n / k as usize;

        for i in (0..n).step_by(size) {
            *mp.entry(s[i..i + size].to_string()).or_insert(0) += 1;
        }

        for i in (0..n).step_by(size) {
            *mp.entry(t[i..i + size].to_string()).or_insert(0) -= 1;
        }

        for value in mp.values() {
            if *value != 0 {
                return false;
            }
        }

        true
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read input
    let s = lines.next().unwrap()?;
    let t = lines.next().unwrap()?;
    let k: i32 = lines.next().unwrap()?.parse().unwrap();

    // Process input
    let solution = Solution;
    let result = solution.is_possible_to_rearrange(&s, &t, k);

    // Output result
    if result {
        println!("true");
    } else {
        println!("false");
    }

    Ok(())
}