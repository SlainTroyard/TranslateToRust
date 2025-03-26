use std::io;
use std::io::Read;
use std::cmp::min;

struct Solution {}

impl Solution {
    pub fn min_max_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        // Closure to calculate the contribution based on the length m
        let count = |m: i32| -> i64 {
            if m > k {
                (m as i64 * 2 - k as i64 + 1) * k as i64 / 2
            } else {
                (m as i64 + 1) * m as i64 / 2
            }
        };

        // Closure to calculate the sum of subarray mins/maxes
        let sum_subarray_mins = |nums: &Vec<i32>| -> i64 {
            let mut res: i64 = 0;
            let mut st: Vec<usize> = vec![];
            st.push(0);
            let mut nums_with_padding = nums.clone();
            for r in 1..nums_with_padding.len() {
                while st.len() > 1 && nums_with_padding[st.last().unwrap()] >= nums_with_padding[r] {
                    let i = st.pop().unwrap();
                    let l = *st.last().unwrap();

                    let cnt = count((r - l - 1) as i32) - count((i - l - 1) as i32) - count((r - i - 1) as i32);
                    res += nums_with_padding[i] as i64 * cnt;
                }
                st.push(r);
            }
            res
        };

        let mut nums_modified = nums.clone();
        nums_modified.insert(0, i32::min_value() / 2);

        let ans_max = sum_subarray_mins(&nums_modified);

        let mut nums_negated = nums.clone();
        for x in &mut nums_negated {
            *x = -*x;
        }
        nums_negated.insert(0, i32::max_value() / 2);
        let ans_min = sum_subarray_mins(&nums_negated);

        ans_max - ans_min
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.lines();
    let first_line = lines.next().unwrap();
    let mut split = first_line.split_whitespace();
    let n: i32 = split.next().unwrap().parse().unwrap();
    let k: i32 = split.next().unwrap().parse().unwrap();

    let second_line = lines.next().unwrap();
    let nums: Vec<i32> = second_line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let sol = Solution {};
    println!("{}", sol.min_max_subarray_sum(nums, k));
}