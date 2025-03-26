use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn max_good_number(nums: &mut Vec<i32>) -> i32 {
        nums.sort_by(|&a, &b| {
            let len_a = (a as f64).log2().floor() as i32 + 1;
            let len_b = (b as f64).log2().floor() as i32 + 1;
            let val_a = (a << len_b) | b;
            let val_b = (b << len_a) | a;
            val_b.cmp(&val_a)
        });

        let mut ans = 0;
        for &x in nums {
            let len_x = (x as f64).log2().floor() as i32 + 1;
            ans = (ans << len_x) | x;
        }
        ans
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the size of the vector
    let num_size: usize = lines.next().unwrap()?.trim().parse().unwrap();

    // Read the vector elements
    let mut nums: Vec<i32> = lines
        .take(num_size)
        .map(|line| line.unwrap().trim().parse().unwrap())
        .collect();

    // Calculate and print the result
    let result = Solution::max_good_number(&mut nums);
    println!("{}", result);

    Ok(())
}