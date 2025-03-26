use std::io;

fn count_valid_selections(nums: &[i32]) -> i32 {
    let mut sum_left = 0;
    let mut sum_right: i32 = nums.iter().sum();
    let mut ret = 0;

    for &num in nums {
        if num != 0 {
            sum_left += num;
            sum_right -= num;
        } else {
            let diff = sum_left - sum_right;
            if diff == 0 {
                ret += 2;
            } else if diff.abs() == 1 {
                ret += 1;
            }
        }
    }

    ret
}

fn main() {
    // Read entire input as a string and split into whitespace-separated tokens
    let input = io::read_to_string(io::stdin()).expect("Failed to read input");
    let mut tokens = input.split_whitespace();

    // Parse first token as array length
    let n: usize = tokens
        .next()
        .expect("No input provided")
        .parse()
        .expect("Invalid array length");

    // Parse next n tokens as array elements
    let nums: Vec<i32> = tokens
        .take(n)
        .map(|s| s.parse().expect("Invalid integer in array"))
        .collect();

    // Calculate and print result
    let result = count_valid_selections(&nums);
    println!("{}", result);
}