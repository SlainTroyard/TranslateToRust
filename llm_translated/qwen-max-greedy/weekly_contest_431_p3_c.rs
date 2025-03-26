use std::io::{self, BufRead, Write};

// Helper function to compare coins for sorting
fn compare(a: &[i32], b: &[i32]) -> i32 {
    a[0].cmp(&b[0]).into()
}

// Function to find the maximum value between two i64 values
fn max_ll(a: i64, b: i64) -> i64 {
    if a > b { a } else { b }
}

// Main solution function
fn maximum_coins(coins: &mut [Vec<i32>], k: i32) -> i64 {
    // Sort coins by the left boundary (coins[i][0])
    coins.sort_by(|a, b| a[0].cmp(&b[0]));

    // Calculate prefix sum of the coins' values
    let mut presum = vec![0; coins.len() + 1];
    for i in 1..=coins.len() {
        presum[i] = presum[i - 1] + (coins[i - 1][1] - coins[i - 1][0] + 1) as i64 * coins[i - 1][2] as i64;
    }

    let mut ans = 0;
    let mut left = 0;
    let mut right = 0;

    // First pass: moving right pointer forward
    while right < coins.len() && left < coins.len() {
        while left < coins.len() && coins[right][1] - coins[left][0] + 1 > k as i32 {
            let tamp = k - (coins[right][0] - coins[left][0]);
            ans = max_ll(ans, tamp as i64 * coins[right][2] as i64 + presum[right] - presum[left]);
            left += 1;
        }
        if left >= coins.len() { break; }
        ans = max_ll(ans, presum[right + 1] - presum[left]);
        right += 1;
    }

    // Second pass: moving left pointer backward
    left = coins.len() - 1;
    right = coins.len() - 1;
    while right >= 0 && left >= 0 {
        while right >= 0 && coins[right][1] - coins[left][0] + 1 > k as i32 {
            let tamp = k - (coins[right][1] - coins[left][1]);
            ans = max_ll(ans, tamp as i64 * coins[left][2] as i64 + presum[right + 1] - presum[left + 1]);
            right -= 1;
        }
        if right < 0 { break; }
        ans = max_ll(ans, presum[right + 1] - presum[left]);
        left -= 1;
    }

    ans
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    // Read n and k from stdin
    let mut input = String::new();
    stdin.lock().read_line(&mut input).unwrap();
    let parts: Vec<i32> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    let n = parts[0];
    let k = parts[1];

    // Allocate and initialize the coins array
    let mut coins = vec![vec![0; 3]; n as usize];
    for i in 0..n {
        input.clear();
        stdin.lock().read_line(&mut input).unwrap();
        coins[i as usize] = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    }

    // Call the solution function
    let result = maximum_coins(&mut coins, k);

    // Print the result
    writeln!(stdout, "{}", result).unwrap();
}