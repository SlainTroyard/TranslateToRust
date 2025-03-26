fn solve() -> Result<(), Box<dyn std::error::Error>> {
    use std::io::{self, BufRead, Write};

    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut stdout = io::stdout();

    let mut line = String::new();
    reader.read_line(&mut line)?;
    let mut parts = line.trim().split_whitespace();
    let n: usize = parts.next().unwrap().parse()?;
    let k: i32 = parts.next().unwrap().parse()?;

    line.clear();
    reader.read_line(&mut line)?;
    let nums: Vec<i32> = line
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    struct Solution {}
    impl Solution {
        pub fn max_frequency(&self, nums: Vec<i32>, k: i32) -> i32 {
            let mut f0 = 0;
            let mut f1 = [0; 51];
            let mut max_f1 = 0;
            let mut f2 = 0;

            for x in nums {
                f2 = std::cmp::max(f2, max_f1) + if x == k { 1 } else { 0 };
                f1[x as usize] = std::cmp::max(f1[x as usize], f0) + 1;
                f0 += if x == k { 1 } else { 0 };
                max_f1 = std::cmp::max(max_f1, f1[x as usize]);
            }
            std::cmp::max(max_f1, f2)
        }
    }

    let solution = Solution {};
    let result = solution.max_frequency(nums, k);
    writeln!(stdout, "{}", result)?;

    Ok(())
}

fn main() {
    solve().unwrap();
}