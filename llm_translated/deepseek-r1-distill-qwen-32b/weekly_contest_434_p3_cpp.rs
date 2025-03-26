use std::io;

struct Solution;

impl Solution {
    fn max_frequency(&self, nums: Vec<i32>, k: i32) -> i32 {
        let mut f0 = 0;
        let mut f1 = [0; 51];
        let mut max_f1 = 0;
        let mut f2 = 0;
        
        for x in nums {
            f2 = std::cmp::max(f2, max_f1) + if x == k { 1 } else { 0 };
            f1[x as usize] = std::cmp::max(f1[x as usize], f0) + 1;
            if x == k {
                f0 += 1;
            }
            if f1[x as usize] > max_f1 {
                max_f1 = f1[x as usize];
            }
        }
        
        std::cmp::max(max_f1, f2)
    }
}

fn main() {
    let mut input = io::stdin().lines();
    let first_line = input.next().unwrap().unwrap();
    let second_line = input.next().unwrap().unwrap();
    
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let k: i32 = parts.next().unwrap().parse().unwrap();
    
    let nums: Vec<i32> = second_line
        .split_whitespace()
        .take(n)
        .map(|s| s.parse().unwrap())
        .collect();
    
    let solution = Solution;
    println!("{}", solution.max_frequency(nums, k));
}