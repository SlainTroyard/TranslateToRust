use std::io::{self, BufRead, Write};

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

struct Solution;

impl Solution {
    fn minimum_increments(nums: &Vec<usize>, target: &Vec<usize>) -> usize {
        let n = nums.len();
        let m = target.len();

        let mut g = vec![1; 1 << m];
        for i in 0..(1 << m) {
            for j in 0..m {
                if (i >> j) & 1 == 1 {
                    g[i] = lcm(g[i], target[j]);
                }
            }
        }

        const INF: usize = 1_000_000_000_000_000_000;
        let mut f = vec![vec![INF; 1 << m]; 2];

        f[0][0] = 0;

        for i in 1..=n {
            for j in 0..(1 << m) {
                f[i & 1][j] = f[(i & 1) ^ 1][j];
            }
            for j in 0..(1 << m) {
                let mut k = j;
                while k > 0 {
                    let v = ((nums[i - 1] + g[k] - 1) / g[k]) * g[k] - nums[i - 1];
                    f[i & 1][j] = f[i & 1][j].min(f[(i & 1) ^ 1][j ^ k] + v);
                    k = (k - 1) & j;
                }
            }
        }

        f[n & 1][(1 << m) - 1]
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buffer = String::new();

    // Read the number of elements in nums and target
    stdin.lock().read_line(&mut buffer).unwrap();
    let mut iter = buffer.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    buffer.clear();

    // Read the elements of nums
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        stdin.lock().read_line(&mut buffer).unwrap();
        nums.push(buffer.trim().parse().unwrap());
        buffer.clear();
    }

    // Read the elements of target
    let mut target = Vec::with_capacity(m);
    for _ in 0..m {
        stdin.lock().read_line(&mut buffer).unwrap();
        target.push(buffer.trim().parse().unwrap());
        buffer.clear();
    }

    // Create an instance of Solution and call the function
    let solution = Solution;
    let result = solution.minimum_increments(&nums, &target);

    // Output the result
    writeln!(stdout, "{}", result).unwrap();
}