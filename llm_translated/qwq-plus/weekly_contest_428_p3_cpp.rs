use std::io;

pub fn beautiful_splits(nums: &[i32]) -> i32 {
    let n = nums.len();
    let mut lcp = vec![vec![0; n + 1]; n + 1];

    // Fill the LCP table
    for i in (0..n).rev() {
        for j in (i..n).rev() {
            if nums[i] == nums[j] {
                lcp[i][j] = lcp[i + 1][j + 1] + 1;
            }
        }
    }

    let mut ans = 0;
    // Iterate over possible splits
    for i in 1..n - 1 {
        for j in (i + 1)..n {
            let condition1 = i <= (j - i);
            let condition2 = lcp[0][i] >= i as i32;
            let condition3 = lcp[i][j] >= (j - i) as i32;
            if (condition1 && condition2) || condition3 {
                ans += 1;
            }
        }
    }
    ans
}

fn main() {
    let stdin = io::stdin();
    let mut tokens = Vec::new();

    // Read all tokens from stdin
    for line in stdin.lock().lines() {
        let line = line.expect("Failed to read line");
        tokens.extend(line.split_whitespace().map(|s| s.to_string()));
    }

    // Parse input
    let n: usize = tokens[0].parse().expect("First token must be a number");
    let nums: Vec<i32> = tokens[1..=n]
        .iter()
        .map(|s| s.parse().expect("Invalid number"))
        .collect();

    // Compute and print result
    let result = beautiful_splits(&nums);
    println!("{}", result);
}