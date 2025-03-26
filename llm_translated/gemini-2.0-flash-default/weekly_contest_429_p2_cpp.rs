use std::io;
use std::io::Read;
use std::cmp::{max, min};
use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn max_distinct_elements(arr: &mut Vec<i32>, diff: i32) -> i32 {
        let mut prev = i32::min_value();
        let mut c: HashSet<i32> = HashSet::new();
        arr.sort();
        for i in 0..arr.len() {
            let x = max(prev + 1, arr[i] - diff);

            if x <= arr[i] + diff {
                c.insert(x);
                prev = x;
            }
        }
        c.len() as i32
    }
}

fn main() {
    let solution = Solution {};

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.lines();

    let n = lines.next().unwrap().parse::<i32>().unwrap();
    let diff = lines.next().unwrap().parse::<i32>().unwrap();

    let arr_str = lines.next().unwrap();
    let arr: Vec<i32> = arr_str
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let mut arr_mut = arr.clone(); // Create a mutable copy

    let result = solution.max_distinct_elements(&mut arr_mut, diff);

    println!("{}", result);
}