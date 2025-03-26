use std::cmp::Ordering;

struct Solution;

impl Solution {
    pub fn max_good_number(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        // Custom sort based on concatenated value comparison
        nums.sort_by(|a, b| {
            let len_a = a.ilog2() + 1; // Calculate bit length of a
            let len_b = b.ilog2() + 1; // Calculate bit length of b
            
            // Compute concatenated values for comparison
            let a_concat = (a << len_b) | b;
            let b_concat = (b << len_a) | a;
            
            // Sort in descending order of concatenated value
            b_concat.cmp(&a_concat)
        });
        
        let mut ans = 0;
        for &x in &nums {
            let len_x = x.ilog2() + 1; // Get bit length of current number
            ans = (ans << len_x) | x;   // Shift and concatenate
        }
        ans
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut tokens = input.split_whitespace();
    
    // Parse number of elements
    let num_size: usize = tokens.next()
        .expect("Missing number of elements")
        .parse()
        .expect("Invalid number of elements");
    
    // Parse numbers
    let nums: Vec<i32> = tokens
        .take(num_size)
        .map(|s| s.parse().expect("Invalid number"))
        .collect();
    
    // Ensure exactly num_size numbers were provided
    if nums.len() != num_size {
        panic!("Insufficient numbers provided");
    }
    
    let result = Solution::max_good_number(nums);
    println!("{}", result);
}