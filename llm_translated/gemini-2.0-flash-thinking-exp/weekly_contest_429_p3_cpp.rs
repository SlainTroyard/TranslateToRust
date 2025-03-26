use std::io;
use std::cmp::{max, min};

struct Solution {}

impl Solution {
    fn merge(&self, intervals: Vec<Vec<i32>>, len: i32) -> Vec<Vec<i32>> {
        if intervals.is_empty() {
            return Vec::new();
        }
        let n = intervals.len();
        let mut res: Vec<Vec<i32>> = Vec::new();
        res.push(intervals[0].clone());

        for i in 1..n {
            let curr_start = intervals[i][0];
            let curr_end = intervals[i][1];

            if curr_start <= res.last().unwrap()[1] && (curr_start - res.last().unwrap()[0] + 1) <= len {
                res.last_mut().unwrap()[1] = max(res.last().unwrap()[1], curr_end);
            } else {
                res.push(intervals[i].clone());
            }
        }
        res
    }

    fn is_poss(&self, s: String, op: i32, mid: i32) -> bool {
        let n = s.len();
        let mut i = 0;
        let mut j = 0;
        let mut zero = 0;
        let mut one = 0;
        let mut intervals: Vec<Vec<i32>> = Vec::new();
        let s_chars: Vec<char> = s.chars().collect();

        while j < n {
            if s_chars[j] == '0' {
                zero += 1;
            } else {
                one += 1;
            }

            while (j - i + 1) as i32 > mid {
                if s_chars[i] == '0' {
                    zero -= 1;
                } else {
                    one -= 1;
                }
                i += 1;
            }

            if (j - i + 1) as i32 == mid {
                if zero == 0 || one == 0 {
                    intervals.push(vec![i as i32, j as i32]);
                }
            }
            j += 1;
        }

        let merged = self.merge(intervals, mid);
        merged.len() as i32 <= op
    }

    fn get_mini(&self, s: String, even: char, odd: char) -> i32 {
        let n = s.len();
        let mut op = 0;
        let s_chars: Vec<char> = s.chars().collect();
        for i in 0..n {
            if i % 2 == 1 && s_chars[i] != odd {
                op += 1;
            } else if i % 2 == 0 && s_chars[i] != even {
                op += 1;
            }
        }
        op
    }

    fn min_length(&self, s: String, num_ops: i32) -> i32 {
        let n = s.len();
        let mut start = 3;
        let mut end = n as i32;
        let mut res = 2;

        let mut op = n + 1;
        op = min(op as i32, self.get_mini(s.clone(), '0', '1'));
        op = min(op as i32, self.get_mini(s.clone(), '1', '0'));
        if op <= num_ops {
            return 1;
        }

        while start <= end {
            let mid = start + (end - start) / 2;
            let curr = self.is_poss(s.clone(), num_ops, mid);
            if curr {
                end = mid - 1;
            } else {
                res = mid;
                start = mid + 1;
            }
        }
        res
    }
}

fn main() {
    let solution = Solution {};
    let mut s = String::new();
    let mut num_ops_str = String::new();

    // Input the binary string
    io::stdin().read_line(&mut s).unwrap();
    let s = s.trim().to_string();

    // Input the number of operations
    io::stdin().read_line(&mut num_ops_str).unwrap();
    let num_ops: i32 = num_ops_str.trim().parse().unwrap();

    // Compute the result
    let result = solution.min_length(s, num_ops);

    // Output the result
    println!("{}", result);
}