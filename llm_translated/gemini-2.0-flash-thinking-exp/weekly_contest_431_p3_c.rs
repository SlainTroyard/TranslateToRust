use std::io;
use std::cmp::max;

fn maximum_coins(coins: &mut Vec<Vec<i32>>, k: i32) -> i64 {
    // Sort coins by the left boundary (coins[i][0])
    coins.sort_by_key(|coin| coin[0]);

    let coins_size = coins.len();
    // Calculate prefix sum of the coins' values
    let mut presum: Vec<i64> = vec![0; coins_size + 1];
    for i in 1..=coins_size {
        presum[i] = presum[i - 1] + (coins[i - 1][1] as i64 - coins[i - 1][0] as i64 + 1) * coins[i - 1][2] as i64;
    }

    let mut ans: i64 = 0;
    let mut left: usize = 0;
    let mut right: usize = 0;

    // First pass: moving right pointer forward
    while right < coins_size && left < coins_size {
        while left < coins_size && coins[right][1] - coins[left][0] + 1 > k {
            let tamp = k as i64 - (coins[right][0] as i64 - coins[left][0] as i64);
            ans = max(ans, tamp * coins[right][2] as i64 + presum[right] - presum[left]);
            left += 1;
        }
        if left >= coins_size {
            break;
        }
        ans = max(ans, presum[right + 1] - presum[left]);
        right += 1;
    }

    // Second pass: moving left pointer backward
    left = coins_size - 1;
    right = coins_size - 1;
    while right >= 0 && left >= 0 {
        while right >= 0 && coins[right][1] - coins[left][0] + 1 > k {
            let tamp = k as i64 - (coins[right][1] as i64 - coins[left][1] as i64);
            ans = max(ans, tamp * coins[left][2] as i64 + presum[right + 1] - presum[left + 1]);
            right = if right > 0 { right - 1 } else { break; }; // prevent underflow if right is 0
        }
        if right < 0 {
            break;
        }
        ans = max(ans, presum[right + 1] - presum[left]);
        left = if left > 0 { left - 1 } else { break; }; // prevent underflow if left is 0
    }

    ans
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.trim().split_whitespace();
    let n: i32 = iter.next().unwrap().parse().expect("Failed to parse n");
    let k: i32 = iter.next().unwrap().parse().expect("Failed to parse k");

    let mut coins: Vec<Vec<i32>> = Vec::new();
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let coin_vals: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Failed to parse coin value"))
            .collect();
        coins.push(coin_vals);
    }

    println!("{}", maximum_coins(&mut coins, k));
}