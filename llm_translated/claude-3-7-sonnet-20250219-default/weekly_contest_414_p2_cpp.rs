use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn max_possible_score(start: &mut Vec<i32>, d: i32) -> i32 {
        start.sort();

        // Closure to check if a given score is possible
        let check = |score: i32| -> bool {
            let mut x = i64::MIN;
            for &s in start {
                x = x.saturating_add(score as i64).max(s as i64);
                if x > (s as i64) + (d as i64) {
                    return false;
                }
            }
            true
        };

        // Binary search to find the maximum valid score
        let mut left = 0;
        let mut right = (start.last().unwrap() + d - start[0]) / ((start.len() - 1) as i32) + 1;
        
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
    
    // Read the input values
    let input_line = lines.next().unwrap().unwrap();
    let mut values = input_line.split_whitespace();
    let start_size: usize = values.next().unwrap().parse().unwrap();
    let d: i32 = values.next().unwrap().parse().unwrap();
    
    // Read the start array
    let input_line = lines.next().unwrap().unwrap();
    let mut start: Vec<i32> = input_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let sol = Solution;
    println!("{}", sol.max_possible_score(&mut start, d));
}