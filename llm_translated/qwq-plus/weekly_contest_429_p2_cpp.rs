use std::collections::HashSet;
use std::io;

struct Solution;

impl Solution {
    fn max_distinct_elements(arr: Vec<i32>, diff: i32) -> usize {
        let mut arr = arr;
        arr.sort_unstable();

        let mut prev = i32::MIN;
        let mut set = HashSet::new();

        for &current in &arr {
            let candidate_x = prev + 1;
            let min_x = current - diff;
            let x = candidate_x.max(min_x);

            if x <= current + diff {
                set.insert(x);
                prev = x;
            }
        }

        set.len()
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let numbers: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let n = numbers[0] as usize;
    let diff = numbers[1];
    let arr = &numbers[2..2 + n];
    let arr_vec = arr.to_vec();

    let solution = Solution;
    let result = solution.max_distinct_elements(arr_vec, diff);

    println!("{}", result);
}