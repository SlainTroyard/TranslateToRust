use std::cmp::max;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the binary string
    let s = lines.next().unwrap().unwrap();
    
    // Read the number of operations
    let num_ops = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
    
    // Compute the result
    let result = Solution::min_length(&s, num_ops);
    
    // Output the result
    println!("{}", result);
}

struct Solution;

impl Solution {
    fn merge(intervals: Vec<Vec<i32>>, len: i32) -> Vec<Vec<i32>> {
        if intervals.is_empty() {
            return vec![];
        }
        let n = intervals.len();
        let mut res = vec![];
        res.push(intervals[0].clone());
        
        for i in 1..n {
            let curr_start = intervals[i][0];
            let curr_end = intervals[i][1];
            
            if curr_start <= res.last().unwrap()[1] && (curr_start - res.last().unwrap()[0] + 1) <= len {
                let last = res.last_mut().unwrap();
                last[1] = max(last[1], curr_end);
            } else {
                res.push(intervals[i].clone());
            }
        }
        res
    }

    fn is_poss(s: &str, op: usize, mid: usize) -> bool {
        let n = s.len();
        let mut i = 0;
        let mut j = 0;
        let mut zero = 0;
        let mut one = 0;
        let mut intervals = vec![];
        
        while j < n {
            if s.chars().nth(j).unwrap() == '0' {
                zero += 1;
            } else {
                one += 1;
            }
            
            while (j - i + 1) > mid {
                if s.chars().nth(i).unwrap() == '0' {
                    zero -= 1;
                } else {
                    one -= 1;
                }
                i += 1;
            }
            
            if (j - i + 1) == mid {
                if zero == 0 || one == 0 {
                    intervals.push(vec![i as i32, j as i32]);
                }
            }
            j += 1;
        }
        
        let merged = Self::merge(intervals, mid as i32);
        merged.len() <= op
    }

    fn get_mini(s: &str, even: char, odd: char) -> usize {
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

    fn min_length(s: &str, num_ops: usize) -> usize {
        let n = s.len();
        let mut start = 3;
        let mut end = n;
        let mut res = 2;
        
        let mut op = n + 1;
        op = op.min(Self::get_mini(s, '0', '1'));
        op = op.min(Self::get_mini(s, '1', '0'));
        if op <= num_ops {
            return 1;
        }
        
        while start <= end {
            let mid = start + (end - start) / 2;
            let curr = Self::is_poss(s, num_ops, mid);
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