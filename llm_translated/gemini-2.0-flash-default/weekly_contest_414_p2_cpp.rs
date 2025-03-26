use std::cmp::max;
use std::io;
use std::io::prelude::*;

struct Solution {}

impl Solution {
    pub fn max_possible_score(&self, start: &mut Vec<i32>, d: i32) -> i32 {
        start.sort();

        let check = |score: i32| -> bool {
            let mut x: i64 = i64::min_value();
            for &s in start.iter() {
                x = max(x + score as i64, s as i64);
                if x > s as i64 + d as i64 {
                    return false;
                }
            }
            true
        };

        let mut left = 0;
        let mut right = (start.last().unwrap() + d - start[0]) / (start.len() as i32 - 1) + 1;
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

fn main() {
    let stdin = io::stdin();
    let mut iter = stdin.lock().lines();

    let first_line = iter.next().unwrap().unwrap();
    let mut split = first_line.split_whitespace();
    let start_size: usize = split.next().unwrap().parse().unwrap();
    let d: i32 = split.next().unwrap().parse().unwrap();

    let second_line = iter.next().unwrap().unwrap();
    let start: Vec<i32> = second_line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut start_mut = start;
    let sol = Solution {};
    println!("{}", sol.max_possible_score(&mut start_mut, d));
}