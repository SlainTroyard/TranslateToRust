use std::collections::HashMap;
use std::io::{self, BufRead};

struct Solution {}

impl Solution {
    pub fn find_x_sum(&self, nums: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let mut mp: HashMap<i32, i32> = HashMap::new(); // Use HashMap in Rust
        let mut res: Vec<i32> = Vec::new();
        let mut l = 0;
        for r in 0..nums.len() {
            *mp.entry(nums[r]).or_insert(0) += 1; // Increment count in HashMap

            if (r as i32 - l as i32 + 1) == k {
                let mut vec: Vec<(i32, i32)> = mp.iter().map(|(&num, &cnt)| (num, cnt)).collect(); // Convert HashMap to Vec of tuples

                vec.sort_by(|&lhs, &rhs| { // Sort using sort_by with closure
                    if rhs.1 == lhs.1 {
                        rhs.0.cmp(&lhs.0) // Descending order of num if counts are same
                    } else {
                        rhs.1.cmp(&lhs.1) // Descending order of count
                    }
                });

                let mut sum = 0;
                for i in 0..std::cmp::min(x as usize, vec.len()) { // Iterate up to x or vec size
                    sum += vec[i].0 * vec[i].1;
                }
                res.push(sum);

                if let Some(count) = mp.get_mut(&nums[l]) { // Decrement count for the leftmost element
                    *count -= 1;
                    if *count == 0 {
                        mp.remove(&nums[l]); // Remove from HashMap if count becomes 0
                    }
                }
                l += 1;
            }
        }
        res
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut first_line_iter = first_line.split_whitespace();
    let k: i32 = first_line_iter.next().unwrap().parse().unwrap();
    let x: i32 = first_line_iter.next().unwrap().parse().unwrap();

    let second_line = lines.next().unwrap().unwrap();
    let nums_size: usize = second_line.trim().parse().unwrap();

    let third_line = lines.next().unwrap().unwrap();
    let nums_str: Vec<&str> = third_line.split_whitespace().collect();
    let mut nums: Vec<i32> = Vec::new();
    for s in nums_str {
        nums.push(s.parse().unwrap());
    }

    let s = Solution {};
    let res = s.find_x_sum(nums, k, x);

    for i in 0..res.len() {
        print!("{} ", res[i]);
    }
    println!();
}