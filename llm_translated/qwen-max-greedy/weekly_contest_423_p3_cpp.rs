use std::io::{self, BufRead, Write};

const MOD: i64 = 1_000_000_007;

struct Solution;

impl Solution {
    fn sum_of_good_subsequences(nums: &Vec<i32>) -> i64 {
        let mut cnt: std::collections::HashMap<i32, i64> = std::collections::HashMap::new();
        let mut sum: std::collections::HashMap<i32, i64> = std::collections::HashMap::new();

        for &i in nums.iter() {
            // Update cnt[i] by considering subsequences from i-1, i, and i+1
            *cnt.entry(i).or_insert(0) += 
                (cnt.get(&(i - 1)).unwrap_or(&0) + cnt.get(&(i + 1)).unwrap_or(&0) + 1) % MOD;
            *cnt.entry(i).or_insert(0) %= MOD;

            // Update sum[i] by considering subsequences from i-1, i, and i+1
            *sum.entry(i).or_insert(0) += 
                (sum.get(&(i - 1)).unwrap_or(&0) + sum.get(&(i + 1)).unwrap_or(&0)) % MOD;
            *sum.entry(i).or_insert(0) %= MOD;

            // Add the contribution of the subsequences that end at i
            *sum.entry(i).or_insert(0) += 
                ((cnt.get(&(i - 1)).unwrap_or(&0) + cnt.get(&(i + 1)).unwrap_or(&0) + 1) % MOD * i as i64) % MOD;
            *sum.entry(i).or_insert(0) %= MOD;
        }

        // Calculate the final result by summing all the values in sum
        let mut res: i64 = 0;
        for (_, &value) in sum.iter() {
            res += value;
            res %= MOD;
        }

        res
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut stdout = io::stdout();
    let mut stdout_lock = stdout.lock();

    // Reading input
    let mut n = String::new();
    stdin_lock.read_line(&mut n).expect("Failed to read line");
    let n: usize = n.trim().parse().expect("Please type a number!");

    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        let mut num = String::new();
        stdin_lock.read_line(&mut num).expect("Failed to read line");
        nums.push(num.trim().parse::<i32>().expect("Please type a number!"));
    }

    // Create an object of Solution
    let solution = Solution;

    // Call the method and get the result
    let result = solution.sum_of_good_subsequences(&nums);

    // Output the result
    writeln!(stdout_lock, "{}", result).expect("Failed to write line");
}