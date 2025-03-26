use std::cmp::max;
use std::io;
use std::io::Read;

fn maximum_coins(coins: &mut Vec<Vec<i32>>, k: i32) -> i64 {
    // Sort coins by the left boundary (coins[i][0])
    coins.sort_by(|a, b| a[0].cmp(&b[0]));

    // Calculate prefix sum of the coins' values
    let mut presum: Vec<i64> = vec![0; coins.len() + 1];
    for i in 1..=coins.len() {
        presum[i] = presum[i - 1] + (coins[i - 1][1] as i64 - coins[i - 1][0] as i64 + 1) * coins[i - 1][2] as i64;
    }

    let mut ans: i64 = 0;
    let mut left: usize = 0;
    let mut right: usize = 0;

    // First pass: moving right pointer forward
    while right < coins.len() && left < coins.len() {
        while left < coins.len() && coins[right][1] as i64 - coins[left][0] as i64 + 1 > k as i64 {
            let tamp: i64 = k as i64 - (coins[right][0] as i64 - coins[left][0] as i64);
            ans = max(ans, tamp * coins[right][2] as i64 + presum[right] - presum[left]);
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
        while right >= 0 && coins[right][1] as i64 - coins[left][0] as i64 + 1 > k as i64 {
            let tamp: i64 = k as i64 - (coins[right][1] as i64 - coins[left][1] as i64);
            ans = max(ans, tamp * coins[left][2] as i64 + presum[right + 1] - presum[left + 1]);
            if right == 0 {
                break;
            }
            right -= 1;
        }
        if right < 0 {
            break;
        }
        ans = max(ans, presum[right + 1] - presum[left]);
        if left == 0{
            break;
        }
        left -= 1;
    }

    ans
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let first_line = lines.next().unwrap();
    let mut first_line_iter = first_line.split_whitespace();
    let n: usize = first_line_iter.next().unwrap().parse()?;
    let k: i32 = first_line_iter.next().unwrap().parse()?;

    let mut coins: Vec<Vec<i32>> = Vec::new();
    for _ in 0..n {
        let line = lines.next().unwrap();
        let mut iter = line.split_whitespace();
        let a: i32 = iter.next().unwrap().parse()?;
        let b: i32 = iter.next().unwrap().parse()?;
        let c: i32 = iter.next().unwrap().parse()?;
        coins.push(vec![a, b, c]);
    }

    println!("{}", maximum_coins(&mut coins, k));

    Ok(())
}