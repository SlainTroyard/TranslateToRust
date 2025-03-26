use std::io::{self, BufRead, Write};

const MOD: i64 = 1_000_000_007;

struct Solution;

impl Solution {
    fn sum_of_good_subsequences(nums: &[i32]) -> i64 {
        let mut cnt = std::collections::HashMap::new();
        let mut sum = std::collections::HashMap::new();

        for &i in nums {
            // Update cnt[i] by considering subsequences from i-1, i, and i+1
            *cnt.entry(i).or_insert(0) += (cnt.get(&(i - 1)).unwrap_or(&0)
                + cnt.get(&(i + 1)).unwrap_or(&0)
                + 1) % MOD;
            *cnt.get_mut(&i).unwrap() %= MOD;

            // Update sum[i] by considering subsequences from i-1, i, and i+1
            *sum.entry(i).or_insert(0) += (sum.get(&(i - 1)).unwrap_or(&0)
                + sum.get(&(i + 1)).unwrap_or(&0)) % MOD;
            *sum.get_mut(&i).unwrap() %= MOD;

            // Add the contribution of the subsequences that end at i
            *sum.get_mut(&i).unwrap() += ((cnt.get(&(i - 1)).unwrap_or(&0)
                + cnt.get(&(i + 1)).unwrap_or(&0)
                + 1) % MOD * i as i64) % MOD;
            *sum.get_mut(&i).unwrap() %= MOD;
        }

        // Calculate the final result by summing all the values in sum
        let mut res = 0;
        for (_, &value) in &sum {
            res += value;
            res %= MOD;
        }

        res
    }
}

fn main() {
    // Reading input
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut stdin_lock = stdin.lock();
    let mut buffer = String::new();

    stdin_lock.read_line(&mut buffer).expect("Failed to read line");
    let n: usize = buffer.trim().parse().expect("Failed to parse n");

    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        buffer.clear();
        stdin_lock.read_line(&mut buffer).expect("Failed to read line");
        nums.push(buffer.trim().parse().expect("Failed to parse num"));
    }

    // Create an object of Solution
    let solution = Solution;

    // Call the method and get the result
    let result = solution.sum_of_good_subsequences(&nums);

    // Output the result
    writeln!(stdout, "{}", result).expect("Failed to write result");
}