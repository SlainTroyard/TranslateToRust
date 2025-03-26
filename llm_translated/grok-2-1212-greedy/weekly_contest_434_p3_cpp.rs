use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn max_frequency(nums: &Vec<i32>, k: i32) -> i32 {
        let mut f0 = 0;
        let mut f1 = [0; 51];
        let mut max_f1 = 0;
        let mut f2 = 0;
        
        for &x in nums {
            f2 = f2.max(max_f1) + if x == k { 1 } else { 0 };
            f1[x as usize] = f1[x as usize].max(f0) + 1;
            f0 += if x == k { 1 } else { 0 };
            max_f1 = max_f1.max(f1[x as usize]);
        }
        
        max_f1.max(f2)
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read n and k
    let first_line = lines.next().unwrap()?;
    let mut iter = first_line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();

    // Read nums
    let second_line = lines.next().unwrap()?;
    let nums: Vec<i32> = second_line.split_whitespace().map(|s| s.parse().unwrap()).collect();

    // Calculate and print result
    let result = Solution::max_frequency(&nums, k);
    println!("{}", result);

    Ok(())
}