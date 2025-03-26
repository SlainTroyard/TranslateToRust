use std::io::{self, Read};

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
    if m == 0 {
        return 0;
    }

    let target: Vec<i64> = target.iter().map(|&x| x as i64).collect();
    let mut g = vec![1i64; 1 << m];
    for i in 0..(1 << m) {
        for j in 0..m {
            if (i >> j) & 1 != 0 {
                let t = target[j];
                let current_g = g[i];
                let gcd_val = gcd(current_g, t);
                g[i] = (current_g / gcd_val) * t;
            }
        }
    }

    const INF: i64 = 1_000_000_000_000_000_000;
    let mut prev = vec![INF; 1 << m];
    prev[0] = 0;
    let mut curr = vec![INF; 1 << m];

    for i in 1..=n {
        curr.copy_from_slice(&prev);
        let num = nums[i - 1] as i64;

        for j in 0..(1 << m) {
            let mut k = j;
            while k > 0 {
                let prev_mask = j ^ k;
                let g_k = g[k];
                let required = (num + g_k - 1) / g_k * g_k;
                let cost = required - num;

                if prev[prev_mask] != INF {
                    let total = prev[prev_mask] + cost;
                    if total < curr[j] {
                        curr[j] = total;
                    }
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
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut tokens = input.split_whitespace().map(|s| s.parse::<i32>().expect("Invalid input token"));

    let n = match tokens.next() {
        Some(val) => val,
        None => panic!("Missing n"),
    };
    let m = match tokens.next() {
        Some(val) => val,
        None => panic!("Missing m"),
    };

    let nums: Vec<i32> = (0..n).map(|_| tokens.next().expect("Missing nums element")).collect();
    let target: Vec<i32> = (0..m).map(|_| tokens.next().expect("Missing target element")).collect();

    if tokens.next().is_some() {
        panic!("Extra input tokens");
    }

    let result = minimum_increments(&nums, &target);
    println!("{}", result);
}