use std::collections::BTreeSet;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn max_distinct_elements(arr: &mut Vec<i32>, diff: i32) -> i32 {
        let mut prev = i32::MIN;
        let mut c = BTreeSet::new();
        arr.sort_unstable();

        for &num in arr {
            let x = prev.max(num - diff);

            if x <= num + diff {
                c.insert(x);
                prev = x;
            }
        }

        c.len() as i32
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Input array size and difference
    let n: usize = lines.next().unwrap()?.trim().parse().unwrap();
    let diff: i32 = lines.next().unwrap()?.trim().parse().unwrap();

    // Input array elements
    let mut arr: Vec<i32> = lines
        .take(n)
        .map(|line| line.unwrap().trim().parse().unwrap())
        .collect();

    // Compute the result
    let result = Solution::max_distinct_elements(&mut arr, diff);

    // Output the result
    println!("{}", result);

    Ok(())
}