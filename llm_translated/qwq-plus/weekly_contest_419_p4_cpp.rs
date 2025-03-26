use std::collections::{BTreeSet, HashMap};

struct Solution;

impl Solution {
    pub fn find_x_sum(nums: Vec<i32>, k: usize, x: usize) -> Vec<i64> {
        let n = nums.len();
        let mut ans = vec![0; n - k + 1];
        let mut L = BTreeSet::new();
        let mut R = BTreeSet::new();
        let mut sum_l: i64 = 0;
        let mut cnt = HashMap::new();

        for r in 0..n {
            let in_val = nums[r];

            // Step 1: del the previous count of in_val
            {
                let prev_count = *cnt.get(&in_val).unwrap_or(&0);
                let p = (prev_count, in_val);
                if prev_count > 0 {
                    if L.contains(&p) {
                        sum_l -= (p.0 as i64) * (p.1 as i64);
                        L.remove(&p);
                    } else {
                        R.remove(&p);
                    }
                }
            }

            // Step 2: increment the count
            *cnt.entry(in_val).or_insert(0) += 1;

            // Step 3: add the new count
            let new_count = cnt.get(&in_val).unwrap();
            let new_p = (*new_count, in_val);
            if new_count == &0 {
                panic!("Unexpected zero count after increment");
            }

            if !L.is_empty() {
                let first_l = L.iter().next().unwrap();
                if new_p > *first_l {
                    sum_l += (new_p.0 as i64) * (new_p.1 as i64);
                    L.insert(new_p);
                } else {
                    R.insert(new_p);
                }
            } else {
                R.insert(new_p);
            }

            // Check window validity
            let l = r + 1 - k;
            if l < 0 {
                continue;
            }

            // Adjust L and R to have exactly x elements in L
            // Move from R to L until L has x elements or R is empty
            while !R.is_empty() && L.len() < x {
                let p = R.iter().next_back().unwrap().clone();
                sum_l += (p.0 as i64) * (p.1 as i64);
                R.remove(&p);
                L.insert(p);
            }

            // If L has more than x elements, move the smallest to R
            while L.len() > x {
                let p = L.iter().next().unwrap().clone();
                sum_l -= (p.0 as i64) * (p.1 as i64);
                L.remove(&p);
                R.insert(p);
            }

            // Record the answer
            ans[l] = sum_l;

            // Process outgoing element (nums[l])
            let out_val = nums[l as usize];

            // Step 1: del the previous count of out_val
            {
                let prev_count = *cnt.get(&out_val).unwrap();
                let p = (prev_count, out_val);
                if L.contains(&p) {
                    sum_l -= (p.0 as i64) * (p.1 as i64);
                    L.remove(&p);
                } else {
                    R.remove(&p);
                }
            }

            // Step 2: decrement the count
            *cnt.entry(out_val).or_insert(0) -= 1;

            // Step 3: add the new count
            let new_count = cnt.get(&out_val).unwrap_or(&0);
            let new_p = (*new_count, out_val);
            if *new_count == 0 {
                // do nothing
            } else {
                if !L.is_empty() {
                    let first_l = L.iter().next().unwrap();
                    if new_p > *first_l {
                        sum_l += (new_p.0 as i64) * (new_p.1 as i64);
                        L.insert(new_p);
                    } else {
                        R.insert(new_p);
                    }
                } else {
                    R.insert(new_p);
                }
            }
        }

        ans
    }
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let k: usize = parts.next().unwrap().parse().unwrap();
    let x: usize = parts.next().unwrap().parse().unwrap();

    let nums_size_line = lines.next().unwrap().unwrap();
    let nums_size: usize = nums_size_line.parse().unwrap();

    let nums_line = lines.next().unwrap().unwrap();
    let nums: Vec<i32> = nums_line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    assert_eq!(nums.len(), nums_size);

    let res = Solution::find_x_sum(nums, k, x);

    for num in res {
        print!("{} ", num);
    }
    println!();
}