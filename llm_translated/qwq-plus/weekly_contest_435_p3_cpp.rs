use std::io;

fn gcd(a: i64, b: i64) -> i64 {
    let mut a = a.abs();
    let mut b = b.abs();
    while b != 0 {
        let t = b;
        b = a % t;
        a = t;
    }
    a
}

struct Solution;

impl Solution {
    fn minimum_increments(nums: Vec<i32>, target: Vec<i32>) -> i64 {
        let m = target.len();
        let size = 1 << m;

        // Precompute LCM for all subsets
        let mut g = vec![1; size];
        for i in 0..size {
            let mut current = 1;
            for j in 0..m {
                if (i & (1 << j)) != 0 {
                    let t = target[j] as i64;
                    let divisor = gcd(current, t);
                    current = (current / divisor) * t;
                }
            }
            g[i] = current;
        }

        const INF: i64 = 1_000_000_000_000_000_000;
        let mut f_prev = vec![INF; size];
        f_prev[0] = 0;

        for i in 1..=nums.len() {
            let mut f_curr = f_prev.clone();
            for j in 0..size {
                let mut k = j;
                while k > 0 {
                    let prev_mask = j ^ k;
                    let g_k = g[k];
                    let num = nums[i - 1] as i64;
                    let required = (num + g_k - 1) / g_k * g_k;
                    let v = required - num;
                    if f_prev[prev_mask] + v < f_curr[j] {
                        f_curr[j] = f_prev[prev_mask] + v;
                    }
                    k = (k - 1) & j;
                }
            }
            f_prev = f_curr;
        }

        f_prev[size - 1]
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let n = tokens[0] as usize;
    let m = tokens[1] as usize;
    let nums = tokens[2..2 + n].to_vec();
    let target = tokens[2 + n..2 + n + m].to_vec();

    let solution = Solution;
    let result = solution.minimum_increments(nums, target);
    println!("{}", result);
}