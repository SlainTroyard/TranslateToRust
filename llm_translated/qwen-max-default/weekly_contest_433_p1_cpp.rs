use std::io::{self, BufRead, Write};

fn subarray_sum(nums: &Vec<i32>) -> i32 {
    let n = nums.len();
    let mut s = vec![0; n + 1];
    for i in 0..n {
        s[i + 1] = s[i] + nums[i];
    }

    let mut ans = 0;
    for i in 0..n {
        let start = if i as i32 - nums[i] >= 0 { (i as i32 - nums[i]) as usize } else { 0 };
        ans += s[i + 1] - s[start];
    }
    ans
}

fn main() {
    // Read the number of elements from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Please type a number!");

    // Read the elements into a vector
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let num: i32 = input.trim().parse().expect("Please type a number!");
        nums.push(num);
    }

    // Calculate and print the result
    let solution = subarray_sum(&nums);
    println!("{}", solution);
}