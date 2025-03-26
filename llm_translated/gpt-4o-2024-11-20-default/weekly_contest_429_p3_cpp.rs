use std::io::{self, Write};
use std::cmp::{min, max};

struct Solution;

impl Solution {
    // Merges overlapping intervals with additional constraints
    fn merge(intervals: Vec<Vec<usize>>, len: usize) -> Vec<Vec<usize>> {
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

    // Determines if a given mid-length window can be split into valid intervals within `op` operations
    fn is_poss(s: &str, op: usize, mid: usize) -> bool {
        let n = s.len();
        let mut i = 0;
        let mut j = 0;
        let mut zero_count = 0;
        let mut one_count = 0;
        let mut intervals = vec![];

        while j < n {
            if s.chars().nth(j).unwrap() == '0' {
                zero_count += 1;
            } else {
                one_count += 1;
            }

            while (j - i + 1) > mid {
                if s.chars().nth(i).unwrap() == '0' {
                    zero_count -= 1;
                } else {
                    one_count -= 1;
                }
                i += 1;
            }

            if (j - i + 1) == mid && (zero_count == 0 || one_count == 0) {
                intervals.push(vec![i, j]);
            }

            j += 1;
        }

        let merged = Self::merge(intervals, mid);
        merged.len() <= op
    }

    // Calculates the minimum number of operations to convert the string to alternating characters
    fn get_mini(s: &str, even: char, odd: char) -> usize {
        let n = s.len();
        let mut op = 0;

        for (i, c) in s.chars().enumerate() {
            if (i % 2 == 0 && c != even) || (i % 2 == 1 && c != odd) {
                op += 1;
            }
        }

        op
    }

    // Main function to find the minimum length of a valid substring under given constraints
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
    // Input handling
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let s = input.trim().to_string();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let num_ops: usize = input.trim().parse().unwrap();

    // Create a solution instance and compute the result
    let solution = Solution;
    let result = solution.min_length(&s, num_ops);

    // Output the result
    writeln!(io::stdout(), "{}", result).unwrap();
}