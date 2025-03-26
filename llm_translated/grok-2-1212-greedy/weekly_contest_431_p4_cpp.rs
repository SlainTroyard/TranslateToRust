use std::cmp::min;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap()?.parse().unwrap();
    let mut intervals = Vec::with_capacity(n);

    for _ in 0..n {
        let line = lines.next().unwrap()?;
        let nums: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        intervals.push(nums);
    }

    let solution = Solution {};
    let ans = solution.maximum_weight(&mut intervals);

    for &num in &ans {
        print!("{} ", num);
    }
    println!();

    Ok(())
}

struct Solution;

impl Solution {
    fn maximum_weight(&self, intervals: &mut Vec<Vec<i32>>) -> Vec<i32> {
        let n = intervals.len();

        let mut vec = vec![(-1, -1)];
        for i in 0..n {
            let seg = &intervals[i];
            vec.push((seg[0], -1));
            vec.push((seg[1], i as i32));
        }
        vec.sort_unstable();

        let n = vec.len();
        const INF: i64 = 1e18 as i64;
        let mut f = vec![[[INF; 5]; 5]; n];

        // Initialize f[0]
        for j in 1..=4 {
            f[0][j] = [INF; 5];
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
                let mut head = 0;
                let mut tail = i - 1;
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
                    tmp[j] = idx as i64;
                    tmp.sort_unstable();
                    f[i][j] = min_array(f[i][j], tmp);
                }
            }
        }

        // Calculate answer
        let mut ans = [INF; 5];
        for j in 1..=4 {
            ans = min_array(ans, f[n - 1][j]);
        }
        let mut ret = Vec::new();
        for j in 1..=4 {
            if ans[j] < INF {
                ret.push(ans[j] as i32);
            }
        }
        ret
    }
}

fn min_array(a: [i64; 5], b: [i64; 5]) -> [i64; 5] {
    let mut result = [0; 5];
    for i in 0..5 {
        result[i] = std::cmp::min(a[i], b[i]);
    }
    result
}