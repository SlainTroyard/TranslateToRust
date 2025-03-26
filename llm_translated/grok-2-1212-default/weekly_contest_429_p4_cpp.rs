use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn merge(intervals: &mut Vec<Vec<i32>>, len: i32) -> Vec<Vec<i32>> {
        if intervals.is_empty() {
            return Vec::new();
        }

        let mut res = vec![intervals[0].clone()];

        for i in 1..intervals.len() {
            let curr_start = intervals[i][0];
            let curr_end = intervals[i][1];

            if curr_start <= res.last().unwrap()[1] && (curr_start - res.last().unwrap()[0] + 1) <= len {
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

            while (j as i32 - i as i32 + 1) > mid {
                if s.chars().nth(i).unwrap() == '0' {
                    zero -= 1;
                } else {
                    one -= 1;
                }
                i += 1;
            }

            if (j as i32 - i as i32 + 1) == mid {
                if zero == 0 || one == 0 {
                    intervals.push(vec![i as i32, j as i32]);
                }
            }
            j += 1;
        }

        let merged = Solution::merge(&mut intervals, mid);
        merged.len() as i32 <= op
    }

    fn get_mini(s: &str, even: char, odd: char) -> i32 {
        let n = s.len();
        let mut op = 0;
        for i in 0..n {
            if i % 2 == 1 && s.chars().nth(i).unwrap() != odd {
                op += 1;
            } else if i % 2 == 0 && s.chars().nth(i).unwrap() != even {
                op += 1;
            }
        }
        op
    }

    fn min_length(s: String, num_ops: i32) -> i32 {
        let n = s.len();
        let mut start = 3;
        let mut end = n;
        let mut res = 2;

        let mut op = n as i32 + 1;
        op = op.min(Solution::get_mini(&s, '0', '1'));
        op = op.min(Solution::get_mini(&s, '1', '0'));
        if op <= num_ops {
            return 1;
        }

        while start <= end {
            let mid = start + (end - start) / 2;
            let curr = Solution::is_poss(&s, num_ops, mid as i32);
            if curr {
                end = mid - 1;
            } else {
                res = mid;
                start = mid + 1;
            }
        }
        res as i32
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Input the binary string and the number of operations
    let s = lines.next().unwrap()?.trim().to_string();
    let num_ops: i32 = lines.next().unwrap()?.trim().parse().unwrap();

    // Compute the result
    let result = Solution::min_length(s, num_ops);

    // Output the result
    println!("{}", result);

    Ok(())
}