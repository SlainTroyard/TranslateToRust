use std::io;
use std::cmp;

struct Solution;

impl Solution {
    fn max_possible_score(start: &mut Vec<i32>, d: i32) -> i32 {
        start.sort();

        let check = |score: i32| -> bool {
            let mut x: i64 = i64::MIN;
            for &s in start {
                x = cmp::max(x + score as i64, s as i64);
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
    // Read startSize and d
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut parts = input.trim().split_whitespace();
    
    let start_size: usize = parts.next().unwrap().parse().expect("Invalid start_size");
    let d: i32 = parts.next().unwrap().parse().expect("Invalid d");
    
    // Read the start vector
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut start: Vec<i32> = input.trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid number in start vector"))
        .collect();
    
    // Ensure we have the correct number of elements
    if start.len() < start_size {
        let remaining = start_size - start.len();
        for _ in 0..remaining {
            input.clear();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let num: i32 = input.trim().parse().expect("Invalid number");
            start.push(num);
        }
    }
    
    // Run the algorithm and print the result
    let result = Solution::max_possible_score(&mut start, d);
    println!("{}", result);
}