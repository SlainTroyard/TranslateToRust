use std::io;

fn gcd(a: i64, b: i64) -> i64 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn minimum_increments(nums: &[i32], target: &[i32]) -> i32 {
    let n = nums.len();
    let m = target.len();
    let size = 1 << m;

    // Compute the g array for all subsets
    let mut g = vec![1; size];
    for i in 0..size {
        let mut lcm = 1;
        for j in 0..m {
            if (i >> j) & 1 != 0 {
                let t = target[j] as i64;
                let current_gcd = gcd(lcm, t);
                lcm = (lcm / current_gcd) * t;
            }
        }
        g[i] = lcm;
    }

    // Initialize DP arrays
    const INF: i64 = 1e18 as i64;
    let mut prev = vec![INF; size];
    prev[0] = 0;
    let mut curr = vec![INF; size];

    for &num in nums {
        curr.clone_from(&prev);
        for j in 0..size {
            let mut k = j;
            while k > 0 {
                let v = ((num as i64 + g[k] - 1) / g[k]) * g[k] - (num as i64);
                let prev_state = j ^ k;
                if prev[prev_state] + v < curr[j] {
                    curr[j] = prev[prev_state] + v;
                }
                k = (k - 1) & j;
            }
        }
        std::mem::swap(&mut prev, &mut curr);
    }

    prev[(1 << m) - 1] as i32
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut parts = input.split_whitespace();
    let n = parts.next().and_then(|s| s.parse().ok()).expect("Invalid input");
    let m = parts.next().and_then(|s| s.parse().ok()).expect("Invalid input");

    let mut nums = Vec::with_capacity(n);
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    for s in input.split_whitespace().take(n) {
        nums.push(s.parse().expect("Invalid input"));
    }

    let mut target = Vec::with_capacity(m);
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    for s in input.split_whitespace().take(m) {
        target.push(s.parse().expect("Invalid input"));
    }

    let result = minimum_increments(&nums, &target);
    println!("{}", result);
}