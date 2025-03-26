use std::io;

struct Solution {}

impl Solution {
    pub fn find_maximum_score(&self, nums: &Vec<i32>) -> i64 {
        let mut ans: i64 = 0;
        let mut mx: i32 = 0;
        for i in 0..nums.len() - 1 {
            mx = mx.max(nums[i]);
            ans += mx as i64;
        }
        ans
    }
}

fn main() {
    // Read numsSize from stdin
    let mut nums_size_str = String::new();
    io::stdin().read_line(&mut nums_size_str).expect("Failed to read line");
    let nums_size: usize = nums_size_str.trim().parse().expect("Invalid input for numsSize");

    // Create a vector to store nums
    let mut nums: Vec<i32> = Vec::with_capacity(nums_size);

    // Read nums from stdin
    for _ in 0..nums_size {
        let mut num_str = String::new();
        io::stdin().read_line(&mut num_str).expect("Failed to read line");
        let num: i32 = num_str.trim().parse().expect("Invalid input for num");
        nums.push(num);
    }

    // Create a Solution instance
    let sol = Solution {};

    // Call findMaximumScore and print the result to stdout
    println!("{}", sol.find_maximum_score(&nums));
}