use std::cmp::max;
use std::io::{self, BufRead};

/// Translates the logic of the original C code into Rust.
/// Maintains the same algorithm and input/output format.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read n and k
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line)?;
    let mut iter = line.split_whitespace();
    let n: usize = iter.next().ok_or("Missing n")?.parse()?;
    let k: i32 = iter.next().ok_or("Missing k")?.parse()?;

    // Read coins data
    // Each coin is: [left, right, value]
    let mut coins = Vec::with_capacity(n);
    for _ in 0..n {
        let mut line = String::new();
        stdin.lock().read_line(&mut line)?;
        let mut iter = line.split_whitespace();
        let left = iter.next().ok_or("Missing left boundary")?.parse::<i32>()?;
        let right = iter.next().ok_or("Missing right boundary")?.parse::<i32>()?;
        let val = iter.next().ok_or("Missing coin value")?.parse::<i32>()?;
        coins.push([left, right, val]);
    }

    // Compute the answer
    let ans = maximum_coins(&mut coins, k);

    // Print the result (same format as the original C code: %lld\n)
    println!("{}", ans);
    Ok(())
}

/// Sorts the coins by their left boundary, calculates prefix sums, and performs
/// two passes (moving pointers forward/backward) to determine the maximum coins.
fn maximum_coins(coins: &mut [[i32; 3]], k: i32) -> i64 {
    // Sort by left boundary
    coins.sort_by_key(|coin| coin[0]);

    let coins_size = coins.len();
    let mut presum = vec![0i64; coins_size + 1];
    
    // Compute prefix sums of the coin "value" ( (right-left+1)*val )
    for i in 1..=coins_size {
        let left = coins[i - 1][0];
        let right = coins[i - 1][1];
        let val = coins[i - 1][2];
        presum[i] = presum[i - 1]
            + (right as i64 - left as i64 + 1) * (val as i64);
    }

    let mut ans = 0i64;

    // First pass: moving right pointer forward
    let mut left = 0usize;
    let mut right = 0usize;
    while right < coins_size && left < coins_size {
        // If current window exceeds k in length, trim from the left
        while left < coins_size && (coins[right][1] - coins[left][0] + 1) > k {
            // Compute partial overlap scenario
            let tamp = (k as i64) - (coins[right][0] as i64 - coins[left][0] as i64);
            ans = max(
                ans,
                tamp * (coins[right][2] as i64) + presum[right] - presum[left],
            );
            left += 1;
        }
        if left >= coins_size {
            break;
        }
        ans = max(ans, presum[right + 1] - presum[left]);
        right += 1;
    }

    // Second pass: moving left pointer backward
    if coins_size > 0 {
        let mut left = coins_size as i32 - 1;
        let mut right = coins_size as i32 - 1;
        while right >= 0 && left >= 0 {
            // If current window exceeds k in length, trim from the right
            while right >= 0
                && (coins[right as usize][1] - coins[left as usize][0] + 1) > k
            {
                // Compute partial overlap scenario
                let tamp = (k as i64)
                    - ((coins[right as usize][1] as i64)
                        - (coins[left as usize][1] as i64));
                ans = max(
                    ans,
                    tamp * (coins[left as usize][2] as i64)
                        + presum[(right + 1) as usize]
                        - presum[(left + 1) as usize],
                );
                right -= 1;
                if right < 0 {
                    break;
                }
            }
            if right < 0 {
                break;
            }
            ans = max(
                ans,
                presum[(right + 1) as usize] - presum[left as usize],
            );
            left -= 1;
        }
    }

    ans
}