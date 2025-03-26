use std::io::{self, BufRead};

const MOD: i64 = 1_000_000_007;
const MX: usize = 100_000;

// F[i] = i!
// INV_F[i] = i!^-1
static mut F: [i64; MX] = [0; MX];
static mut INV_F: [i64; MX] = [0; MX];

fn pow(mut x: i64, mut n: i32) -> i64 {
    let mut res = 1;
    while n > 0 {
        if n & 1 == 1 {
            res = (res * x) % MOD;
        }
        x = (x * x) % MOD;
        n /= 2;
    }
    res
}

// Initialize F and INV_F arrays
fn init() {
    unsafe {
        F[0] = 1;
        for i in 1..MX {
            F[i] = (F[i - 1] * i as i64) % MOD;
        }

        INV_F[MX - 1] = pow(F[MX - 1], MOD as i32 - 2);
        for i in (0..MX - 1).rev() {
            INV_F[i] = (INV_F[i + 1] * (i + 1) as i64) % MOD;
        }
    }
}

fn comb(n: i32, m: i32) -> i64 {
    if m > n {
        return 0;
    }
    unsafe {
        ((F[n as usize] * INV_F[m as usize]) % MOD * INV_F[(n - m) as usize]) % MOD
    }
}

struct Solution;

impl Solution {
    fn min_max_sums(nums: &mut Vec<i32>, k: i32) -> i64 {
        nums.sort_unstable();
        let n = nums.len();
        let mut ans = 0;
        let mut s = 1;
        for i in 0..n {
            ans = (ans + s * (nums[i] as i64 + nums[n - 1 - i] as i64)) % MOD;
            s = (s * 2 - comb(i as i32, k - 1) + MOD) % MOD;
        }
        ans
    }
}

fn main() -> io::Result<()> {
    init();

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read n and k
    let nk = lines.next().unwrap()?.split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let n = nk[0];
    let k = nk[1];

    // Read nums
    let mut nums = lines.next().unwrap()?.split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    // Calculate and print result
    let sol = Solution::min_max_sums(&mut nums, k);
    println!("{}", sol);

    Ok(())
}