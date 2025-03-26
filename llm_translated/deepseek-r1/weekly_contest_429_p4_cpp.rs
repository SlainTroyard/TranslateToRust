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
        let s_chars: Vec<char> = s.chars().collect();
        let n = s_chars.len();
        let mut intervals = Vec::new();
        let mut i = 0;
        let mut j = 0;
        let mut zero = 0;
        let mut one = 0;

        while j < n {
            if s_chars[j] == '0' {
                zero += 1;
            } else {
                one += 1;
            }

            // Shrink the window from the left if it's larger than mid
            while (j - i + 1) > mid as usize {
                if s_chars[i] == '0' {
                    zero -= 1;
                } else {
                    one -= 1;
                }
                i += 1;
            }

            // Check if the current window size is exactly mid
            if (j - i + 1) == mid as usize {
                if zero == 0 || one == 0 {
                    intervals.push(vec![i as i32, j as i32]);
                }
            }

            j += 1;
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

        // Check if we can achieve length 1 by making the string alternate
        let op1 = Self::get_mini(&s, '0', '1');
        let op2 = Self::get_mini(&s, '1', '0');
        let op = op1.min(op2);
        if op <= num_ops {
            return 1;
        }

        let mut start = 3;
        let mut end = n as i32;
        let mut res = 2; // Default result if all checks fail

        while start <= end {
            let mid = start + (end - start) / 2; // Prevent potential overflow
            let curr = Self::is_poss(&s, num_ops, mid);
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
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();
    let s = tokens.next().expect("binary string expected").to_string();
    let num_ops: i32 = tokens.next().expect("number of operations expected").parse().unwrap();

    let result = Solution::min_length(s, num_ops);
    println!("{}", result);
}