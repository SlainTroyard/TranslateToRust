use std::io::{self, BufRead};

fn gcd(a: i64, b: i64) -> i64 {
    let mut a = a.abs();
    let mut b = b.abs();
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn minimum_increments(nums: Vec<i64>, target: Vec<i64>) -> i64 {
    let n = nums.len();
    let m = target.len();
    let mask_count = 1 << m;

    // Precompute LCM for all subsets of target
    let mut g = vec![1i64; mask_count];
    for mask in 0..mask_count {
        for j in 0..m {
            if (mask >> j) & 1 != 0 {
                let t = target[j];
                let current = g[mask];
                let gcd_val = gcd(current, t);
                g[mask] = (current / gcd_val) * t;
            }
        }
    }

    const INF: i64 = 1_000_000_000_000_000_000;
    let mut prev = vec![INF; mask_count];
    prev[0] = 0;

    for num in nums {
        let mut curr = prev.clone();

        for j in 0..mask_count {
            let mut k = j;
            while k > 0 {
                let subset = k;
                let lcm = g[subset];
                // Calculate required value and cost
                let required = ((num + lcm - 1) / lcm) * lcm;
                let cost = required - num;

                let prev_mask = j ^ subset;
                if prev[prev_mask] != INF {
                    let new_cost = prev[prev_mask] + cost;
                    if new_cost < curr[j] {
                        curr[j] = new_cost;
                    }
                }

                k = (k - 1) & j;
            }
        }

        prev = curr;
    }

    let full_mask = (1 << m) - 1;
    if prev[full_mask] == INF {
        -1 // According to problem constraints, this case shouldn't happen
    } else {
        prev[full_mask]
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());

    // Read n and m from the first line
    let first_line = lines.next().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let m: usize = parts.next().unwrap().parse().unwrap();

    // Read nums array
    let nums_line = lines.next().unwrap();
    let nums: Vec<i64> = nums_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Read target array
    let target_line = lines.next().unwrap();
    let target: Vec<i64> = target_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let result = minimum_increments(nums, target);
    println!("{}", result);
}