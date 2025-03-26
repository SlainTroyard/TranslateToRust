use std::io::{self, BufRead, Write};

/// A function to calculate the maximum good number from a list of integers.
fn max_good_number(nums: &mut Vec<i32>) -> i32 {
    // Sort the numbers based on a custom comparator
    nums.sort_unstable_by(|&a, &b| {
        let len_a = (a as f64).log2().floor() as u32 + 1;
        let len_b = (b as f64).log2().floor() as u32 + 1;
        ((a << len_b) | b).cmp(&((b << len_a) | a))
    });

    // Construct the maximum good number
    let mut ans = 0;
    for &x in nums.iter() {
        let len_x = (x as f64).log2().floor() as u32 + 1;
        ans = (ans << len_x) | x;
    }
    ans
}

fn main() {
    // Read the size of the input vector
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buffer = String::new();
    stdin.lock().read_line(&mut buffer).expect("Failed to read line");
    let num_size: usize = buffer.trim().parse().expect("Please type a number!");

    // Read the input numbers
    let mut nums = vec![0; num_size];
    for i in 0..num_size {
        buffer.clear();
        stdin.lock().read_line(&mut buffer).expect("Failed to read line");
        nums[i] = buffer.trim().parse().expect("Please type a number!");
    }

    // Calculate and print the result
    let solution = max_good_number(&mut nums);
    writeln!(stdout, "{}", solution).expect("Failed to write to stdout");
}