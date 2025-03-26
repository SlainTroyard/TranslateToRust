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

    let first_line = lines.next().unwrap()?;
    let mut values = first_line.split_whitespace();
    let start_size: usize = values.next().unwrap().parse().unwrap();
    let d: i32 = values.next().unwrap().parse().unwrap();

    let mut start = Vec::with_capacity(start_size);
    for _ in 0..start_size {
        let line = lines.next().unwrap()?;
        start.push(line.trim().parse().unwrap());
    }

    let sol = Solution;
    let result = sol.max_possible_score(&mut start, d);
    println!("{}", result);

    Ok(())
}