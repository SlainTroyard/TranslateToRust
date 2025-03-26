use std::cmp::max;
use std::io;
use std::str::FromStr;

fn main() {
    let mut start_size = String::new();
    io::stdin()
        .read_line(&mut start_size)
        .expect("Failed to read line");
    let start_size: usize = start_size.trim().parse().expect("Invalid input");

    let mut d = String::new();
    io::stdin()
        .read_line(&mut d)
        .expect("Failed to read line");
    let d: i32 = d.trim().parse().expect("Invalid input");

    let mut start_str = String::new();
    io::stdin()
        .read_line(&mut start_str)
        .expect("Failed to read line");
    let start: Vec<i32> = start_str
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Invalid input"))
        .collect();

    let sol = Solution {};
    println!("{}", sol.max_possible_score(start, d));
}

struct Solution {}

impl Solution {
    fn max_possible_score(&self, mut start: Vec<i32>, d: i32) -> i32 {
        start.sort();

        let check = |score: i32| -> bool {
            let mut x: i64 = i64::min_value();
            for &s in &start {
                x = max(x + score as i64, s as i64);
                if x > (s as i64 + d as i64) {
                    return false;
                }
            }
            true
        };

        let mut left: i32 = 0;
        let mut right: i32 = (start.last().unwrap() + d - start[0]) / (start.len() as i32 - 1) + 1;
        while left + 1 < right {
            let mid: i32 = left + (right - left) / 2;
            if check(mid) {
                left = mid;
            } else {
                right = mid;
            }
        }
        left
    }
}