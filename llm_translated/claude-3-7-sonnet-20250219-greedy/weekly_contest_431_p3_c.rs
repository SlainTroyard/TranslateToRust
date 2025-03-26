// Problem: Weekly Contest 431 Problem 3
use std::cmp::Ordering;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read n and k from the first line
    let first_line = lines.next().unwrap()?;
    let mut iter = first_line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();
    
    // Read coins data
    let mut coins: Vec<Vec<i32>> = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap()?;
        let values: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        coins.push(values);
    }
    
    // Create a vector to represent coinsColSize (all 3 in this case)
    let coins_col_size: Vec<i32> = vec![3; n];
    
    // Call the solution function
    let result = maximum_coins(&coins, n, &coins_col_size, k);
    println!("{}", result);
    
    Ok(())
}

// Main solution function
fn maximum_coins(coins: &[Vec<i32>], coins_size: usize, _coins_col_size: &[i32], k: i32) -> i64 {
    // Create a sorted copy of coins by the left boundary (coins[i][0])
    let mut sorted_coins = coins.to_vec();
    sorted_coins.sort_by(|a, b| a[0].cmp(&b[0]));
    
    // Calculate prefix sum of the coins' values
    let mut presum: Vec<i64> = vec![0; coins_size + 1];
    for i in 1..=coins_size {
        let coin = &sorted_coins[i - 1];
        presum[i] = presum[i - 1] + (coin[1] - coin[0] + 1) as i64 * coin[2] as i64;
    }
    
    let mut ans: i64 = 0;
    let mut left = 0;
    let mut right = 0;
    
    // First pass: moving right pointer forward
    while right < coins_size && left < coins_size {
        while left < coins_size && sorted_coins[right][1] - sorted_coins[left][0] + 1 > k {
            let tamp = k as i64 - (sorted_coins[right][0] - sorted_coins[left][0]) as i64;
            ans = ans.max(tamp * sorted_coins[right][2] as i64 + presum[right] - presum[left]);
            left += 1;
        }
        if left >= coins_size {
            break;
        }
        ans = ans.max(presum[right + 1] - presum[left]);
        right += 1;
    }
    
    // Second pass: moving left pointer backward
    left = coins_size - 1;
    right = coins_size - 1;
    while right >= 0 && left >= 0 {
        while right >= 0 && sorted_coins[right][1] - sorted_coins[left][0] + 1 > k {
            let tamp = k as i64 - (sorted_coins[right][1] - sorted_coins[left][1]) as i64;
            ans = ans.max(tamp * sorted_coins[left][2] as i64 + presum[right + 1] - presum[left + 1]);
            if right == 0 {
                break;
            }
            right -= 1;
        }
        if right >= coins_size || right < 0 {
            break;
        }
        ans = ans.max(presum[right + 1] - presum[left]);
        if left == 0 {
            break;
        }
        left -= 1;
    }
    
    ans
}