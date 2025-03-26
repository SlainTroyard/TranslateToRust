use std::cmp;
use std::io;

// Helper function to compare coins for sorting
fn compare(a: &Vec<i32>, b: &Vec<i32>) -> i32 {
    a[0].cmp(&b[0]).into()
}

// Function to find the maximum value between two 64-bit integers
fn max_ll(a: i64, b: i64) -> i64 {
    if a > b { a } else { b }
}

// Main solution function
fn maximum_coins(coins: &mut Vec<Vec<i32>>, k: i32) -> i64 {
    // Sort coins by the left boundary (coins[i][0])
    coins.sort_by(|a, b| compare(a, b).cmp(&compare(b, a)));
    
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
        while left < coins.len() && coins[right][1] - coins[left][0] + 1 > k {
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
        while right >= 0 && coins[right][1] - coins[left][0] + 1 > k {
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
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().expect("Failed to parse n");
    let k: i32 = iter.next().unwrap().parse().expect("Failed to parse k");
    
    let mut coins: Vec<Vec<i32>> = Vec::with_capacity(n);
    for _ in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let coin: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().expect("Failed to parse coin"))
            .collect();
        coins.push(coin);
    }
    
    let result = maximum_coins(&mut coins, k);
    println!("{}", result);
}