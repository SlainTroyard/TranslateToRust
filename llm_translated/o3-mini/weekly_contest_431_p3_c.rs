use std::io::{self, BufRead};

fn maximum_coins(coins: &mut Vec<[i32; 3]>, k: i64) -> i64 {
    // Sort coins by the left boundary (coins[i][0])
    coins.sort_unstable_by_key(|coin| coin[0]);

    let n = coins.len();
    // Build prefix sum array.
    // presum[i] stores the sum for coins[0..i].
    let mut presum = vec![0_i64; n + 1];
    for i in 1..=n {
        // For coin i-1, the contribution is:
        // (right - left + 1) * coin_value.
        // Use i64 arithmetic.
        let coin = coins[i - 1];
        let segment_value = (coin[1] as i64 - coin[0] as i64 + 1) * coin[2] as i64;
        presum[i] = presum[i - 1] + segment_value;
    }

    // Start with answer = 0.
    let mut ans = 0_i64;

    // First pass: move the right pointer forward.
    let mut left = 0;
    let mut right = 0;
    while right < n && left < n {
        // While the current window exceeds k,
        // move left pointer.
        while left < n && (coins[right][1] as i64 - coins[left][0] as i64 + 1) > k {
            // Calculate available extra coins from the right coin.
            let tamp = k - (coins[right][0] as i64 - coins[left][0] as i64);
            // Update answer with using coin[right][2] on tamp extra coins plus full coins in between.
            ans = ans.max(tamp * coins[right][2] as i64 + (presum[right] - presum[left]));
            left += 1;
        }
        if left >= n {
            break;
        }
        // When within window, consider the sum of coins from left to right inclusive.
        ans = ans.max(presum[right + 1] - presum[left]);
        right += 1;
    }

    // Second pass: move the left pointer backward.
    // Here we use isize indices for safe decrement.
    let mut left = n as isize - 1;
    let mut right = n as isize - 1;
    while right >= 0 && left >= 0 {
        // While window is too wide, narrow it by moving the right pointer leftwards.
        while right >= 0 && (coins[right as usize][1] as i64 - coins[left as usize][0] as i64 + 1) > k {
            // Compute extra coins available from the left coin.
            let tamp = k - (coins[right as usize][1] as i64 - coins[left as usize][1] as i64);
            // Update answer using coin[left][2] for the extra portion plus the full sum between [right+1, left+1)
            ans = ans.max(tamp * coins[left as usize][2] as i64 + (presum[right as usize + 1] - presum[left as usize + 1]));
            right -= 1;
        }
        if right < 0 {
            break;
        }
        // Consider the full window sum.
        ans = ans.max(presum[right as usize + 1] - presum[left as usize]);
        left -= 1;
    }

    ans
}

fn main() {
    // Use standard input and output
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut input = String::new();

    // Read the entire input, then split_whitespace for tokens.
    reader.read_to_string(&mut input).expect("Failed to read input");
    let mut tokens = input.split_whitespace();

    // Parse the number of coins n and k.
    let n: usize = tokens
        .next()
        .expect("Expected n")
        .parse()
        .expect("n is not a number");
    let k: i64 = tokens
        .next()
        .expect("Expected k")
        .parse()
        .expect("k is not a number");

    // Read the coin data. Each coin consists of three integers.
    let mut coins = Vec::with_capacity(n);
    for _ in 0..n {
        let left: i32 = tokens
            .next()
            .expect("Expected left boundary")
            .parse()
            .expect("left is not a number");
        let right: i32 = tokens
            .next()
            .expect("Expected right boundary")
            .parse()
            .expect("right is not a number");
        let value: i32 = tokens
            .next()
            .expect("Expected coin value")
            .parse()
            .expect("value is not a number");
        coins.push([left, right, value]);
    }

    // Call solution function and write answer to stdout.
    let ans = maximum_coins(&mut coins, k);
    println!("{}", ans);
}