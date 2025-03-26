use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

fn find_x_sum(nums: &Vec<i32>, k: i32, x: i32) -> Vec<i32> {
    let mut mp: HashMap<i32, i32> = HashMap::new();
    let mut res: Vec<i32> = Vec::new();
    let mut l = 0;
    for r in 0..nums.len() {
        let count = mp.entry(nums[r]).or_insert(0);
        *count += 1;

        if r as i32 - l as i32 + 1 == k {
            let mut vec: Vec<(&i32, &i32)> = mp.iter().collect(); // num - cnt

            vec.sort_by(|a, b| {
                if a.1 == b.1 {
                    b.0.cmp(a.0) // 出现次数相同，num大的放前边
                } else {
                    b.1.cmp(a.1) // 出现次数不同，cnt大的在前边
                }
            });

            let mut sum = 0;
            for i in 0..x.min(vec.len() as i32) {
                sum += vec[i as usize].0 * vec[i as usize].1;
            }
            res.push(sum);

            if let Some(count) = mp.get_mut(&nums[l]) {
                *count -= 1;
                if *count == 0 {
                    mp.remove(&nums[l]);
                }
            }

            l += 1;
        }
    }
    res
}

fn main() {
    let mut k_x_str = String::new();
    io::stdin().read_line(&mut k_x_str).expect("Failed to read line");
    let k_x: Vec<i32> = k_x_str
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let k = k_x[0];
    let x = k_x[1];

    let mut nums_size_str = String::new();
    io::stdin().read_line(&mut nums_size_str).expect("Failed to read line");
    let nums_size: usize = nums_size_str.trim().parse().unwrap();

    let mut nums_str = String::new();
    io::stdin().read_line(&mut nums_str).expect("Failed to read line");
    let nums: Vec<i32> = nums_str
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let res = find_x_sum(&nums, k, x);
    for i in 0..res.len() {
        print!("{} ", res[i]);
    }
    println!();
}