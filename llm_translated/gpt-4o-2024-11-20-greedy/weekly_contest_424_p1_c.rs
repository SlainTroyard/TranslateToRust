use std::io::{self, BufRead};

fn count_valid_selections(nums: &[i32]) -> i32 {
    let mut sum_left = 0;
    let mut sum_right: i32 = nums.iter().sum();
    let mut ret = 0;

    for &num in nums {
        if num != 0 {
            sum_left += num;
            sum_right -= num;
        } else {
            if sum_left == sum_right {
                ret += 2;
            } else if (sum_left - sum_right).abs() == 1 {
                ret += 1;
            }
        }
    }

    ret
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first line for the size of the array
    let n: usize = lines
        .next()
        .expect("Expected input for array size")
        .expect("Failed to read line")
        .trim()
        .parse()
        .expect("Failed to parse array size");

    // Read the second line for the array elements
    let nums: Vec<i32> = lines
        .next()
        .expect("Expected input for array elements")
        .expect("Failed to read line")
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse array element"))
        .collect();

    // Ensure the input array size matches the declared size
    assert_eq!(nums.len(), n, "Array size does not match the declared size");

    // Compute the result
    let result = count_valid_selections(&nums);

    // Print the result
    println!("{}", result);
}