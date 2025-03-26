use std::io;
use std::collections::HashSet;

struct Solution;

impl Solution {
    fn max_distinct_elements(&self, arr: Vec<i32>, diff: i32) -> i32 {
        let mut arr = arr;
        arr.sort();
        let mut prev = i32::MIN;
        let mut c = HashSet::new();
        for num in arr {
            let x = std::cmp::max(prev + 1, num - diff);
            if x <= num + diff {
                c.insert(x);
                prev = x;
            }
        }
        c.len() as i32
    }
}

fn main() -> io::Result<()> {
    let solution = Solution;

    // Read n
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let n: i32 = input.trim().parse().expect("Invalid integer for n");

    // Read diff
    input.clear();
    io::stdin().read_line(&mut input)?;
    let diff: i32 = input.trim().parse().expect("Invalid integer for diff");

    // Read array
    input.clear();
    io::stdin().read_line(&mut input)?;
    let arr: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid array element"))
        .collect();

    // Compute result
    let result = solution.max_distinct_elements(arr, diff);

    // Output result
    println!("{}", result);

    Ok(())
}