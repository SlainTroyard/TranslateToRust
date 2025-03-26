use std::io::{self, BufRead, Write};

struct Solution;

impl Solution {
    fn minimum_sum_subarray(nums: &Vec<i32>, l: usize, r: usize) -> i32 {
        let n = nums.len();
        let mut mini = i32::MAX;
        for i in 0..n {
            let mut currsum = 0;
            for j in i..n {
                currsum += nums[j];
                let length = j - i + 1;
                if (l..=r).contains(&length) && currsum > 0 {
                    mini = mini.min(currsum);
                }
            }
        }
        if mini == i32::MAX { -1 } else { mini }
    }
}

fn main() {
    // Input the size of the array
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut stdout = io::stdout();
    let mut stdout_lock = stdout.lock();

    let mut input = String::new();
    stdin_lock.read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().expect("Invalid input");

    // Input the array elements
    input.clear();
    stdin_lock.read_line(&mut input).unwrap();
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Invalid input"))
        .collect();

    // Input the range [l, r]
    input.clear();
    stdin_lock.read_line(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let l: usize = iter.next().unwrap().parse().expect("Invalid input");
    let r: usize = iter.next().unwrap().parse().expect("Invalid input");

    // Compute the minimum sum subarray
    let solution = Solution;
    let result = solution.minimum_sum_subarray(&nums, l, r);

    // Output the result
    writeln!(stdout_lock, "{}", result).unwrap();
}