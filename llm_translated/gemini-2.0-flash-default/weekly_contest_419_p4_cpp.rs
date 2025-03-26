use std::collections::{BTreeSet, HashMap};
use std::io;

struct Solution {}

impl Solution {
    pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i64> {
        // Define a type alias for pair of (count, value)
        type Pii = (i32, i32);

        // Use BTreeSet for L and R to maintain order and uniqueness
        let mut l: BTreeSet<Pii> = BTreeSet::new();
        let mut r: BTreeSet<Pii> = BTreeSet::new();

        let mut sum_l: i64 = 0;
        let mut cnt: HashMap<i32, i32> = HashMap::new();

        // Closure to add an element
        let mut add = |x: i32| {
            let p: Pii = (*cnt.get(&x).unwrap_or(&0), x);
            if p.0 == 0 {
                return;
            }
            if !l.is_empty() && p > *l.iter().next().unwrap() {
                sum_l += p.0 as i64 * p.1 as i64;
                l.insert(p);
            } else {
                r.insert(p);
            }
        };

        // Closure to delete an element
        let mut del = |x: i32| {
            let p: Pii = (*cnt.get(&x).unwrap_or(&0), x);
            if p.0 == 0 {
                return;
            }

            if l.contains(&p) {
                sum_l -= p.0 as i64 * p.1 as i64;
                l.remove(&p);
            } else {
                r.remove(&p);
            }
        };

        // Closure to move from L to R
        let mut l2r = || {
            let p = *l.iter().next().unwrap();
            sum_l -= p.0 as i64 * p.1 as i64;
            l.remove(&p);
            r.insert(p);
        };

        // Closure to move from R to L
        let mut r2l = || {
            let p = *r.iter().next_back().unwrap();
            sum_l += p.0 as i64 * p.1 as i64;
            r.remove(&p);
            l.insert(p);
        };

        let mut ans: Vec<i64> = Vec::new();
        for _ in 0..(nums.len() as i32 - k + 1) {
            ans.push(0);
        }

        for r_idx in 0..nums.len() {
            let r = r_idx as usize;
            let in_val = nums[r];
            del(in_val);
            *cnt.entry(in_val).or_insert(0) += 1;
            add(in_val);

            let l_idx = r as i32 + 1 - k;
            if l_idx < 0 {
                continue;
            }
            let l_idx_usize = l_idx as usize;

            while !r.is_empty() && l.len() < x as usize {
                r2l();
            }
            while l.len() > x as usize {
                l2r();
            }
            ans[l_idx_usize] = sum_l;

            let out_val = nums[l_idx_usize];
            del(out_val);
            *cnt.entry(out_val).or_insert(0) -= 1;
            add(out_val);
        }
        return ans;
    }
}

fn main() {
    let mut k = String::new();
    io::stdin().read_line(&mut k).expect("Failed to read line");
    let mut k_x = k.trim().split_whitespace();
    let k: i32 = k_x.next().unwrap().parse().unwrap();
    let x: i32 = k_x.next().unwrap().parse().unwrap();

    let mut nums_size_str = String::new();
    io::stdin()
        .read_line(&mut nums_size_str)
        .expect("Failed to read line");
    let nums_size: usize = nums_size_str.trim().parse().unwrap();

    let mut nums_str = String::new();
    io::stdin().read_line(&mut nums_str).expect("Failed to read line");
    let nums: Vec<i32> = nums_str
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let s = Solution {};
    let res = s.find_x_sum(nums, k, x);

    for i in 0..res.len() {
        print!("{} ", res[i]);
    }
    println!();
}