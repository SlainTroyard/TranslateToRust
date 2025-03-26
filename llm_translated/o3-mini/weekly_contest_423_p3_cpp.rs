use std::collections::HashMap;
use std::io::{self, BufRead};

const MOD: i64 = 1_000_000_007;

// Helper function to safely get a value from a HashMap, returns 0 if the key is not present.
fn get(map: &HashMap<i32, i64>, key: i32) -> i64 {
    *map.get(&key).unwrap_or(&0)
}

// This function calculates the sum of good subsequences as described in the problem.
fn sum_of_good_subsequences(nums: &[i32]) -> i32 {
    // We use two HashMaps to store counts and weighted sums.
    let mut cnt: HashMap<i32, i64> = HashMap::new();
    let mut sum: HashMap<i32, i64> = HashMap::new();
    
    // Loop through all numbers in the nums array.
    for &i in nums {
        // Calculate the additional subsequences count ending at i.
        // It uses counts for i-1 and i+1 and also creates a new subsequence with the current number.
        let add_cnt = (get(&cnt, i - 1) + get(&cnt, i + 1) + 1) % MOD;
        // Update cnt[i] with the additional count (and take mod).
        let entry = cnt.entry(i).or_insert(0);
        *entry = (*entry + add_cnt) % MOD;

        // Calculate the additional sum for subsequences ending at i.
        let add_sum_base = (get(&sum, i - 1) + get(&sum, i + 1)) % MOD;
        // Add the contribution from the current number multiplied by the count (from i-1 and i+1 and new subsequence).
        let add_sum_contrib = (add_cnt * i as i64) % MOD;
        let add_sum = (add_sum_base + add_sum_contrib) % MOD;
        // Update sum[i] accordingly (and take mod).
        let entry_sum = sum.entry(i).or_insert(0);
        *entry_sum = (*entry_sum + add_sum) % MOD;
    }
    
    // Calculate the final result by summing all the values stored in sum.
    let mut res = 0;
    for value in sum.values() {
        res = (res + value) % MOD;
    }
    
    res as i32
}

fn main() {
    // Set up input reading from standard input.
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read n, the first integer from input.
    let n: usize = match lines.next() {
        Some(Ok(line)) => line.trim().parse().expect("Invalid input for n"),
        _ => {
            eprintln!("No input provided for n.");
            return;
        }
    };
    
    // This vector will hold the numbers.
    let mut nums: Vec<i32> = Vec::with_capacity(n);

    // Since the input might be in multiple lines or multiple numbers per line, we iterate until we have all n numbers.
    while nums.len() < n {
        if let Some(Ok(line)) = lines.next() {
            // Split the line on whitespace and parse each token as an integer.
            for token in line.trim().split_whitespace() {
                if nums.len() >= n {
                    break;
                }
                let num = token.parse::<i32>().expect("Invalid input number");
                nums.push(num);
            }
        } else {
            break;
        }
    }
    
    // Call the solution function and get the result.
    let result = sum_of_good_subsequences(&nums);
    
    // Output the result (exactly the same as the original C++ output format).
    println!("{}", result);
}