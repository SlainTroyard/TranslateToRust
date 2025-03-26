use std::cmp::{max, min};
use std::io::{self, Write};

struct Solution;

impl Solution {
    // Merges overlapping intervals based on the given length constraint
    fn merge(intervals: &mut Vec<Vec<usize>>, len: usize) -> Vec<Vec<usize>> {
        if intervals.is_empty() {
            return vec![];
        }

        let mut res = vec![intervals[0].clone()];

        for i in 1..intervals.len() {
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

    // Checks if it's possible to achieve the desired result with the given constraints
    fn is_poss(s: &str, op: usize, mid: usize) -> bool {
        let n = s.len();
        let mut i = 0;
        let mut j = 0;
        let mut zero = 0;
        let mut one = 0;
        let mut intervals = vec![];

        let chars: Vec<char> = s.chars().collect();

        while j < n {
            if chars[j] == '0' {
                zero += 1;
            } else {
                one += 1;
            }

            while (j - i + 1) > mid {
                if chars[i] == '0' {
                    zero -= 1;
                } else {
                    one -= 1;
                }
                i += 1;
            }

            if (j - i + 1) == mid {
                if zero == 0 || one == 0 {
                    intervals.push(vec![i, j]);
                }
            }
            j += 1;
        }

        let merged = Self::merge(&mut intervals, mid);
        merged.len() <= op
    }

    // Calculates the minimum operations needed to make the string alternate with given characters
    fn get_mini(s: &str, even: char, odd: char) -> usize {
        let n = s.len();
        let mut op = 0;
        let chars: Vec<char> = s.chars().collect();

        for i in 0..n {
            if i % 2 == 1 && chars[i] != odd {
                op += 1;
            } else if i % 2 == 0 && chars[i] != even {
                op += 1;
            }
        }

        op
    }

    // Finds the minimum length of a substring that can be made alternating with the given constraints
    fn min_length(s: &str, num_ops: usize) -> usize {
        let n = s.len();
        let mut start = 3;
        let mut end = n;
        let mut res = 2;

        let mut op = n + 1;
        op = min(op, Self::get_mini(s, '0', '1'));
        op = min(op, Self::get_mini(s, '1', '0'));

        if op <= num_ops {
            return 1;
        }

        while start <= end {
            let mid = start + (end - start) / 2;
            if Self::is_poss(s, num_ops, mid) {
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
    io::stdin().read_line(&mut input).unwrap();
    let s = input.trim().to_string();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let num_ops: usize = input.trim().parse().unwrap();

    let solution = Solution;
    let result = solution.min_length(&s, num_ops);

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    writeln!(handle, "{}", result).unwrap();
}