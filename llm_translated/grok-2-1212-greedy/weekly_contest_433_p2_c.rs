// Problem: Weekly Contest 433 Problem 2

use std::io::{self, BufRead, Write};

const MOD: i64 = 1_000_000_007;
const MX: usize = 100_000;

// F[i] = i!
// INV_F[i] = i!^-1
static mut F: [i64; MX] = [0; MX];
static mut INV_F: [i64; MX] = [0; MX];

// 比较函数用于排序
fn compare(a: &i32, b: &i32) -> std::cmp::Ordering {
    a.cmp(b)
}

// 幂运算函数
fn power(x: i64, n: i32) -> i64 {
    let mut res = 1;
    let mut x = x;
    let mut n = n;
    while n > 0 {
        if n % 2 == 1 {
            res = (res * x) % MOD;
        }
        x = (x * x) % MOD;
        n /= 2;
    }
    res
}

// 组合数计算函数
fn comb(n: i32, m: i32) -> i64 {
    if m > n {
        return 0;
    }
    unsafe {
        (((F[n as usize] * INV_F[m as usize]) % MOD) * INV_F[(n - m) as usize]) % MOD
    }
}

// 初始化阶乘和逆元数组
fn initialize() {
    unsafe {
        F[0] = 1;
        for i in 1..MX {
            F[i] = (F[i - 1] * i as i64) % MOD;
        }

        INV_F[MX - 1] = power(F[MX - 1], MOD as i32 - 2);
        for i in (1..MX).rev() {
            INV_F[i - 1] = (INV_F[i] * i as i64) % MOD;
        }
    }
}

// 主函数实现
fn min_max_sums(nums: &mut [i32], nums_size: usize, k: i32) -> i32 {
    // 确保阶乘和逆元数组已初始化
    static mut INITIALIZED: bool = false;
    unsafe {
        if !INITIALIZED {
            initialize();
            INITIALIZED = true;
        }
    }

    // 对数组排序
    nums.sort_unstable_by(compare);

    let mut ans = 0;
    let mut s = 1;
    for i in 0..nums_size {
        ans = (ans + s * (nums[i] as i64 + nums[nums_size - 1 - i] as i64)) % MOD;
        s = (s * 2 - comb(i as i32, k - 1) + MOD) % MOD;
    }

    ans as i32
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut lines = stdin.lock().lines();

    // 读取输入
    let first_line = lines.next().unwrap()?;
    let mut iter = first_line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();

    // 读取数组
    let mut nums: Vec<i32> = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap()?;
        nums.push(line.trim().parse().unwrap());
    }

    // 调用函数计算结果
    let result = min_max_sums(&mut nums, n, k);

    // 输出结果
    writeln!(stdout, "{}", result)?;

    Ok(())
}