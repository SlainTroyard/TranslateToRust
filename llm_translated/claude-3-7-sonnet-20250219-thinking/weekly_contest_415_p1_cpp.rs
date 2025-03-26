use std::io;

struct Solution;

impl Solution {
    fn get_sneaky_numbers(nums: &Vec<i32>) -> Vec<i32> {
        let n = nums.len() - 2;
        let mut xor_all = (n as i32) ^ ((n + 1) as i32);
        
        for i in 0..nums.len() {
            xor_all ^= i as i32 ^ nums[i];
        }
        
        // Equivalent to __builtin_ctz in C++
        let shift = xor_all.trailing_zeros();

        let mut ans = vec![0, 0];
        for i in 0..nums.len() {
            if i < n {
                ans[((i as i32) >> shift & 1) as usize] ^= i as i32;
            }
            ans[(nums[i] >> shift & 1) as usize] ^= nums[i];
        }
        
        ans
    }
}

fn main() {
    // Read the size
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut num_size: usize = input.trim().parse().expect("Failed to parse numSize");
    num_size += 2; // Match C++ behavior where numSize is incremented by 2
    
    // Read the array elements
    let mut nums = Vec::with_capacity(num_size);
    for _ in 0..num_size {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let num: i32 = input.trim().parse().expect("Failed to parse number");
        nums.push(num);
    }
    
    // Solve and output
    let solution = Solution;
    let result = solution.get_sneaky_numbers(&nums);
    
    // Format output exactly as in the original C++ code
    for i in 0..2 {
        print!("{}", result[i]);
        if i < 1 {
            print!(" ");
        }
    }
}