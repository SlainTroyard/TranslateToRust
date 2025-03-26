use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut tokens = input.split_whitespace();
    
    let n: usize = tokens.next().expect("Missing n").parse().expect("Invalid n");
    let k: i32 = tokens.next().expect("Missing k").parse().expect("Invalid k");
    
    let mut coins = Vec::with_capacity(n);
    for _ in 0..n {
        let left = tokens.next().expect("Missing left").parse().expect("Invalid left");
        let right = tokens.next().expect("Missing right").parse().expect("Invalid right");
        let value = tokens.next().expect("Missing value").parse().expect("Invalid value");
        coins.push((left, right, value));
    }
    
    coins.sort_by_key(|c| c.0);
    
    let result = maximum_coins(&coins, k);
    println!("{}", result);
}

fn maximum_coins(coins: &[(i32, i32, i32)], k: i32) -> i64 {
    let n = coins.len();
    let mut presum = vec![0i64; n + 1];
    for i in 1..=n {
        let (left, right, value) = coins[i - 1];
        let contribution = (right - left + 1) as i64 * value as i64;
        presum[i] = presum[i - 1] + contribution;
    }
    
    let mut ans = 0i64;
    
    // First pass: move right pointer forward
    let mut left = 0;
    let mut right = 0;
    while right < n && left < n {
        while left < n {
            let current_right = &coins[right];
            let window_size = current_right.1 - coins[left].0 + 1;
            if window_size <= k {
                break;
            }
            let tamp = (k - (current_right.0 - coins[left].0)) as i64;
            let sum_part = presum[right] - presum[left];
            ans = ans.max(tamp * current_right.2 as i64 + sum_part);
            left += 1;
        }
        if left >= n {
            break;
        }
        let current_sum = presum[right + 1] - presum[left];
        ans = ans.max(current_sum);
        right += 1;
    }
    
    // Second pass: move left pointer backward
    let mut left = n as isize - 1;
    let mut right = n as isize - 1;
    while right >= 0 && left >= 0 {
        while right >= 0 {
            let cr = coins[right as usize];
            let cl = coins[left as usize];
            if cr.1 - cl.0 + 1 <= k {
                break;
            }
            let tamp = (k - (cr.1 - cl.1)) as i64;
            let sum_part = presum[(right + 1) as usize] - presum[(left + 1) as usize];
            ans = ans.max(tamp * cl.2 as i64 + sum_part);
            right -= 1;
        }
        if right < 0 {
            break;
        }
        let current_sum = presum[(right + 1) as usize] - presum[left as usize];
        ans = ans.max(current_sum);
        left -= 1;
    }
    
    ans
}