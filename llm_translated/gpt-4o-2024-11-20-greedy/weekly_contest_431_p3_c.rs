use std::cmp::max;
use std::io::{self, BufRead};

/// Helper function to compare coins for sorting based on their left boundary.
fn compare(a: &Vec<i32>, b: &Vec<i32>) -> std::cmp::Ordering {
    a[0].cmp(&b[0])
}

/// Main solution function to compute the maximum coins collectible within constraints.
fn maximum_coins(coins: &Vec<Vec<i32>>, coins_size: usize, k: i32) -> i64 {
    // Sort coins by the left boundary (coins[i][0])
    let mut coins = coins.clone();
    coins.sort_by(compare);

    // Calculate prefix sum of the coins' values
    let mut presum = vec![0i64; coins_size + 1];
    for i in 1..=coins_size {
        let left = coins[i - 1][0];
        let right = coins[i - 1][1];
        let value = coins[i - 1][2];
        presum[i] = presum[i - 1] + ((right - left + 1) as i64) * (value as i64);
    }

    let mut ans: i64 = 0;

    // First pass: moving the right pointer forward
    let (mut left, mut right) = (0, 0);
    while right < coins_size && left < coins_size {
        while left < coins_size && coins[right][1] - coins[left][0] + 1 > k {
            let tamp = k - (coins[right][0] - coins[left][0]);
            if tamp >= 0 {
                ans = max(
                    ans,
                    (tamp as i64) * (coins[right][2] as i64)
                        + (presum[right] - presum[left]),
                );
            }
            left += 1;
        }
        if left >= coins_size {
            break;
        }
        ans = max(ans, presum[right + 1] - presum[left]);
        right += 1;
    }

    // Second pass: moving the left pointer backward
    let mut left = coins_size - 1;
    right = coins_size - 1;
    while right >= 0 && left >= 0 {
        while right >= 0 && coins[right][1] - coins[left][0] + 1 > k {
            let tamp = k - (coins[right][1] - coins[left][1]);
            if tamp >= 0 {
                ans = max(
                    ans,
                    (tamp as i64) * (coins[left][2] as i64)
                        + (presum[right + 1] - presum[left + 1]),
                );
            }
            if right == 0 {
                break;
            }
            right -= 1;
        }
        if right < 0 {
            break;
        }
        ans = max(ans, presum[right + 1] - presum[left]);
        if left == 0 {
            break;
        }
        left -= 1;
    }

    ans
}

/// Entry point
fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines();

    // Read `n` and `k`
    let first_line = input.next().unwrap().unwrap();
    let mut split = first_line.split_whitespace();
    let n: usize = split.next().unwrap().parse().unwrap();
    let k: i32 = split.next().unwrap().parse().unwrap();

    // Read coins
    let mut coins = Vec::new();
    for _ in 0..n {
        let line = input.next().unwrap().unwrap();
        let mut coin = Vec::new();
        coin.extend(line.split_whitespace().map(|s| s.parse::<i32>().unwrap()));
        coins.push(coin);
    }

    // Solve the problem and print the result
    println!("{}", maximum_coins(&coins, n, k));
}