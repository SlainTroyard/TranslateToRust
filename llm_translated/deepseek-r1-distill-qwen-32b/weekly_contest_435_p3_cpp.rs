use std::io;

fn gcd(a: i64, b: i64) -> i64 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    let n = tokens.next().unwrap().parse::<usize>().unwrap();
    let m = tokens.next().unwrap().parse::<usize>().unwrap();

    let nums: Vec<i32> = tokens.take(n).map(|s| s.parse().unwrap()).collect();
    let target: Vec<i32> = tokens.take(m).map(|s| s.parse().unwrap()).collect();

    let size = 1 << m;
    let mut g = vec![1; size];
    for i in 0..size {
        for j in 0..m {
            if (i >> j) & 1 != 0 {
                let target_j = target[j] as i64;
                let current_gcd = gcd(g[i], target_j);
                g[i] = g[i] / current_gcd * target_j;
            }
        }
    }

    const INF: i64 = 1e18 as i64;
    let mut prev = vec![INF; size];
    prev[0] = 0;

    for num in &nums {
        let mut curr = prev.clone();
        for j in 0..size {
            let mut k = j;
            while k != 0 {
                let g_k = g[k];
                let v = if g_k == 0 {
                    0
                } else {
                    let q = (num as i64 + g_k - 1) / g_k;
                    q * g_k - num as i64
                };
                let prev_state = prev[(j ^ k) as usize];
                if prev_state + v < curr[j as usize] {
                    curr[j as usize] = prev_state + v;
                }
                k = (k - 1) & j;
            }
        }
        prev = curr;
    }

    println!("{}", prev[(1 << m) - 1]);
}