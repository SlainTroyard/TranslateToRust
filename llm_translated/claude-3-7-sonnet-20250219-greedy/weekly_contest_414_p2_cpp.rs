use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn max_possible_score(start: &mut Vec<i32>, d: i32) -> i32 {
        start.sort();

        let check = |score: i32| -> bool {
            let mut x: i64 = i64::MIN;
            for &s in start.iter() {
                x = x.saturating_add(score as i64).max(s as i64);
                if x > (s + d) as i64 {
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
    let mut lines = stdin.lock().lines();
    
    // Read the first line containing startSize and d
    let first_line = lines.next().unwrap().unwrap();
    let mut iter = first_line.split_whitespace();
    let start_size: usize = iter.next().unwrap().parse().unwrap();
    let d: i32 = iter.next().unwrap().parse().unwrap();
    
    // Read the second line containing the start array
    let second_line = lines.next().unwrap().unwrap();
    let mut start: Vec<i32> = second_line
        .split_whitespace()
        .take(start_size)
        .map(|s| s.parse().unwrap())
        .collect();
    
    let result = Solution::max_possible_score(&mut start, d);
    println!("{}", result);
}