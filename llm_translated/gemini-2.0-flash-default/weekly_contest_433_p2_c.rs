use std::io;
use std::io::Read;
use std::cmp::min;

const MOD: i64 = 1_000_000_007;
const MX: usize = 100000;

static mut F: [i64; MX] = [0; MX]; // F[i] = i!
static mut INV_F: [i64; MX] = [0; MX]; // INV_F[i] = i!^-1
static mut INITIALIZED: bool = false;

// 幂运算函数
fn power(x: i64, n: i32) -> i64 {
    let mut res = 1;
    let mut x_mut = x % MOD;
    let mut n_mut = n;
    while n_mut > 0 {
        if n_mut % 2 == 1 {
            res = (res * x_mut) % MOD;
        }
        x_mut = (x_mut * x_mut) % MOD;
        n_mut /= 2;
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
fn min_max_sums(nums: &mut [i32], k: i32) -> i32 {
    // 确保阶乘和逆元数组已初始化
    unsafe {
        if !INITIALIZED {
            initialize();
            INITIALIZED = true;
        }
    }

    // 对数组排序
    nums.sort_unstable();

    let mut ans: i64 = 0;
    let mut s: i64 = 1;
    let nums_len = nums.len();

    for i in 0..nums_len {
        ans = (ans + s * (nums[i] as i64 + nums[nums_len - 1 - i] as i64)) % MOD;
        s = (s * 2 - comb(i as i32, k - 1) + MOD) % MOD;
    }

    ans as i32
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let first_line = lines.next().unwrap();
    let mut nk = first_line.split_whitespace();
    let n: i32 = nk.next().unwrap().parse().unwrap();
    let k: i32 = nk.next().unwrap().parse().unwrap();

    let second_line = lines.next().unwrap();
    let nums: Vec<i32> = second_line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut nums_mut = nums.clone();

    // 调用函数计算结果
    let result = min_max_sums(&mut nums_mut, k);

    // 输出结果
    println!("{}", result);

    Ok(())
}