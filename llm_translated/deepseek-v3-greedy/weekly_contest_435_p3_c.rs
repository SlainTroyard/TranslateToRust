use std::cmp::min;
use std::io::{self, Write};

// 计算最大公约数
fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

// 主函数实现
fn minimum_increments(nums: &[i32], target: &[i32]) -> i32 {
    let n = nums.len();
    let m = target.len();
    
    // 计算g数组 - 用于计算可整除性
    let mut g = vec![1; 1 << m];
    for i in 0..(1 << m) {
        for j in 0..m {
            if (i >> j) & 1 != 0 {
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
            let mut k = j;
            while k > 0 {
                let v = ((nums[i - 1] as i64 + g[k] - 1) / g[k]) * g[k] - nums[i - 1] as i64;
                f[i & 1][j] = min(f[i & 1][j], f[(i & 1) ^ 1][j ^ k] + v);
                k = (k - 1) & j;
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
    let n: usize = iter.next().unwrap().parse().expect("Invalid input for n");
    let m: usize = iter.next().unwrap().parse().expect("Invalid input for m");
    
    // 读取nums数组
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        nums.push(input.trim().parse().expect("Invalid input for nums"));
    }
    
    // 读取target数组
    let mut target = Vec::with_capacity(m);
    for _ in 0..m {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        target.push(input.trim().parse().expect("Invalid input for target"));
    }
    
    // 调用函数计算结果
    let result = minimum_increments(&nums, &target);
    
    // 输出结果
    println!("{}", result);
}