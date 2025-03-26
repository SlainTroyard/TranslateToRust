use std::cmp::Ordering;
use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    let stdout = io::stdout();
    let mut stdout_lock = stdout.lock();

    // Read the number of intervals
    let n: usize = {
        let mut input = String::new();
        stdin_lock.read_line(&mut input).unwrap();
        input.trim().parse().unwrap()
    };

    // Read the intervals
    let mut intervals: Vec<Vec<i32>> = Vec::with_capacity(n);
    for _ in 0..n {
        let mut input = String::new();
        stdin_lock.read_line(&mut input).unwrap();
        let nums: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        intervals.push(nums);
    }

    // Solve the problem
    let solution = Solution::new();
    let result = solution.maximum_weight(intervals);

    // Output the result
    for (i, &val) in result.iter().enumerate() {
        if i > 0 {
            write!(&mut stdout_lock, " ").unwrap();
        }
        write!(&mut stdout_lock, "{}", val).unwrap();
    }
    writeln!(&mut stdout_lock).unwrap();
}

struct Solution;

impl Solution {
    fn new() -> Self {
        Solution
    }

    fn maximum_weight(&self, intervals: Vec<Vec<i32>>) -> Vec<i32> {
        let n = intervals.len();
        let mut vec: Vec<(i32, i32)> = vec![(-1, -1)];
        for (i, seg) in intervals.iter().enumerate() {
            vec.push((seg[0], -1));
            vec.push((seg[1], i as i32));
        }
        vec.sort_unstable_by(|a, b| a.0.cmp(&b.0));

        let n = vec.len();
        const INF: i64 = 1_000_000_000_000_000_000;
        let mut f: Vec<[LLArray; 5]> = vec![[LLArray::default(); 5]; n];

        // Initialize f[0]
        for j in 1..=4 {
            f[0][j] = [INF, INF, INF, INF, INF];
        }
        f[0][0] = [0, INF, INF, INF, INF];

        // Dynamic programming
        for i in 1..n {
            for j in 0..=4 {
                f[i][j] = f[i - 1][j];
            }
            let idx = vec[i].1;
            if idx >= 0 {
                let l = intervals[idx as usize][0];
                let (mut head, mut tail) = (0, i - 1);
                while head < tail {
                    let mid = (head + tail + 1) / 2;
                    if vec[mid].0 < l {
                        head = mid;
                    } else {
                        tail = mid - 1;
                    }
                }

                for j in 1..=4 {
                    let mut tmp = f[head][j - 1];
                    tmp[0] -= intervals[idx as usize][2] as i64;
                    tmp[j] = idx;
                    tmp.sort_unstable();
                    f[i][j] = f[i][j].min(tmp);
                }
            }
        }

        // Calculate the answer
        let mut ans: LLArray = [INF, INF, INF, INF, INF];
        for j in 1..=4 {
            ans = ans.min(f[n - 1][j]);
        }
        ans.iter()
            .filter(|&&x| x < INF)
            .map(|&x| x as i32)
            .collect()
    }
}

type LLArray = [i64; 5];

impl PartialEq for [LLArray; 5] {
    fn eq(&self, other: &[LLArray; 5]) -> bool {
        self.iter().zip(other.iter()).all(|(a, b)| a == b)
    }
}

impl Eq for [LLArray; 5] {}

impl PartialOrd for [LLArray; 5] {
    fn partial_cmp(&self, other: &[LLArray; 5]) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for [LLArray; 5] {
    fn cmp(&self, other: &[LLArray; 5]) -> Ordering {
        self.iter().zip(other.iter()).fold(Ordering::Equal, |acc, (a, b)| {
            acc.then_with(|| a.cmp(b))
        })
    }
}