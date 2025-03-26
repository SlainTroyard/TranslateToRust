use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

fn get_largest_outlier(nums: &Vec<i32>) -> i32 {
    let mut ctr: HashMap<i32, i32> = HashMap::new();
    let mut sm: i32 = 0;

    for &num in nums {
        *ctr.entry(num).or_insert(0) += 1;
        sm += num;
    }

    let mut cands: Vec<i32> = ctr.keys().cloned().collect();
    cands.sort_by(|a, b| b.cmp(a)); // Sort in descending order

    for &n in &cands {
        let d = (sm - n) / 2;
        let m = (sm - n) % 2;
        if m == 0 {
            if ctr.contains_key(&d) {
                if d != n || (d == n && ctr[&d] > 1) {
                    return n;
                }
            }
        }
    }
    return -1;
}

fn main() {
    let mut n_str = String::new();
    io::stdin().read_line(&mut n_str).unwrap();
    let n: i32 = n_str.trim().parse().unwrap();

    let mut nums: Vec<i32> = Vec::new();
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let nums_str: Vec<&str> = line.trim().split_whitespace().collect();
    for num_str in nums_str {
        nums.push(num_str.parse().unwrap());
    }

    let result = get_largest_outlier(&nums);
    println!("{}", result);
}