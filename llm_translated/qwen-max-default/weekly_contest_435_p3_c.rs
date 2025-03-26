use std::io::{self, Write};

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
    if a < b { a } else { b }
}

// 主函数实现
fn minimum_increments(nums: &[i32], target: &[i32]) -> i32 {
    let n = nums.len();
    let m = target.len();

    // 计算g数组 - 用于计算可整除性
    let mut g = vec![1; 1 << m];
    for i in 0..(1 << m) {
        for j in 0..m {
            if (i >> j) & 1 == 1 {
                g[i] = g[i] / gcd(g[i], target[j] as i64) * target[j] as i64;
            }
        }
    }

    // 动态规划数组
    const INF: i64 = 1e18 as i64;
    let mut f = vec![vec![INF; 1 << m]; 2];

    // 初始化DP数组
    f[0][0] = 0;

    // 动态规划过程
    for i in 1..=n {
        for j in 0..(1 << m) {
            f[i & 1][j] = f[(i & 1) ^ 1][j];
        }

        for j in 0..(1 << m) {
            for k in (0..j).rev() {
                if (k | j) == j {
                    let v = ((nums[i - 1] as i64 + g[k] - 1) / g[k]) * g[k] - nums[i - 1] as i64;
                    f[i & 1][j] = min(f[i & 1][j], f[(i & 1) ^ 1][j ^ k] + v);
                }
            }
        }
    }

    // 获取结果
    f[n & 1][(1 << m) - 1] as i32
}

fn main() {
    // 读取输入
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().expect("Failed to parse n");
    let m: usize = iter.next().unwrap().parse().expect("Failed to parse m");

    // 分配内存并读取数组
    let mut nums = vec![0; n];
    let mut target = vec![0; m];

    for i in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        nums[i] = input.trim().parse().expect("Failed to parse nums");
    }

    for i in 0..m {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        target[i] = input.trim().parse().expect("Failed to parse target");
    }

    // 调用函数计算结果
    let result = minimum_increments(&nums, &target);

    // 输出结果
    println!("{}", result);
}