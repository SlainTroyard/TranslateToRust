// Problem: Weekly Contest 419 Problem 4
// -------------------------------------
// This Rust program replicates the exact logic of the provided C++ solution.
// It reads the same input format and produces the same output format.
//
// The input format must be (all separated by whitespace, possibly across lines):
//   k x
//   numsSize
//   nums[0] nums[1] ... nums[numsSize-1]
//
// The program outputs the resulting vector, separated by spaces, followed by a newline.

use std::collections::{BTreeSet, HashMap};
use std::io::{self, Read};

// A simple struct to hold (count, value) pairs and provide an ordering
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
struct Pair {
    count: i32,
    value: i32,
}

// We implement the same algorithm as in the original C++ code
struct Solution;

impl Solution {
    fn find_x_sum(&self, nums: &Vec<i32>, k: usize, x: usize) -> Vec<i64> {
        let mut L = BTreeSet::new();
        let mut R = BTreeSet::new();
        let mut sum_l: i64 = 0;
        let mut cnt = HashMap::<i32, i32>::new();

        // Function to remove an element (with its current count) from L or R
        fn del(
            val: i32,
            L: &mut BTreeSet<Pair>,
            R: &mut BTreeSet<Pair>,
            sum_l: &mut i64,
            cnt: &HashMap<i32, i32>,
        ) {
            let c = *cnt.get(&val).unwrap_or(&0);
            if c == 0 {
                return;
            }
            let p = Pair { count: c, value: val };
            if L.contains(&p) {
                *sum_l -= p.count as i64 * p.value as i64;
                L.remove(&p);
            } else {
                R.remove(&p);
            }
        }

        // Function to add an element (with its current count) to L or R
        fn add(
            val: i32,
            L: &mut BTreeSet<Pair>,
            R: &mut BTreeSet<Pair>,
            sum_l: &mut i64,
            cnt: &HashMap<i32, i32>,
        ) {
            let c = *cnt.get(&val).unwrap_or(&0);
            if c == 0 {
                return;
            }
            let p = Pair { count: c, value: val };
            // Mirror the original C++ logic:
            // if (!L.empty() && p > *L.begin()) then place it in L (and add to sum_l)
            // else place it in R
            if !L.is_empty() && p > *L.iter().next().unwrap() {
                *sum_l += p.count as i64 * p.value as i64;
                L.insert(p);
            } else {
                R.insert(p);
            }
        }

        // Move the smallest element in L to R (and adjust sum_l)
        fn l2r(L: &mut BTreeSet<Pair>, R: &mut BTreeSet<Pair>, sum_l: &mut i64) {
            if let Some(&p) = L.iter().next() {
                *sum_l -= p.count as i64 * p.value as i64;
                L.remove(&p);
                R.insert(p);
            }
        }

        // Move the largest element in R to L (and adjust sum_l)
        fn r2l(L: &mut BTreeSet<Pair>, R: &mut BTreeSet<Pair>, sum_l: &mut i64) {
            if let Some(&p) = R.iter().last() {
                *sum_l += p.count as i64 * p.value as i64;
                R.remove(&p);
                L.insert(p);
            }
        }

        let n = nums.len();
        // We need space for (n - k + 1) outputs
        let mut ans = vec![0i64; n - k + 1];

        // Slide the window over nums
        for r in 0..n {
            let in_val = nums[r];
            // Remove in_val's old pair (if it existed with an old count)
            del(in_val, &mut L, &mut R, &mut sum_l, &cnt);
            // Increase the count
            *cnt.entry(in_val).or_insert(0) += 1;
            // Add the new pair with the updated count
            add(in_val, &mut L, &mut R, &mut sum_l, &cnt);

            // Once we have a window of size k: [r-k+1, r]
            let l = r as isize + 1 - k as isize;
            if l < 0 {
                continue;
            }

            // Ensure L has exactly x elements (or as many as possible up to x)
            while !R.is_empty() && L.len() < x {
                r2l(&mut L, &mut R, &mut sum_l);
            }
            while L.len() > x {
                l2r(&mut L, &mut R, &mut sum_l);
            }

            // Record the result
            ans[l as usize] = sum_l;

            // Remove the oldest element in the window from our sets
            let out_val = nums[l as usize];
            del(out_val, &mut L, &mut R, &mut sum_l, &cnt);
            *cnt.entry(out_val).or_insert(0) -= 1;
            add(out_val, &mut L, &mut R, &mut sum_l, &cnt);
        }

        ans
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read all input from stdin as whitespace-separated tokens
    let mut input_str = String::new();
    io::stdin().read_to_string(&mut input_str)?;

    let mut tokens = input_str.split_whitespace();
    // Parse k, x
    let k: usize = tokens
        .next()
        .expect("Expected k")
        .parse()
        .expect("Failed to parse k");
    let x: usize = tokens
        .next()
        .expect("Expected x")
        .parse()
        .expect("Failed to parse x");

    // Parse numsSize
    let nums_size: usize = tokens
        .next()
        .expect("Expected numsSize")
        .parse()
        .expect("Failed to parse numsSize");

    // Parse the array nums
    let mut nums = Vec::with_capacity(nums_size);
    for _ in 0..nums_size {
        let val: i32 = tokens
            .next()
            .expect("Expected a number in nums")
            .parse()
            .expect("Failed to parse an element of nums");
        nums.push(val);
    }

    // Solve
    let solver = Solution;
    let result = solver.find_x_sum(&nums, k, x);

    // Output the results in the same format as the original C++ code
    for i in 0..result.len() {
        print!("{} ", result[i]);
    }
    println!();

    Ok(())
}