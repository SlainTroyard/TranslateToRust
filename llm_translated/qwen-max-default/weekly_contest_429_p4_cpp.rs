use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn merge(intervals: &mut Vec<Vec<i32>>, len: i32) -> Vec<Vec<i32>> {
        if intervals.is_empty() {
            return vec![];
        }
        let n = intervals.len();

        let mut res = vec![intervals[0].clone()];

        for i in 1..n {
            let curr_start = intervals[i][0];
            let curr_end = intervals[i][1];

            if curr_start <= res.last().unwrap()[1] && (curr_start - res.last().unwrap()[0] + 1) <= len as i32 {
                res.last_mut().unwrap()[1] = res.last().unwrap()[1].max(curr_end);
            } else {
                res.push(intervals[i].clone());
            }
        }
        res
    }

    fn is_poss(s: &str, op: i32, mid: i32) -> bool {
        let n = s.len();
        let mut i = 0;
        let mut j = 0;
        let mut zero = 0;
        let mut one = 0;
        let mut intervals = Vec::new();
        while j < n {
            if s.chars().nth(j).unwrap() == '0' {
                zero += 1;
            } else {
                one += 1;
            }

            while (j - i + 1) > mid as usize {
                if s.chars().nth(i).unwrap() == '0' {
                    zero -= 1;
                } else {
                    one -= 1;
                }
                i += 1;
            }

            if (j - i + 1) == mid as usize {
                if zero == 0 || one == 0 {
                    intervals.push(vec![i, j]);
                }
            }
            j += 1;
        }

        let merged = Self::merge(&mut intervals, mid);
        merged.len() as i32 <= op
    }

    fn get_mini(s: &str, even: char, odd: char) -> i32 {
        let n = s.len();
        let mut op = 0;
        for (i, c) in s.chars().enumerate() {
            if i % 2 == 1 && c != odd {
                op += 1;
            } else if i % 2 == 0 && c != even {
                op += 1;
            }
        }
        op
    }

    fn min_length(s: &str, num_ops: i32) -> i32 {
        let n = s.len();
        let mut start = 3;
        let mut end = n;
        let mut res = 2;

        let mut op = n as i32 + 1;
        op = op.min(Self::get_mini(s, '0', '1'));
        op = op.min(Self::get_mini(s, '1', '0'));
        if op <= num_ops {
            return 1;
        }

        while start <= end {
            let mid = start + (end - start) / 2;
            if Self::is_poss(s, num_ops, mid as i32) {
                end = mid - 1;
            } else {
                res = mid as i32;
                start = mid + 1;
            }
        }
        res
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut s = String::new();
    stdin_lock.read_line(&mut s).expect("Failed to read line");
    s = s.trim().to_string();

    let mut num_ops_str = String::new();
    stdin_lock.read_line(&mut num_ops_str).expect("Failed to read line");
    let num_ops: i32 = num_ops_str.trim().parse().expect("Failed to parse number of operations");

    let result = Solution::min_length(&s, num_ops);

    println!("{}", result);
}