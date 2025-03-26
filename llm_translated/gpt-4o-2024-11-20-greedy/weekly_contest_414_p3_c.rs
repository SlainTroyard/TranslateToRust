use std::io::{self, Write};

fn find_maximum_score(nums: &[i32]) -> i64 {
    let mut ans: i64 = 0;
    let mut l: usize = 0;
    let mut r: usize = 1;

    while r < nums.len() {
        if nums[l] < nums[r] {
            ans += (r - l) as i64 * nums[l] as i64;
            l = r;
        }
        r += 1;
    }

    ans + (r - l - 1) as i64 * nums[l] as i64
}

fn main() {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let nums_size: usize = input.trim().parse().expect("Failed to parse numsSize");

    let mut nums = Vec::with_capacity(nums_size);
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    nums.extend(
        input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i32>().expect("Failed to parse number")),
    );

    // Ensure the input size matches nums_size
    assert_eq!(nums.len(), nums_size, "Input size does not match numsSize");

    // Compute the result
    let result = find_maximum_score(&nums);

    // Print the result
    println!("{}", result);
}