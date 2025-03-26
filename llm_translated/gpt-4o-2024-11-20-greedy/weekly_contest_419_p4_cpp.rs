use std::collections::{BTreeSet, HashMap};
use std::io::{self, BufRead};

type Pair = (i32, i32);

struct Solution;

impl Solution {
    pub fn find_x_sum(nums: Vec<i32>, k: usize, x: usize) -> Vec<i64> {
        let mut l_set: BTreeSet<Pair> = BTreeSet::new(); // Left set
        let mut r_set: BTreeSet<Pair> = BTreeSet::new(); // Right set
        let mut sum_l: i64 = 0; // Sum of elements in the left set
        let mut cnt: HashMap<i32, i32> = HashMap::new(); // Frequency map

        // Helper function to add an element
        let mut add = |val: i32| {
            let freq = *cnt.get(&val).unwrap_or(&0);
            let pair = (freq, val);
            if freq == 0 {
                return;
            }
            if !l_set.is_empty() && pair > *l_set.iter().next().unwrap() {
                sum_l += (freq as i64) * (val as i64);
                l_set.insert(pair);
            } else {
                r_set.insert(pair);
            }
        };

        // Helper function to remove an element
        let mut del = |val: i32| {
            let freq = *cnt.get(&val).unwrap_or(&0);
            let pair = (freq, val);
            if freq == 0 {
                return;
            }
            if l_set.remove(&pair) {
                sum_l -= (freq as i64) * (val as i64);
            } else {
                r_set.remove(&pair);
            }
        };

        // Move the smallest element from L to R
        let mut l2r = || {
            if let Some(&pair) = l_set.iter().next() {
                sum_l -= (pair.0 as i64) * (pair.1 as i64);
                l_set.remove(&pair);
                r_set.insert(pair);
            }
        };

        // Move the largest element from R to L
        let mut r2l = || {
            if let Some(&pair) = r_set.iter().next_back() {
                sum_l += (pair.0 as i64) * (pair.1 as i64);
                r_set.remove(&pair);
                l_set.insert(pair);
            }
        };

        let mut ans = vec![0; nums.len() - k + 1];
        for r in 0..nums.len() {
            let in_val = nums[r];
            del(in_val);
            *cnt.entry(in_val).or_insert(0) += 1;
            add(in_val);

            let l = r + 1 - k;
            if l < 0 {
                continue;
            }

            while !r_set.is_empty() && l_set.len() < x {
                r2l();
            }
            while l_set.len() > x {
                l2r();
            }
            ans[l] = sum_l;

            let out_val = nums[l];
            del(out_val);
            *cnt.entry(out_val).or_insert(0) -= 1;
            add(out_val);
        }
        ans
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read k and x
    let first_line = lines.next().unwrap().unwrap();
    let mut first_iter = first_line.split_whitespace();
    let k: usize = first_iter.next().unwrap().parse().unwrap();
    let x: usize = first_iter.next().unwrap().parse().unwrap();

    // Read nums size
    let second_line = lines.next().unwrap().unwrap();
    let nums_size: usize = second_line.parse().unwrap();

    // Read nums array
    let third_line = lines.next().unwrap().unwrap();
    let nums: Vec<i32> = third_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Solve the problem
    let solution = Solution;
    let result = solution.find_x_sum(nums, k, x);

    // Print the result
    for (i, val) in result.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", val);
    }
    println!();
}