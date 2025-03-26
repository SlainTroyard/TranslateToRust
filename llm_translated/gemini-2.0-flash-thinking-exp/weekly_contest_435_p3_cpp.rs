use std::io::{self, BufRead, Write};
use std::str::SplitWhitespace;

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

struct Solution {}

impl Solution {
    pub fn minimumIncrements(&self, nums: &[i32], target: &[i32]) -> i64 {
        let n = nums.len();
        let m = target.len();

        let mut g: Vec<i64> = vec![0; 1 << m];
        for i in 0..(1 << m) {
            g[i] = 1;
            for j in 0..m {
                if (i >> j) & 1 != 0 {
                    g[i] = (g[i] / gcd(g[i], target[j] as i64)) * (target[j] as i64);
                }
            }
        }

        const INF: i64 = 1_000_000_000_000_000_000;
        let mut f: Vec<Vec<i64>> = vec![vec![INF; 1 << m]; 2];
        f[0][0] = 0;

        for i in 1..=n {
            for j in 0..(1 << m) {
                f[i & 1][j] = f[(i - 1) & 1][j];
            }
            for j in 0..(1 << m) {
                let mut k = j;
                while k > 0 {
                    let v = (nums[i - 1] as i64 + g[k] - 1) / g[k] * g[k] - nums[i - 1] as i64;
                    f[i & 1][j] = std::cmp::min(f[i & 1][j], f[(i - 1) & 1][j ^ k] + v);
                    k = (k - 1) & j;
                }
            }
        }

        f[n & 1][(1 << m) - 1]
    }
}

fn parse_input<T>(input: &mut SplitWhitespace) -> T
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    input.next().unwrap().parse::<T>().unwrap()
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut reader = stdin.lock();
    let mut buffer = String::new();

    reader.read_line(&mut buffer)?;
    let mut input = buffer.split_whitespace();
    let n: usize = parse_input(&mut input);
    let m: usize = parse_input(&mut input);

    buffer.clear();
    reader.read_line(&mut buffer)?;
    let nums_str = buffer.split_whitespace();
    let mut nums: Vec<i32> = Vec::with_capacity(n);
    for s in nums_str {
        nums.push(s.parse::<i32>().unwrap());
    }

    buffer.clear();
    reader.read_line(&mut buffer)?;
    let target_str = buffer.split_whitespace();
    let mut target: Vec<i32> = Vec::with_capacity(m);
    for s in target_str {
        target.push(s.parse::<i32>().unwrap());
    }

    let solution = Solution {};
    let result = solution.minimumIncrements(&nums, &target);

    writeln!(stdout, "{}", result)?;

    Ok(())
}