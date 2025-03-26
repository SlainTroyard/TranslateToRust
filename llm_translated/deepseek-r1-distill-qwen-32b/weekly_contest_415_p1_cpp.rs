struct Solution;

impl Solution {
    fn get_sneaky_numbers(&self, nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len() - 2;
        let mut xor_all = (n as i32) ^ ((n + 1) as i32);
        for (i, num) in nums.iter().enumerate() {
            xor_all ^= (i as i32) ^ num;
        }
        let shift = xor_all.trailing_zeros();
        let mut ans = vec![0; 2];
        for (i, num) in nums.iter().enumerate() {
            if i < n {
                let idx = ((i as u32) >> shift) & 1;
                ans[idx as usize] ^= i as i32;
            }
            let idx = ((num as u32) >> shift) & 1;
            ans[idx as usize] ^= num;
        }
        ans
    }
}

fn main() -> std::io::Result<()> {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first line as num_size
    let num_size_line = lines.next().unwrap()?;
    let original_num_size: i32 = num_size_line.trim().parse()?;

    let total_numbers = original_num_size as usize + 2;

    // Read the next line(s) for the numbers
    let mut nums = Vec::with_capacity(total_numbers);
    while nums.len() < total_numbers {
        let line = lines.next().unwrap()?;
        let parts: Vec<i32> = line.trim().split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        nums.extend(parts);
    }

    let solution = Solution;
    let result = solution.get_sneaky_numbers(nums);

    println!("{} {}", result[0], result[1]);

    Ok(())
}