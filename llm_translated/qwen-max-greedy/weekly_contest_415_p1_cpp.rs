use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buffer = String::new();

    // Read the size of the input array
    stdin.lock().read_line(&mut buffer).unwrap();
    let num_size: usize = buffer.trim().parse().unwrap();
    buffer.clear();

    // Initialize the vector with the correct size
    let mut nums: Vec<i32> = vec![0; num_size + 2];

    // Read the elements of the vector
    for i in 0..num_size + 2 {
        stdin.lock().read_line(&mut buffer).unwrap();
        nums[i] = buffer.trim().parse().unwrap();
        buffer.clear();
    }

    // Create a solution instance and get the result
    let solution = Solution;
    let result = solution.get_sneaky_numbers(&nums);

    // Print the result
    for &num in &result {
        write!(stdout, "{} ", num).unwrap();
    }
}

struct Solution;

impl Solution {
    /// Finds the two sneaky numbers in the given vector.
    ///
    /// # Arguments
    ///
    /// * `nums` - A reference to the vector of integers.
    ///
    /// # Returns
    ///
    /// A vector containing the two sneaky numbers.
    fn get_sneaky_numbers(nums: &[i32]) -> Vec<i32> {
        let n = nums.len() - 2;
        let mut xor_all = n as i32 ^ (n as i32 + 1);
        for (i, &num) in nums.iter().enumerate() {
            xor_all ^= i as i32 ^ num;
        }
        let shift = xor_all.trailing_zeros() as u32;

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