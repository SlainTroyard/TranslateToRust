use std::cmp::max;
use std::io::{self, BufRead};

// Helper function to compare coins for sorting
fn compare(a: &Vec<i32>, b: &Vec<i32>) -> std::cmp::Ordering {
    a[0].cmp(&b[0]) // Compare by left boundary
}

// Main solution function
fn maximum_coins(coins: &mut Vec<Vec<i32>>, k: i32) -> i64 {
    // Sort coins by the left boundary (coins[i][0])
    coins.sort_by(compare);
    
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
        while left < coins.len() && (coins[right][1] - coins[left][0] + 1) > k {
            let tamp = k - (coins[right][0] - coins[left][0]);
            ans = max(ans, tamp as i64 * coins[right][2] as i64 + presum[right] - presum[left]);
            left += 1;
        }
        if left >= coins.len() {
            break;
        }
        ans = max(ans, presum[right + 1] - presum[left]);
        right += 1;
    }
    
    // Second pass: moving left pointer backward
    left = coins.len() - 1;
    right = coins.len() - 1;
    while right >= 0 && left >= 0 {
        while right >= 0 && (coins[right][1] - coins[left][0] + 1) > k {
            let tamp = k - (coins[right][1] - coins[left][1]);
            ans = max(ans, tamp as i64 * coins[left][2] as i64 + presum[right + 1] - presum[left + 1]);
            right -= 1;
        }
        if right < 0 {
            break;
        }
        ans = max(ans, presum[right + 1] - presum[left]);
        left -= 1;
    }
    
    ans
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read n and k
    let first_line = lines.next().unwrap().unwrap();
    let mut iter = first_line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();
    
    // Read coins
    let mut coins = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        let l: i32 = iter.next().unwrap().parse().unwrap();
        let r: i32 = iter.next().unwrap().parse().unwrap();
        let v: i32 = iter.next().unwrap().parse().unwrap();
        coins.push(vec![l, r, v]);
    }
    
    // Call the solution function
    let result = maximum_coins(&mut coins, k);
    println!("{}", result);
}