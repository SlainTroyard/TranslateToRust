use std::io;
use std::io::Read;

struct Solution {}

impl Solution {
    pub fn count_valid_selections(&self, nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut res = 0;
        let mut left = vec![0; n];
        let mut right = vec![0; n];

        // Compute prefix sums from the left
        for i in 1..n {
            left[i] = left[i - 1] + nums[i - 1];
        }

        // Compute prefix sums from the right
        for i in 1..n {
            right[n - i - 1] = right[n - i] + nums[n - i];
        }

        // Iterate through each element to count valid selections
        for i in 0..n {
            if nums[i] != 0 {
                continue;
            }
            if left[i] == right[i] {
                res += 2;
            }
            if (left[i] - right[i]).abs() == 1 {
                res += 1;
            }
        }
        res
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let n = lines.next().unwrap().parse::<usize>().unwrap();

    let nums: Vec<i32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let solution = Solution {};
    let result = solution.count_valid_selections(nums);

    println!("{}", result);

    Ok(())
}