use std::io::{self, BufRead};

struct Solution;

impl Solution {
    // Calculates the number of beautiful splits as per the given algorithm.
    pub fn beautiful_splits(&self, nums: &[i32]) -> i32 {
        let n = nums.len();
        // lcp[i][j] represents the Longest Common Prefix (LCP) between s[i:] and s[j:].
        // We create a 2D vector with dimensions (n+1) x (n+1) initialized to 0.
        let mut lcp = vec![vec![0; n + 1]; n + 1];

        // Calculate LCP in reverse order.
        for i in (0..n).rev() {
            for j in (i..n).rev() {
                if nums[i] == nums[j] {
                    lcp[i][j] = lcp[i + 1][j + 1] + 1;
                }
            }
        }

        let mut ans = 0;
        // Iterate over potential split indices.
        // The first split point i goes from 1 to n-2 (since i < n - 1).
        for i in 1..(n - 1) {
            // The second split point j goes from i+1 to n-1.
            for j in (i + 1)..n {
                // Check if the split satisfies the beautiful condition.
                if (i <= j - i && lcp[0][i] >= i) || (lcp[i][j] >= j - i) {
                    ans += 1;
                }
            }
        }
        ans
    }
}

fn main() -> io::Result<()> {
    // Build a buffered reader for efficient input reading.
    let stdin = io::stdin();
    let reader = stdin.lock();
    
    // Collect all tokens from the standard input.
    let mut tokens = Vec::new();
    for line in reader.lines() {
        let line = line?;
        tokens.extend(line.split_whitespace().map(String::from));
    }
    
    let mut iter = tokens.into_iter();
    
    // First token: size of array, n.
    let n: usize = iter
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Expected an integer for n"))?
        .parse()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    
    // Next n tokens: array elements.
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        let num: i32 = iter
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Not enough numbers"))?
            .parse()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        nums.push(num);
    }
    
    // Calculate and print the result.
    let solution = Solution;
    let result = solution.beautiful_splits(&nums);
    println!("{}", result);
    
    Ok(())
}