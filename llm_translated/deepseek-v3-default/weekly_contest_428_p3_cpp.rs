use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the size of the array
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Read the array elements
    let nums: Vec<i32> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Calculate and print the result
    let result = beautiful_splits(nums);
    println!("{}", result);
}

fn beautiful_splits(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    // lcp[i][j] represents the Longest Common Prefix (LCP) between s[i:] and s[j:]
    let mut lcp = vec![vec![0; n + 1]; n + 1];

    // Fill the lcp table
    for i in (0..n).rev() {
        for j in (i..n).rev() {
            if nums[i] == nums[j] {
                lcp[i][j] = lcp[i + 1][j + 1] + 1;
            }
        }
    }

    let mut ans = 0;
    // Iterate over all possible splits
    for i in 1..n - 1 {
        for j in i + 1..n {
            // Check if the split satisfies the beautiful condition
            if (i <= j - i && lcp[0][i] >= i as i32) || lcp[i][j] >= (j - i) as i32 {
                ans += 1;
            }
        }
    }
    ans
}