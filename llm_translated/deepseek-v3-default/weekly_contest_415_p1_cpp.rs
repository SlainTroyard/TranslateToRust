use std::io::{self, BufRead};

fn main() {
    // Read the size of the array from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let num_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Adjust the size by adding 2, as per the original C++ code
    let num_size = num_size + 2;
    
    // Read the array elements from stdin
    let mut nums = Vec::with_capacity(num_size);
    for _ in 0..num_size {
        let num: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
        nums.push(num);
    }
    
    // Call the function to get the sneaky numbers
    let result = get_sneaky_numbers(nums);
    
    // Print the result
    println!("{} {}", result[0], result[1]);
}

fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len() - 2;
    let mut xor_all = (n as i32) ^ ((n + 1) as i32);
    
    // XOR all indices and values to find the difference
    for (i, &num) in nums.iter().enumerate() {
        xor_all ^= (i as i32) ^ num;
    }
    
    // Find the least significant bit that is set
    let shift = xor_all.trailing_zeros();
    
    // Initialize the result vector
    let mut ans = vec![0, 0];
    
    // Separate the numbers based on the shift bit
    for (i, &num) in nums.iter().enumerate() {
        if i < n {
            ans[(i >> shift) & 1] ^= i as i32;
        }
        ans[(num >> shift) & 1] ^= num;
    }
    
    ans
}