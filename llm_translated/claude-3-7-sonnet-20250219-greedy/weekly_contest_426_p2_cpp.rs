use std::collections::HashMap;
use std::io;

struct Solution;

impl Solution {
    pub fn get_largest_outlier(nums: Vec<i32>) -> i32 {
        let mut ctr: HashMap<i32, i32> = HashMap::new();
        let mut sm = 0;

        for num in &nums {
            *ctr.entry(*num).or_insert(0) += 1;
            sm += num;
        }

        // Collect unique numbers
        let mut cands: Vec<i32> = ctr.keys().cloned().collect();
        
        // Sort in descending order
        cands.sort_unstable_by(|a, b| b.cmp(a));

        for n in cands {
            let d = (sm - n) / 2;
            let m = (sm - n) % 2;
            
            // Check if d exists in the map and either d != n or count of d > 1
            if m == 0 && ctr.contains_key(&d) && (d != n || ctr.get(&d).unwrap() > &1) {
                return n;
            }
        }
        
        -1
    }
}

fn main() {
    let mut input = String::new();
    
    // Input the number of elements
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Invalid input");
    
    // Input the elements of the array
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid number"))
        .collect();
    
    // Call the method and output the result
    let result = Solution::get_largest_outlier(nums);
    println!("{}", result);
}