```rust
use std::io::{self, BufRead};
use std::cmp::max;

// Helper function to parse integers from input
fn read_line_as_ints() -> Vec<i32> {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.read_line(&mut line).expect("Failed to read line");
    line.trim()
        .split_whitespace()
        .map(|s| s.parse::<i32>().expect("Failed to parse integer"))
        .collect()
}

// Helper function to compare coins for sorting
fn compare(a: &[i32], b: &[i32]) -> std::cmp::Ordering {
    a[0].cmp(&b[0]) // Compare by left boundary
}

// Main solution function
fn maximum_coins(coins: &Vec<Vec<i32>>, k: i32) -> i64 {
    // Sort coins by the left boundary (coins[i][0])
    let mut coins = coins.clone();
    coins.sort_by(|a, b| compare(a, b));

    // Calculate prefix sum of the coins' values
    let coins_size = coins.len();
    let mut presum = vec![0i64; coins_size + 1];
    for i in 1..=coins_size {
        let coin = &coins[i - 1];
        let value = (coin[1] - coin[0] + 1) as i64 * coin[2] as i64;
        presum[i] = presum[i - 1] + value;
    }

    let mut ans = 0i64;
    let mut left: usize = 0;
    let mut right: usize = 0;

    // First pass: moving right pointer forward
    while right < coins_size && left < coins_size {
        while left < coins_size && coins[right][1] - coins[left][0] + 1 > k {
            let tamp = k - (coins[right][0] - coins[left][0]);
            let tamp_value = tamp as i64 * coins[right][2] as i64;
            ans = max(
                ans,
                tamp_value + presum[right + 1] - presum[left + 1],
            );
            left += 1;
        }
        if left >= coins_size {
            break;
        }
        ans = max(ans-)