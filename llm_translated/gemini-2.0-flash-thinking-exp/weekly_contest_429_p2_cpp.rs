use std::io;
use std::collections::HashSet;
use std::cmp::max;

struct Solution {}

impl Solution {
    pub fn max_distinct_elements(&self, arr: &mut Vec<i32>, diff: i32) -> i32 {
        let mut prev = i32::min_value();
        let mut c: HashSet<i32> = HashSet::new();
        arr.sort();
        for &val in arr.iter() {
            let x = max(prev + 1, val - diff);
            if x <= val + diff {
                c.insert(x);
                prev = x;
            }
        }
        c.len() as i32
    }
}

fn main() {
    let solution = Solution {};

    // Input array size and difference
    let mut n_str = String::new();
    io::stdin().read_line(&mut n_str).expect("Failed to read line");
    let n: i32 = n_str.trim().parse().expect("Invalid input for n");

    let mut diff_str = String::new();
    io::stdin().read_line(&mut diff_str).expect("Failed to read line");
    let diff: i32 = diff_str.trim().parse().expect("Invalid input for diff");

    // Input array elements
    let mut arr: Vec<i32> = Vec::new();
    for _ in 0..n {
        let mut num_str = String::new();
        io::stdin().read_line(&mut num_str).expect("Failed to read line");
        let num: i32 = num_str.trim().parse().expect("Invalid input for array element");
        arr.push(num);
    }

    // Compute the result
    let result = solution.max_distinct_elements(&mut arr, diff);

    // Output the result
    println!("{}", result);
}