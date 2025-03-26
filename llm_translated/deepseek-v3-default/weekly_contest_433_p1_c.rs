use std::io::{self, Write};

fn main() {
    // Read the number of elements
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Please type a number!");

    // Read the array of numbers
    let mut nums = Vec::with_capacity(n);
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    for num_str in input.split_whitespace() {
        let num: i32 = num_str.parse().expect("Please type a number!");
        nums.push(num);
    }

    // Calculate and print the result
    let result = subarray_sum(&nums);
    println!("{}", result);
}

fn subarray_sum(nums: &[i32]) -> i32 {
    let mut ans = 0;
    let mut sums = vec![0; nums.len() + 1];

    for i in 0..nums.len() {
        sums[i + 1] = nums[i] + sums[i];
        let start = if i as i32 - nums[i] < 0 {
            0
        } else {
            (i as i32 - nums[i]) as usize
        };
        ans += sums[i + 1] - sums[start];
    }

    ans
}