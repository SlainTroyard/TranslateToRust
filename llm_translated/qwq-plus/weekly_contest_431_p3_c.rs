use std::io;

fn main() {
    let mut lines = io::stdin().lines();

    // Read the first line for n and k
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let k: i32 = parts.next().unwrap().parse().unwrap();

    // Read the coins
    let mut coins = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let a: i32 = parts.next().unwrap().parse().unwrap();
        let b: i32 = parts.next().unwrap().parse().unwrap();
        let c: i32 = parts.next().unwrap().parse().unwrap();
        coins.push([a, b, c]);
    }

    // Sort coins by their left boundary (coins[i][0])
    coins.sort_by_key(|coin| coin[0]);

    // Compute prefix sum
    let mut presum = vec![0; coins.len() + 1];
    for i in 0..coins.len() {
        let coin = &coins[i];
        let val = (coin[1] - coin[0] + 1) as i64 * coin[2] as i64;
        presum[i + 1] = presum[i] + val;
    }

    // First pass: moving right pointer forward
    let mut ans = 0i64;
    let (mut left, mut right) = (0, 0);
    while right < coins.len() && left < coins.len() {
        while left < coins.len()
            && (coins[right][1] - coins[left][0] + 1) as i32 > k
        {
            let tamp = (k as i64)
                - (coins[right][0] as i64 - coins[left][0] as i64);
            let val = tamp * coins[right][2] as i64 + (presum[right] - presum[left]);
            ans = ans.max(val);
            left += 1;
        }
        if left >= coins.len() {
            break;
        }
        ans = ans.max(presum[right + 1] - presum[left]);
        right += 1;
    }

    // Second pass: moving left pointer backward
    left = coins.len() - 1;
    right = coins.len() - 1;
    while right >= 0 && left >= 0 {
        while right >= 0
            && (coins[right][1] - coins[left][0] + 1) as i32 > k
        {
            let tamp = (k as i64)
                - (coins[right][1] as i64 - coins[left][1] as i64);
            let val = tamp * coins[left][2] as i64
                + (presum[right + 1] - presum[left + 1]);
            ans = ans.max(val);
            right -= 1;
        }
        if right < 0 {
            break;
        }
        ans = ans.max(presum[right + 1] - presum[left]);
        left -= 1;
    }

    println!("{}", ans);
}