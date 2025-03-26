use std::collections::HashMap;
use std::io;

struct Solution;

impl Solution {
    fn find_x_sum(nums: &Vec<i32>, k: usize, x: usize) -> Vec<i32> {
        let mut mp = HashMap::new();
        let mut res = Vec::new();
        
        let mut l = 0;
        for r in 0..nums.len() {
            // Add the current number to the frequency map
            *mp.entry(nums[r]).or_insert(0) += 1;
            
            if r - l + 1 == k {
                // Convert the map to a vector of (number, count) pairs for sorting
                let mut vec: Vec<(i32, i32)> = mp.iter().map(|(&k, &v)| (k, v)).collect();
                
                // Sort by frequency (descending), then by value (descending) if frequencies are equal
                vec.sort_by(|a, b| {
                    if a.1 == b.1 {
                        b.0.cmp(&a.0) // If counts are equal, larger numbers come first
                    } else {
                        b.1.cmp(&a.1) // Higher frequency comes first
                    }
                });
                
                // Calculate sum of the top x numbers (counting each number multiple times based on frequency)
                let mut sum = 0;
                for i in 0..x.min(vec.len()) {
                    sum += vec[i].0 * vec[i].1;
                }
                res.push(sum);
                
                // Remove the leftmost element from the window
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

fn main() -> io::Result<()> {
    // Read k and x
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let parts: Vec<i32> = input.trim().split_whitespace()
        .map(|s| s.parse().unwrap()).collect();
    let k = parts[0] as usize;
    let x = parts[1] as usize;
    
    // Read numsSize
    input.clear();
    io::stdin().read_line(&mut input)?;
    let nums_size = input.trim().parse::<usize>().unwrap();
    
    // Read nums
    input.clear();
    io::stdin().read_line(&mut input)?;
    let nums: Vec<i32> = input.trim().split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Get and print the result
    let solution = Solution;
    let res = Solution::find_x_sum(&nums, k, x);
    
    for i in 0..res.len() {
        print!("{} ", res[i]);
    }
    println!();
    
    Ok(())
}