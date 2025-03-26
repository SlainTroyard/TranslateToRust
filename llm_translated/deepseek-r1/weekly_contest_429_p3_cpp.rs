use std::io::{self, Read};

struct Solution;

impl Solution {
    fn merge(intervals: &[Vec<i32>], len: i32) -> Vec<Vec<i32>> {
        if intervals.is_empty() {
            return Vec::new();
        }
        let mut res = vec![intervals[0].clone()];
        for interval in intervals.iter().skip(1) {
            let curr_start = interval[0];
            let curr_end = interval[1];
            let last = res.last_mut().unwrap();
            if curr_start <= last[1] && (curr_start - last[0] + 1) <= len {
                last[1] = last[1].max(curr_end);
            } else {
                res.push(interval.clone());
            }
        }
        res
    }

    fn is_poss(s: &str, op: i32, mid: i32) -> bool {
        let n = s.len();
        let s_bytes = s.as_bytes();
        let mut i = 0;
        let mut zero = 0;
        let mut one = 0;
        let mut intervals = Vec::new();

        for j in 0..n {
            let c = s_bytes[j];
            if c == b'0' {
                zero += 1;
            } else {
                one += 1;
            }

            // Adjust window start to maintain size <= mid
            while (j as i32 - i as i32 + 1) > mid {
                let c_i = s_bytes[i];
                if c_i == b'0' {
                    zero -= 1;
                } else {
                    one -= 1;
                }
                i += 1;
            }

            // Check if current window is of size mid and all 0s or 1s
            if (j as i32 - i as i32 + 1) == mid {
                if zero == 0 || one == 0 {
                    intervals.push(vec![i as i32, j as i32]);
                }
            }
        }

        let merged = Self::merge(&intervals, mid);
        merged.len() as i32 <= op
    }

    fn get_mini(s: &str, even: char, odd: char) -> i32 {
        let mut op = 0;
        for (idx, c) in s.chars().enumerate() {
            if idx % 2 == 1 {
                if c != odd {
                    op += 1;
                }
            } else {
                if c != even {
                    op += 1;
                }
            }
        }
        op
    }

    fn min_length(s: String, num_ops: i32) -> i32 {
        let n = s.len();
        let min_op = Self::get_mini(&s, '0', '1').min(Self::get_mini(&s, '1', '0'));
        if min_op <= num_ops {
            return 1;
        }

        let mut start = 3;
        let mut end = n as i32;
        let mut res = 2;

        while start <= end {
            let mid = start + (end - start) / 2;
            if Self::is_poss(&s, num_ops, mid) {
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
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut parts = input.split_whitespace();
    let s = parts.next().unwrap().to_string();
    let num_ops: i32 = parts.next().unwrap().parse().unwrap();

    println!("{}", Solution::min_length(s, num_ops));
}