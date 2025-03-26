use std::io::{self, BufRead, Write};
use std::vec;

const MOD: i64 = 1_000_000_007;
const MX: usize = 100_000;

static mut F: [i64; MX] = [0; MX]; // F[i] = i!
static mut INV_F: [i64; MX] = [0; MX]; // INV_F[i] = i!^-1
static mut INITIALIZED: bool = false;

// 幂运算函数
fn power(x: i64, n: i32) -> i64 {
    let mut res: i64 = 1;
    let mut base = x % MOD;
    let mut exp = n;
    while exp > 0 {
        if exp % 2 == 1 {
            res = (res * base) % MOD;
        }
        base = (base * base) % MOD;
        exp /= 2;
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
            F[i] = (F[i - 1] * (i as i64)) % MOD;
        }

        INV_F[MX - 1] = power(F[MX - 1], (MOD - 2) as i32);
        for i in (1..MX).rev() {
            INV_F[i - 1] = (INV_F[i] * (i as i64)) % MOD;
        }
        INITIALIZED = true;
    }
}

// 主函数实现
fn min_max_sums(nums: &mut [i32], k: i32) -> i32 {
    unsafe {
        if !INITIALIZED {
            initialize();
        }
    }

    nums.sort_unstable();
    let n = nums.len();
    let mut ans: i64 = 0;
    let mut s: i64 = 1;

    for i in 0..n {
        ans = (ans + s * ((nums[i] + nums[n - 1 - i]) as i64)) % MOD;
        s = (s * 2 - comb(i, (k - 1) as usize) + MOD) % MOD;
    }

    ans as i32
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut stdout = io::stdout();

    let first_line = lines.next().ok_or("No input line for n and k")??;
    let mut nk_iter = first_line.split_whitespace();
    let n: usize = nk_iter.next().ok_or("Missing n")?.parse()?;
    let k: i32 = nk_iter.next().ok_or("Missing k")?.parse()?;

    let mut nums: Vec<i32> = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().ok_or("Missing number input")??;
        let num: i32 = line.trim().parse()?;
        nums.push(num);
    }

    let result = min_max_sums(&mut nums, k);

    writeln!(stdout, "{}", result)?;

    Ok(())
}