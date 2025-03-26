// 计算最大公约数
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

// 返回两个数中的较小值
fn min(a: i64, b: i64) -> i64 {
    if a < b {
        a
    } else {
        b
    }
}

// 主函数实现
fn minimum_increments(nums: &[i32], target: &[i32]) -> i32 {
    let n = nums.len();
    let m = target.len();

    // 计算g数组 - 用于计算可整除性
    let mut g: Vec<i64> = vec![0; 1 << m];

    for i in 0..(1 << m) {
        g[i] = 1;
        for j in 0..m {
            if (i >> j) & 1 != 0 {
                g[i] = g[i] / gcd(g[i], target[j] as i64) * (target[j] as i64);
            }
        }
    }

    // 动态规划数组
    const INF: i64 = 1e18 as i64;
    let mut f: Vec<Vec<i64>> = vec![vec![0; 1 << m]; 2];

    // 初始化DP数组
    for j in 0..(1 << m) {
        f[0][j] = INF;
    }
    f[0][0] = 0;

    // 动态规划过程
    for i in 1..=n {
        for j in 0..(1 << m) {
            f[i & 1][j] = f[(i & 1) ^ 1][j];
        }

        for j in 0..(1 << m) {
            let mut k = j;
            while k > 0 {
                let v = ((nums[i - 1] as i64 + g[k] - 1) / g[k]) * g[k] - (nums[i - 1] as i64);
                f[i & 1][j] = min(f[i & 1][j], f[(i & 1) ^ 1][j ^ k] + v);
                k = (k - 1) & j;
            }
        }
    }

    // 获取结果
    f[n & 1][(1 << m) - 1] as i32
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().expect("Failed to read line");
    let mut nm_iter = first_line.split_whitespace();
    let n: usize = nm_iter.next().unwrap().parse().expect("Failed to parse n");
    let m: usize = nm_iter.next().unwrap().parse().expect("Failed to parse m");

    let nums_line = lines.next().unwrap().expect("Failed to read line");
    let nums: Vec<i32> = nums_line
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse num"))
        .collect();

    let target_line = lines.next().unwrap().expect("Failed to read line");
    let target: Vec<i32> = target_line
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse target"))
        .collect();

    if nums.len() != n || target.len() != m {
        eprintln!("Input size mismatch");
        return;
    }

    let result = minimum_increments(&nums, &target);
    println!("{}", result);
}