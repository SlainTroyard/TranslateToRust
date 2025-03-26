use std::collections::HashMap;
use std::io;

struct Solution;

impl Solution {
    pub fn find_x_sum(nums: &Vec<i32>, k: usize, x: usize) -> Vec<i32> {
        let mut mp: HashMap<i32, i32> = HashMap::new();
        let mut res: Vec<i32> = Vec::new();
        let mut l = 0;
        
        for r in 0..nums.len() {
            *mp.entry(nums[r]).or_insert(0) += 1;
            
            if r - l + 1 == k {
                // Convert HashMap to Vec of pairs for sorting
                let mut vec: Vec<(i32, i32)> = mp.iter().map(|(&k, &v)| (k, v)).collect();
                
                // Sort by frequency (descending), then by value (descending)
                vec.sort_by(|lhs, rhs| {
                    if lhs.1 == rhs.1 {
                        // If frequencies are equal, sort by value (descending)
                        rhs.0.cmp(&lhs.0)
                    } else {
                        // Sort by frequency (descending)
                        rhs.1.cmp(&lhs.1)
                    }
                });
                
                // Calculate sum of top x elements
                let mut sum = 0;
                for i in 0..x.min(vec.len()) {
                    sum += vec[i].0 * vec[i].1;
                }
                res.push(sum);
                
                // Remove leftmost element from the window
                *mp.get_mut(&nums[l]).unwrap() -= 1;
                if mp[&nums[l]] == 0 {
                    mp.remove(&nums[l]);
                }
                l += 1;
            }
        }
        
        res
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read k and x");
    let mut values = input.trim().split_whitespace();
    
    let k: usize = values.next().unwrap().parse().expect("Failed to parse k");
    let x: usize = values.next().unwrap().parse().expect("Failed to parse x");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read nums size");
    let nums_size: usize = input.trim().parse().expect("Failed to parse nums size");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read nums");
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse num"))
        .collect();
    
    let s = Solution;
    let res = s.find_x_sum(&nums, k, x);
    
    for (i, val) in res.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", val);
    }
    println!();
}