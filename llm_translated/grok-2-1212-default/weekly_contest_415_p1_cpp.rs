use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn get_sneaky_numbers(nums: &Vec<i32>) -> Vec<i32> {
        let n = nums.len() as i32 - 2;
        let mut xor_all = n ^ (n + 1);
        for (i, &num) in nums.iter().enumerate() {
            xor_all ^= i as i32 ^ num;
        }
        let shift = xor_all.trailing_zeros() as i32;

        let mut ans = vec![0, 0];
        for (i, &num) in nums.iter().enumerate() {
            if i as i32 < n {
                ans[((i as i32) >> shift & 1) as usize] ^= i as i32;
            }
            ans[(num >> shift & 1) as usize] ^= num;
        }
        ans
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the size of the vector
    let num_size: usize = lines.next().unwrap()?.trim().parse().unwrap();
    let num_size = num_size + 2;

    // Read the vector elements
    let mut nums = Vec::with_capacity(num_size);
    for _ in 0..num_size {
        let num: i32 = lines.next().unwrap()?.trim().parse().unwrap();
        nums.push(num);
    }

    // Process the vector
    let result = Solution::get_sneaky_numbers(&nums);

    // Output the result
    for i in 0..2 {
        print!("{} ", result[i]);
    }
    println!();

    Ok(())
}