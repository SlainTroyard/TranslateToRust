use std::cmp;
use std::io::{self, BufRead};
use std::iter::FromIterator;

struct Solution;

impl Solution {
    pub fn gcd_values(nums: Vec<usize>, queries: Vec<i64>) -> Vec<usize> {
        let mx = *nums.iter().max().unwrap();
        let mut cnt_x = vec![0; mx + 1];
        for &x in nums.iter() {
            cnt_x[x] += 1;
        }

        let mut cnt_gcd = vec![0i64; mx + 1];
        for i in (1..=mx).rev() {
            let mut c = 0;
            for j in (i..=mx).step_by(i) {
                c += cnt_x[j];
                cnt_gcd[i] -= cnt_gcd[j];
            }
            cnt_gcd[i] += (c as i64 * (c as i64 - 1)) / 2;
        }

        // Apply cumulative sum (partial_sum equivalent)
        for i in 1..=mx {
            cnt_gcd[i] += cnt_gcd[i - 1];
        }

        // Answer queries
        queries
            .iter()
            .map(|&q| {
                match cnt_gcd.iter().position(|&val| val > q) {
                    Some(pos) => pos,
                    None => mx + 1, // If no element satisfies the condition
                }
            })
            .collect()
    }
}

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines();

    // Read number of elements in nums
    let n: usize = input
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .expect("Failed to parse number of elements");

    // Read nums vector
    let nums: Vec<usize> = input
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to parse number"))
        .collect();

    // Read number of queries
    let q: usize = input
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .expect("Failed to parse number of queries");

    // Read queries vector
    let queries: Vec<i64> = input
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to parse query"))
        .collect();

    // Compute the answer
    let solution = Solution;
    let ans = solution.gcd_values(nums, queries);

    // Print the output
    let output = Vec::from_iter(ans.iter().map(|x| x.to_string()));
    println!("{}", output.join(" "));
}