use std::io;
use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn max_distinct_elements(arr: &mut Vec<i32>, diff: i32) -> i32 {
        let mut prev = i32::min_value();
        let mut c: HashSet<i32> = HashSet::new();
        arr.sort();

        for &val in arr.iter() {
            let x = prev.max(val - diff);

            if x <= val + diff {
                c.insert(x);
                prev = x;
            }
        }
        c.len() as i32
    }
}

fn main() {
    let mut n_str = String::new();
    io::stdin().read_line(&mut n_str).expect("Failed to read line");
    let n: i32 = n_str.trim().parse().expect("Invalid input");

    let mut diff_str = String::new();
    io::stdin().read_line(&mut diff_str).expect("Failed to read line");
    let diff: i32 = diff_str.trim().parse().expect("Invalid input");

    let mut arr_str = String::new();
    io::stdin().read_line(&mut arr_str).expect("Failed to read line");
    let arr: Vec<i32> = arr_str
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid number"))
        .collect();

    let mut arr_mut = arr.clone();
    let result = Solution {}.max_distinct_elements(&mut arr_mut, diff);

    println!("{}", result);
}