use std::io::{self, BufRead};

// Define a struct Solution to mirror the C++ class
struct Solution;

impl Solution {
    // This function replicates the C++ getSneakyNumbers algorithm.
    // It takes a mutable slice of integers and returns a vector of two integers.
    fn get_sneaky_numbers(nums: &[i32]) -> Vec<i32> {
        // n is nums.len() - 2, as in C++ code.
        let n = (nums.len() - 2) as i32;
        // Compute the initial xor_all with n ^ (n + 1)
        let mut xor_all = n ^ (n + 1);
        
        // XOR accumulate for all indices and corresponding nums[i].
        for (i, &val) in nums.iter().enumerate() {
            xor_all ^= (i as i32) ^ val;
        }

        // Count trailing zeros in xor_all.
        // In Rust, trailing_zeros() returns u32.
        let shift = xor_all.trailing_zeros();

        // Prepare answer vector with 2 elements, all initialized to 0.
        let mut ans = vec![0, 0];

        // Iterate over indices and perform the appropriate XOR operations.
        for (i, &val) in nums.iter().enumerate() {
            // Only perform for indices less than n
            if (i as i32) < n {
                // (i >> shift) & 1 is computed by shifting and masking.
                ans[((i as i32) >> shift & 1) as usize] ^= i as i32;
            }
            // Process nums[i] partition similarly.
            ans[(val >> shift & 1) as usize] ^= val;
        }
        ans
    }
}

fn main() -> io::Result<()> {
    // Create a new buffered reader over stdin.
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut input = String::new();

    // Read the first line which contains the number of elements (without the added 2)
    reader.read_line(&mut input)?;
    let mut tokens = input.split_whitespace();
    // Parse the first integer.
    let num_size: usize = tokens
        .next()
        .expect("Expected the size")
        .parse()
        .expect("Parse error");
    
    // Clear input for further reading
    input.clear();
    
    // According to the C++ code, total number count is num_size + 2.
    let total_count = num_size + 2;
    let mut nums: Vec<i32> = Vec::with_capacity(total_count);
    
    // Read numbers until we get total_count integers.
    while nums.len() < total_count {
        // Read next line if needed.
        if input.trim().is_empty() {
            reader.read_line(&mut input)?;
        }
        // Tokenize current line and parse integers.
        for token in input.split_whitespace() {
            if nums.len() < total_count {
                let num: i32 = token.parse().expect("Invalid integer token");
                nums.push(num);
            } else {
                break;
            }
        }
        // Clear the input buffer to read next line.
        input.clear();
    }
    
    // Compute the sneaky numbers using the provided function.
    let result = Solution::get_sneaky_numbers(&nums);
    
    // Output the result exactly as in the C++ code: print two numbers followed by a space.
    // We use print! instead of println! to avoid an additional newline if not desired.
    for num in result {
        print!("{} ", num);
    }
    
    Ok(())
}