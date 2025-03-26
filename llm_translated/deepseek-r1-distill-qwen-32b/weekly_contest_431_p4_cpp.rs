use std::io;
use std::cmp::min;

const INF: i64 = 1e18 as i64;

struct Solution {}

impl Solution {
    pub fn maximum_weight(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        let n = intervals.len();
        let mut vec = Vec::with_capacity(2 * n + 1);
        vec.push((-1, -1));
        for i in 0..n {
            let seg = &intervals[i];
            vec.push((seg[0], -1));
            vec.push((seg[1], i as i32));
        }
        vec.sort();

        let m = vec.len();
        let mut f = vec![vec![[INF; 5]; 5]; m];
        
        // Initialize f[0]
        for j in 1..=4 {
            f[0][j] = [INF; 5];
        }
        f[0][0] = [0, INF, INF, INF, INF];

        for i in 1..m {
            for j in 0..=4 {
                f[i][j] = f[i - 1][j].clone();
            }
            let idx = vec[i].1;
            if idx >= 0 {
                let l = intervals[idx as usize][0];
                let mut head = 0;
                let mut tail = i - 1;
                while head < tail {
                    let mid = (head + tail + 1) >> 1;
                    if vec[mid].0 < l {
                        head = mid;
                    } else {
                        tail = mid - 1;
                    }
                }

                for j in 1..=4 {
                    let mut tmp = f[head][j - 1].clone();
                    tmp[0] -= intervals[idx as usize][2] as i64;
                    tmp[j] = idx as i64;
                    tmp.sort();
                    if tmp < f[i][j] {
                        f[i][j] = tmp;
                    }
                }
            }
        }

        let mut ans = [INF; 5];
        for j in 1..=4 {
            ans = min(ans, f[m - 1][j]);
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

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();
    
    let mut intervals = Vec::with_capacity(n);
    for _ in 0..n {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let nums: Vec<i32> = line.trim()
                                   .split_whitespace()
                                   .map(|x| x.parse().unwrap())
                                   .collect();
        intervals.push(nums);
    }
    
    let sol = Solution {};
    let ans = sol.maximum_weight(intervals);
    for num in ans {
        print!("{} ", num);
    }
    println!();
}