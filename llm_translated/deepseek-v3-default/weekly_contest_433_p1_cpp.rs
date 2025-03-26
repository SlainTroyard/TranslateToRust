use std::io;
use std::cmp::max;

fn main() {
    // Read the number of elements in the array
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let n: usize = input.trim().parse().expect("Invalid input");

    // Read the array elements
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let num: i32 = input.trim().parse().expect("Invalid input");
        nums.push(num);
    }

    // Calculate the result using the subarray_sum function
    let result = subarray_sum(&nums);

    // Print the result
    println!("{}", result);
}

fn subarray_sum(nums: &[i32]) -> i32 {
    let n = nums.len();
    let mut s = vec![0; n + 1];

    // Compute the prefix sum array
    for i in 0..n {
        s[i + 1] = s[i] + nums[i];
    }

    let mut ans = 0;
    for i in 0..n {
        // Calculate the sum of the subarray and add it to the answer
        ans += s[i + 1] - s[max(i as i32 - nums[i], 0) as usize];
    }

    ans
}