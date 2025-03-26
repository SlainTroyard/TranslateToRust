use std::io;

struct Solution;

impl Solution {
    fn merge(&self, intervals: Vec<Vec<i32>>, len: i32) -> Vec<Vec<i32>> {
        if intervals.is_empty() {
            return Vec::new();
        }
        
        let mut res = Vec::new();
        res.push(intervals[0].clone());
        
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
    
    fn is_poss(&self, s: &str, op: i32, mid: i32) -> bool {
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
                    intervals.push(vec![i as i32, j as i32]);
                }
            }
            j += 1;
        }
        
        let merged = self.merge(intervals, mid);
        merged.len() as i32 <= op
    }
    
    fn get_mini(&self, s: &str, even: char, odd: char) -> i32 {
        let n = s.len();
        let mut op = 0;
        
        for i in 0..n {
            let c = s.chars().nth(i).unwrap();
            if i % 2 == 1 {
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
    
    fn min_length(&self, s: String, num_ops: i32) -> i32 {
        let n = s.len();
        if n == 0 {
            return 0;
        }
        
        let mut start = 3;
        let mut end = n as i32;
        let mut res = 2;
        
        let mut min_ops = n as i32 + 1;
        min_ops = min_ops.min(self.get_mini(&s, '0', '1'));
        min_ops = min_ops.min(self.get_mini(&s, '1', '0'));
        
        if min_ops <= num_ops {
            return 1;
        }
        
        while start <= end {
            let mid = start + (end - start) / 2;
            let curr = self.is_poss(&s, num_ops, mid);
            
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
    let mut s = String::new();
    let mut num_ops = String::new();
    
    io::stdin().read_line(&mut s).unwrap();
    io::stdin().read_line(&mut num_ops).unwrap();
    
    s = s.trim().to_string();
    num_ops = num_ops.trim().parse::<i32>().unwrap();
    
    let solution = Solution;
    let result = solution.min_length(s, num_ops);
    
    println!("{}", result);
}