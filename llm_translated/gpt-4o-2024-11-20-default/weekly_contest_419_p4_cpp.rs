use std::collections::{BTreeSet, HashMap};
use std::io::{self, BufRead};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Pair(i32, i32); // Struct equivalent of `pair<int, int>` in C++

struct Solution;

impl Solution {
    fn find_x_sum(nums: Vec<i32>, k: usize, x: usize) -> Vec<i64> {
        let mut l_set = BTreeSet::new(); // Left set
        let mut r_set = BTreeSet::new(); // Right set
        let mut sum_l = 0_i64;          // Sum of elements in `L`
        let mut count_map: HashMap<i32, i32> = HashMap::new(); // Equivalent of `unordered_map`

        // Helper functions using closures to add and remove elements
        let mut add = |value: i32| {
            let freq = *count_map.get(&value).unwrap_or(&0);
            let pair = Pair(freq, value);
            if freq > 0 {
                if !l_set.is_empty() && pair > *l_set.iter().next().unwrap() {
                    sum_l += freq as i64 * value as i64;
                    l_set.insert(pair);
                } else {
                    r_set.insert(pair);
                }
            }
        };

        let mut del = |value: i32| {
            let freq = *count_map.get(&value).unwrap_or(&0);
            let pair = Pair(freq, value);
            if freq > 0 {
                if l_set.remove(&pair) {
                    sum_l -= freq as i64 * value as i64;
                } else {
                    r_set.remove(&pair);
                }
            }
        };

        let mut l2r = || {
            if let Some(&p) = l_set.iter().next() {
                sum_l -= p.0 as i64 * p.1 as i64;
                l_set.remove(&p);
                r_set.insert(p);
            }
        };

        let mut r2l = || {
            if let Some(&p) = r_set.iter().rev().next() {
                sum_l += p.0 as i64 * p.1 as i64;
                r_set.remove(&p);
                l_set.insert(p);
            }
        };

        let mut ans = vec![0_i64; nums.len() - k + 1];
        for (r, &in_value) in nums.iter().enumerate() {
            del(in_value);
            *count_map.entry(in_value).or_default() += 1;
            add(in_value);

            let l = r + 1 - k;
            if l < 0 {
                continue;
            }

            // Adjust the sizes of L and R to match the requirement
            while !r_set.is_empty() && l_set.len() < x {
                r2l();
            }
            while l_set.len() > x {
                l2r();
            }

            ans[l] = sum_l;

            let out_value = nums[l];
            del(out_value);
            *count_map.entry(out_value).or_default() -= 1;
            add(out_value);
        }

        ans
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read k and x
    let first_line = lines.next().unwrap().unwrap();
    let mut first_parts = first_line.split_whitespace();
    let k: usize = first_parts.next().unwrap().parse().unwrap();
    let x: usize = first_parts.next().unwrap().parse().unwrap();

    // Read the number of elements in the array
    let second_line = lines.next().unwrap().unwrap();
    let nums_size: usize = second_line.parse().unwrap();

    // Read the array elements
    let third_line = lines.next().unwrap().unwrap();
    let nums: Vec<i32> = third_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let solution = Solution;
    let result = solution.find_x_sum(nums, k, x);

    // Print the output
    let result_str = result
        .iter()
        .map(|num| num.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    println!("{}", result_str);
}