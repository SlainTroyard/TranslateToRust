use std::io::{self, Write};

const MOD: i64 = 1_000_000_007;
const MX: usize = 100_000;

static mut F: [i64; MX] = [0; MX]; // F[i] = i!
static mut INV_F: [i64; MX] = [0; MX]; // INV_F[i] = i!^-1

// 幂运算函数
fn power(mut x: i64, mut n: i32) -> i64 {
    let mut res = 1;
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
fn comb(n: usize, m: usize) -> i64 {
    if m > n {
        return 0;
    }
    unsafe {
        (((F[n] * INV_F[m]) % MOD) * INV_F[n - m]) % MOD
    }
}

// 初始化阶乘和逆元数组
fn initialize() {
    unsafe {
        F[0] = 1;
        for i in 1..MX {
            F[i] = (F[i - 1] * i as i64) % MOD;
        }

        INV_F[MX - 1] = power(F[MX - 1], (MOD - 2) as i32);
        for i in (1..MX).rev() {
            INV_F[i - 1] = (INV_F[i] * i as i64) % MOD;
        }
    }
}

// 主函数实现
fn min_max_sums(nums: &mut [i32], k: usize) -> i32 {
    // 确保阶乘和逆元数组已初始化
    static INITIALIZED: std::sync::Once = std::sync::Once::new();
    INITIALIZED.call_once(|| {
        initialize();
    });

    // 对数组排序
    nums.sort_unstable();

    let mut ans: i64 = 0;
    let mut s: i64 = 1;
    let n = nums.len();

    for i in 0..n {
        ans = (ans + s * (nums[i] as i64 + nums[n - 1 - i] as i64)) % MOD;
        s = (s * 2 - comb(i, k - 1) + MOD) % MOD;
    }

    ans as i32
}

fn main() -> io::Result<()> {
    // 读取输入
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().ok_or(io::Error::new(io::ErrorKind::InvalidInput, "Missing n"))?.parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid n"))?;
    let k: usize = iter.next().ok_or(io::Error::new(io::ErrorKind::InvalidInput, "Missing k"))?.parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid k"))?;

    // 分配内存并读取数组
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let num: i32 = input.trim().parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid number"))?;
        nums.push(num);
    }

    // 调用函数计算结果
    let result = min_max_sums(&mut nums, k);

    // 输出结果
    println!("{}", result);

    Ok(())
}