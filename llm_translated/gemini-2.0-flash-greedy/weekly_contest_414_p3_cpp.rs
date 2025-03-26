use std::io;

struct Solution {}

impl Solution {
    pub fn find_maximum_score(nums: Vec<i32>) -> i64 {
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
    let mut nums_size_str = String::new();
    io::stdin()
        .read_line(&mut nums_size_str)
        .expect("Failed to read line");
    let nums_size: usize = nums_size_str.trim().parse().expect("Invalid input");

    let mut nums: Vec<i32> = Vec::with_capacity(nums_size);
    let mut nums_str = String::new();
    io::stdin()
        .read_line(&mut nums_str)
        .expect("Failed to read line");

    for num_str in nums_str.trim().split_whitespace() {
        let num: i32 = num_str.parse().expect("Invalid input");
        nums.push(num);
    }

    let sol = Solution {};
    println!("{}", sol.find_maximum_score(nums));
}