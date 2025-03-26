use std::collections::HashMap;
use std::io::{self, Read, Write};

/// Computes and returns the largest outlier number from the list based on the logic:
/// For each candidate number `n` (in descending order), if there exists another number `d`
/// such that (total_sum - n) == 2 * d (with a special check when n and d are the same),
/// then n is the outlier. If no such candidate exists, returns -1.
fn get_largest_outlier(nums: &[i32]) -> i32 {
    let mut ctr = HashMap::new();
    let mut sm = 0;

    // Count frequency of each number and compute the total sum.
    for &num in nums {
        *ctr.entry(num).or_insert(0) += 1;
        sm += num;
    }

    // Collect candidate numbers (keys from the map) into a vector.
    let mut cands: Vec<_> = ctr.keys().copied().collect();
    // Sort in descending order.
    cands.sort_unstable_by(|a, b| b.cmp(a));

    // Iterate over candidates to check the desired condition.
    for n in cands {
        let diff = sm - n;
        // Check if diff is even so that it can be split into two equal integers.
        if diff % 2 == 0 {
            let d = diff / 2;
            if let Some(&count) = ctr.get(&d) {
                // When d equals n, we need at least two occurrences.
                if d != n || count > 1 {
                    return n;
                }
            }
        }
    }
    -1
}

fn main() -> io::Result<()> {
    // Read the entire input from stdin.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Split input tokens by whitespace.
    let mut tokens = input.split_whitespace();

    // The first token is the number of elements.
    let n: usize = tokens
        .next()
        .expect("Expected a number")
        .parse()
        .expect("Failed to parse number");

    // Read the next n tokens as numbers.
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        let token = tokens.next().expect("Expected more numbers");
        let num: i32 = token.parse().expect("Failed to parse number");
        nums.push(num);
    }

    // Compute the result using the helper function.
    let result = get_largest_outlier(&nums);

    // Output the result.
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    writeln!(handle, "{}", result)?;

    Ok(())
}