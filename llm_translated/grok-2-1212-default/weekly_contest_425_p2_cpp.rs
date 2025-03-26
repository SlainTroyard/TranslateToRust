use std::collections::HashMap;
use std::io::{self, Read};

struct Solution;

impl Solution {
    fn is_possible_to_rearrange(s: &str, t: &str, k: i32) -> bool {
        let n = s.len();
        let mut mp: HashMap<String, i32> = HashMap::new();
        let size = n / k as usize;

        for i in (0..n).step_by(size) {
            *mp.entry(s[i..i + size].to_string()).or_default() += 1;
        }

        for i in (0..n).step_by(size) {
            *mp.entry(t[i..i + size].to_string()).or_default() -= 1;
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
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let s = lines.next().unwrap().to_string();
    let t = lines.next().unwrap().to_string();
    let k: i32 = lines.next().unwrap().parse().unwrap();

    let solution = Solution;
    let result = solution.is_possible_to_rearrange(&s, &t, k);

    if result {
        println!("true");
    } else {
        println!("false");
    }

    Ok(())
}