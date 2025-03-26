use std::io::Read;

struct Solution;

impl Solution {
    pub fn max_possible_score(start: &mut Vec<i32>, d: i32) -> i32 {
        start.sort();
        
        let check = |score: i32| -> bool {
            let mut x = i64::MIN;
            for &s in start.iter() {
                x = (x + score as i64).max(s as i64);
                if x > (s as i64 + d as i64) {
                    return false;
                }
            }
            true
        };
        
        // Calculate the initial right bound for binary search
        let start_last = *start.last().unwrap();
        let start_first = start[0];
        let numerator = start_last + d - start_first;
        let denominator = (start.len() - 1) as i32;
        let mut left = 0;
        let mut right = (numerator / denominator) + 1;
        
        // Binary search to find the maximum possible score
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
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();
    
    let start_size: usize = tokens.next().unwrap().parse().unwrap();
    let d: i32 = tokens.next().unwrap().parse().unwrap();
    
    let mut start: Vec<i32> = (0..start_size)
        .map(|_| tokens.next().unwrap().parse().unwrap())
        .collect();
    
    let result = Solution::max_possible_score(&mut start, d);
    println!("{}", result);
}