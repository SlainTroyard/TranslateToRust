use std::io::{self, BufRead, Write};

fn main() {
    // Read the size of the input vector from stdin
    let mut num_size = String::new();
    io::stdin().read_line(&mut num_size).expect("Failed to read line");
    let num_size: usize = num_size.trim().parse().expect("Please type a number!");
    let num_size = num_size + 2;

    // Initialize the vector with the given size and read the values
    let mut nums: Vec<i32> = vec![0; num_size];
    for i in 0..num_size {
        nums[i] = read_int();
    }

    // Create an instance of Solution and get the result
    let solution = Solution;
    let result = solution.get_sneaky_numbers(nums);

    // Print the result
    for &num in &result {
        print!("{} ", num);
    }
}

// Helper function to read an integer from stdin
fn read_int() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Please type a number!")
}

struct Solution;

impl Solution {
    pub fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len() - 2;
        let xor_all = n as i32 ^ (n as i32 + 1);
        let mut xor_all = xor_all;
        for (i, &num) in nums.iter().enumerate() {
            xor_all ^= i as i32 ^ num;
        }
        let shift = xor_all.trailing_zeros();

        let mut ans = vec![0; 2];
        for (i, &num) in nums.iter().enumerate() {
            if i < n {
                ans[(i >> shift) as usize & 1] ^= i as i32;
            }
            ans[(num >> shift) as usize & 1] ^= num;
        }
        ans
    }
}