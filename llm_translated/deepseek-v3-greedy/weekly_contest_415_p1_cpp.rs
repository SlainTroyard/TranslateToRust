use std::io::{self, BufRead};

fn main() {
    // Read the first line to get the size of the input array
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let num_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let num_size = num_size + 2; // Adjust the size as per the original code

    // Read the next line to get the array elements
    let nums_line = lines.next().unwrap().unwrap();
    let nums: Vec<i32> = nums_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Ensure the input size matches the expected size
    assert_eq!(nums.len(), num_size);

    // Call the function to get the sneaky numbers
    let result = get_sneaky_numbers(nums);

    // Print the result in the same format as the original code
    println!("{} {}", result[0], result[1]);
}

fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len() - 2;
    let mut xor_all = (n as i32) ^ ((n + 1) as i32);

    // XOR all indices and elements to find the two unique numbers
    for (i, &num) in nums.iter().enumerate() {
        xor_all ^= (i as i32) ^ num;
    }

    // Find the rightmost set bit
    let shift = xor_all.trailing_zeros();

    // Separate the numbers based on the shift bit
    let mut ans = vec![0, 0];
    for (i, &num) in nums.iter().enumerate() {
        if i < n {
            ans[((i as i32) >> shift) as usize & 1] ^= i as i32;
        }
        ans[(num >> shift) as usize & 1] ^= num;
    }

    ans
}