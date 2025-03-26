use std::cmp::min;
use std::io;

fn main() {
    // Read the size of the array
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let n: usize = input.trim().parse().expect("Invalid input for array size");

    // Read the array elements
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let num: i32 = input.trim().parse().expect("Invalid input for array element");
        nums.push(num);
    }

    // Read the range [l, r]
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let parts: Vec<usize> = input.trim().split_whitespace()
        .map(|s| s.parse().expect("Invalid input for range"))
        .collect();
    let l = parts[0];
    let r = parts[1];

    // Compute the minimum sum subarray
    let result = minimum_sum_subarray(&nums, l, r);

    // Output the result
    println!("{}", result);
}

fn minimum_sum_subarray(nums: &[i32], l: usize, r: usize) -> i32 {
    let n = nums.len();
    let mut mini = i32::MAX;

    for i in 0..n {
        let mut currsum = 0;
        for j in i..n {
            currsum += nums[j];
            let length = j - i + 1;
            if length >= l && length <= r && currsum > 0 {
                mini = min(mini, currsum);
            }
        }
    }

    if mini == i32::MAX {
        -1
    } else {
        mini
    }
}