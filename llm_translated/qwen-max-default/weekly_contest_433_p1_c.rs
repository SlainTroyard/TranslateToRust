use std::io;

/// Translates the given C code to idiomatic Rust.
///
/// This function calculates the sum of all subarrays for a given array of integers.
fn subarray_sum(nums: &[i32]) -> i32 {
    let n = nums.len();
    let mut ans = 0;
    let mut sums = vec![0; n + 1];

    for i in 0..n {
        sums[i + 1] = nums[i] + sums[i];
        let start = if i > nums[i] as usize { i - nums[i] as usize } else { 0 };
        ans += sums[i + 1] - sums[start];
    }

    ans
}

fn main() {
    // Read the number of elements from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Please type a number!");

    // Allocate and read the array of numbers from stdin
    let mut nums: Vec<i32> = Vec::with_capacity(n);
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let num: i32 = input.trim().parse().expect("Please type a number!");
        nums.push(num);
    }

    // Calculate the result and print it to stdout
    let result = subarray_sum(&nums);
    println!("{}", result);
}