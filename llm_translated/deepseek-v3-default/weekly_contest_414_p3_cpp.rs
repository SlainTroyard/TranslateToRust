use std::io;
use std::cmp::max;

struct Solution;

impl Solution {
    // Function to calculate the maximum score based on the given algorithm
    fn find_maximum_score(nums: Vec<i32>) -> i64 {
        let mut ans: i64 = 0;
        let mut mx = 0;
        for i in 0..nums.len() - 1 {
            mx = max(mx, nums[i]);
            ans += mx as i64;
        }
        ans
    }
}

fn main() {
    // Read the size of the vector from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let nums_size: usize = input.trim().parse().expect("Please type a number!");

    // Read the vector elements from stdin
    let mut nums = Vec::with_capacity(nums_size);
    for _ in 0..nums_size {
        let mut num_input = String::new();
        io::stdin().read_line(&mut num_input).expect("Failed to read line");
        let num: i32 = num_input.trim().parse().expect("Please type a number!");
        nums.push(num);
    }

    // Create an instance of Solution and calculate the maximum score
    let sol = Solution;
    let result = sol.find_maximum_score(nums);

    // Print the result
    println!("{}", result);
}