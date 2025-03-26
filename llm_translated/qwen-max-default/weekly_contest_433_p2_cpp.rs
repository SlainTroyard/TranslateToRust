use std::io::{self, BufRead, Write};

const MOD: i64 = 1_000_000_007;
const MX: usize = 100_000;

static mut F: [i64; MX] = [0; MX]; // F[i] = i!
static mut INV_F: [i64; MX] = [0; MX]; // INV_F[i] = i!^-1

fn pow(mut x: i64, mut n: i32) -> i64 {
    let mut res = 1;
    while n > 0 {
        if n % 2 == 1 {
            res = res * x % MOD;
        }
        x = x * x % MOD;
        n /= 2;
    }
    res
}

fn init() {
    unsafe {
        F[0] = 1;
        for i in 1..MX {
            F[i] = F[i - 1] * i as i64 % MOD;
        }

        INV_F[MX - 1] = pow(F[MX - 1], MOD - 2);
        for i in (1..MX).rev() {
            INV_F[i - 1] = INV_F[i] * i as i64 % MOD;
        }
    }
}

fn comb(n: i32, m: i32) -> i64 {
    if m > n {
        0
    } else {
        unsafe {
            F[n as usize] * INV_F[m as usize] % MOD * INV_F[(n - m) as usize] % MOD
        }
    }
}

struct Solution;

impl Solution {
    fn min_max_sums(nums: &Vec<i32>, k: i32) -> i64 {
        let mut nums = nums.clone();
        nums.sort_unstable();
        let n = nums.len();
        let mut ans = 0;
        let mut s = 1;
        for i in 0..n {
            ans = (ans + s * (nums[i] + nums[n - 1 - i]) as i64) % MOD;
            s = (s * 2 - comb(i as i32, k - 1) + MOD) % MOD;
        }
        ans
    }
}

fn main() {
    init();

    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut stdin_lock = stdin.lock();
    let mut buffer = String::new();

    stdin_lock.read_line(&mut buffer).unwrap();
    let mut iter = buffer.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();

    buffer.clear();
    stdin_lock.read_line(&mut buffer).unwrap();
    let nums: Vec<i32> = buffer
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let sol = Solution;
    let result = sol.min_max_sums(&nums, k);
    writeln!(stdout, "{}", result).unwrap();
}