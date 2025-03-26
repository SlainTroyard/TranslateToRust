use std::io;
use std::cmp::min;

// 计算最大公约数
fn gcd(a: i64, b: i64) -> i64 {
    let mut a_mut = a;
    let mut b_mut = b;
    while b_mut != 0 {
        let temp = b_mut;
        b_mut = a_mut % b_mut;
        a_mut = temp;
    }
    a_mut
}

// 主函数实现
fn minimum_increments(nums: &[i32], target: &[i32]) -> i32 {
    let n = nums.len();
    let m = target.len();

    // 计算g数组 - 用于计算可整除性
    let mut g = vec![1i64; 1 << m];

    for i in 0..(1 << m) {
        for j in 0..m {
            if (i >> j) & 1 != 0 {
                g[i] = g[i] / gcd(g[i], target[j] as i64) * (target[j] as i64);
            }
        }
    }

    // 动态规划数组
    const INF: i64 = 1_000_000_000_000_000_000;
    let mut f = vec![vec![INF; 1 << m]; 2];

    // 初始化DP数组
    f[0][0] = 0;

    // 动态规划过程
    for i in 1..=n {
        for j in 0..(1 << m) {
            f[i & 1][j] = f[(i & 1) ^ 1][j];
        }

        for j in 0..(1 << m) {
            let mut k = j;
            while k > 0 {
                let v = (((nums[i - 1] as i64) + g[k] - 1) / g[k]) * g[k] - (nums[i - 1] as i64);
                f[i & 1][j] = min(f[i & 1][j], f[(i & 1) ^ 1][j ^ k] + v);
                k = (k - 1) & j;
            }
        }
    }

    // 获取结果
    f[n & 1][(1 << m) - 1] as i32
}

fn main() -> io::Result<()> {
    // 读取输入
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    let mut iter = line.trim().split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    // 分配内存并读取数组
    let mut nums = vec![0; n];
    let mut target = vec![0; m];

    line.clear();
    io::stdin().read_line(&mut line)?;
    let nums_str: Vec<&str> = line.trim().split_whitespace().collect();
    for i in 0..n {
        nums[i] = nums_str[i].parse().unwrap();
    }

    line.clear();
    io::stdin().read_line(&mut line)?;
    let target_str: Vec<&str> = line.trim().split_whitespace().collect();
    for i in 0..m {
        target[i] = target_str[i].parse().unwrap();
    }

    // 调用函数计算结果
    let result = minimum_increments(&nums, &target);

    // 输出结果
    println!("{}", result);

    Ok(())
}