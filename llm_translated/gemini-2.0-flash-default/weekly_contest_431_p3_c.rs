use std::cmp;
use std::io;
use std::io::Read;

// Helper function to compare coins for sorting
fn compare(a: &[i32], b: &[i32]) -> cmp::Ordering {
    a[0].cmp(&b[0]) // Compare by left boundary
}

// Function to find the maximum value between two long long values
fn max_ll(a: i64, b: i64) -> i64 {
    cmp::max(a, b)
}

// Main solution function
fn maximum_coins(coins: &mut Vec<Vec<i32>>, k: i32) -> i64 {
    // Sort coins by the left boundary (coins[i][0])
    coins.sort_by(|a, b| compare(a, b));

    // Calculate prefix sum of the coins' values
    let mut presum: Vec<i64> = vec![0; coins.len() + 1];
    presum[0] = 0;
    for i in 1..=coins.len() {
        presum[i] = presum[i - 1] + (coins[i - 1][1] as i64 - coins[i - 1][0] as i64 + 1) * coins[i - 1][2] as i64;
    }

    let mut ans: i64 = 0;
    let mut left: usize = 0;
    let mut right: usize = 0;

    // First pass: moving right pointer forward
    while right < coins.len() && left < coins.len() {
        while left < coins.len() && coins[right][1] - coins[left][0] + 1 > k {
            let tamp = k as i64 - (coins[right][0] as i64 - coins[left][0] as i64);
            ans = max_ll(ans, tamp * coins[right][2] as i64 + presum[right] - presum[left]);
            left += 1;
        }
        if left >= coins.len() {
            break;
        }
        ans = max_ll(ans, presum[right + 1] - presum[left]);
        right += 1;
    }

    // Second pass: moving left pointer backward
    left = coins.len() - 1;
    right = coins.len() - 1;
    while right < coins.len() && left < coins.len() {
        while right < coins.len() && right >= 0 && coins[right][1] - coins[left][0] + 1 > k {
            let tamp = k as i64 - (coins[right][1] as i64 - coins[left][1] as i64);
            ans = max_ll(ans, tamp * coins[left][2] as i64 + presum[right + 1] - presum[left + 1]);
            if right == 0 { break; }
            right -= 1;

        }

        if right >= coins.len() || right < 0 {
            break;
        }
        ans = max_ll(ans, presum[right + 1] - presum[left]);
        if left == 0 { break; }
        left -= 1;
    }

    ans
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let first_line = lines.next().unwrap();
    let mut first_split = first_line.split_whitespace();
    let n: usize = first_split.next().unwrap().parse()?;
    let k: i32 = first_split.next().unwrap().parse()?;

    let mut coins: Vec<Vec<i32>> = Vec::with_capacity(n);
    // let mut coins_col_size: Vec<i32> = Vec::with_capacity(n);

    for _ in 0..n {
        let line = lines.next().unwrap();
        let mut split = line.split_whitespace();
        let coin0: i32 = split.next().unwrap().parse()?;
        let coin1: i32 = split.next().unwrap().parse()?;
        let coin2: i32 = split.next().unwrap().parse()?;
        coins.push(vec![coin0, coin1, coin2]);
        // coins_col_size.push(3);
    }

    println!("{}", maximum_coins(&mut coins, k));

    Ok(())
}