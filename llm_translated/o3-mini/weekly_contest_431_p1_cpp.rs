use std::cmp;
use std::io::{self, BufRead, Write};

// Define our solution struct
struct Solution;

impl Solution {
    // This function implements the logic of maxLength
    fn max_length(&self, nums: &[usize]) -> usize {
        let n = nums.len();
        // Find maximum value in nums to size our factor table.
        let m = match nums.iter().max() {
            Some(&max_val) => max_val,
            None => return 0, // Should not happen given input assumptions.
        };

        // Create a vector for factors for each number from 0 to m.
        // We use m+1 because indices go from 0 to m.
        let mut fac: Vec<Vec<usize>> = vec![vec![]; m + 1];
        // Sieve-like approach to fill factors.
        for i in 2..=m {
            // if fac[i] is empty, i is prime (has not been marked with any factor)
            if fac[i].is_empty() {
                let mut j = i;
                while j <= m {
                    fac[j].push(i);
                    j += i;
                }
            }
        }

        // Use sliding window approach.
        let mut ans = 2; // Starting value as in C++ code.
        let mut vis = vec![false; m + 1]; // To track used prime factors.
        let mut j = 0;

        // Iterate with i as the left pointer of sliding window.
        for i in 0..n {
            // Move j as far right as possible while the window is valid.
            while j < n {
                // Check if any prime factor of nums[j] is already used.
                let can_extend = fac[nums[j]].iter().all(|&p| !vis[p]);
                if can_extend {
                    // Mark factors used for nums[j].
                    for &p in &fac[nums[j]] {
                        vis[p] = true;
                    }
                    j += 1;
                } else {
                    break;
                }
            }
            // Update answer with the length of the current window.
            ans = cmp::max(ans, j - i);
            // Remove the effect of nums[i] as we slide the window.
            for &p in &fac[nums[i]] {
                vis[p] = false;
            }
        }
        ans
    }
}

fn main() -> io::Result<()> {
    // Prepare stdin reader and stdout writer.
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let stdout = io::stdout();
    let mut writer = io::BufWriter::new(stdout.lock());

    // Read first line: number of elements.
    let mut line = String::new();
    reader.read_line(&mut line)?;
    let num_size: usize = line.trim().parse().expect("Invalid number format");

    // Read numbers from input
    let mut nums = Vec::with_capacity(num_size);
    // Depending on the input format, numbers could be on the same line or multiple lines.
    // We keep reading until we collect num_size numbers.
    while nums.len() < num_size {
        line.clear();
        reader.read_line(&mut line)?;
        if line.trim().is_empty() {
            continue;
        }
        // Split the line into tokens, parse each token as number.
        for token in line.trim().split_whitespace() {
            let num: usize = token.parse().expect("Invalid integer in input");
            nums.push(num);
            if nums.len() == num_size {
                break;
            }
        }
    }
    
    // Create the solution instance, compute the result and print it.
    let solution = Solution;
    let result = solution.max_length(&nums);
    writeln!(writer, "{}", result)?;
    writer.flush()?;
    Ok(())
}