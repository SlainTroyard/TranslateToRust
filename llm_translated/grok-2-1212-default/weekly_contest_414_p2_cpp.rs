use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn max_possible_score(start: &mut Vec<i32>, d: i32) -> i32 {
        start.sort_unstable();

        let check = |score: i32| -> bool {
            let mut x = i64::MIN;
            for &s in start.iter() {
                x = x.max(x + score as i64);
                x = x.max(s as i64);
                if x > s as i64 + d as i64 {
                    return false;
                }
            }
            true
        };

        let left = 0;
        let right = (start.last().unwrap() + d - start[0]) / (start.len() as i32 - 1) + 1;
        let mut left = left;
        let mut right = right;

        while left + 1 < right {
            let mid = left + (right - left) / 2;
            if check(mid) {
                left = mid;
            } else {
                right = mid;
            }
        }
        left
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read startSize and d
    let first_line = lines.next().unwrap()?;
    let mut iter = first_line.split_whitespace();
    let start_size: usize = iter.next().unwrap().parse().unwrap();
    let d: i32 = iter.next().unwrap().parse().unwrap();

    // Read start array
    let second_line = lines.next().unwrap()?;
    let start: Vec<i32> = second_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Calculate and print the result
    let result = Solution::max_possible_score(&mut start.clone(), d);
    println!("{}", result);

    Ok(())
}