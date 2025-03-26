use std::io;
use std::io::Read;

fn minimum_sum_subarray(nums: &[i32], l: i32, r: i32) -> i32 {
    let mut min_sum: Option<i32> = None;

    for len in l..=r {
        let len = len as usize;
        let mut sum = 0;

        for i in 0..nums.len() {
            sum += nums[i];
            if i >= len {
                sum -= nums[i - len];
            }

            if sum > 0 && i >= len - 1 {
                if min_sum.is_none() || min_sum.unwrap() > sum {
                    min_sum = Some(sum);
                }
            }
        }
    }

    match min_sum {
        Some(val) => val,
        None => -1,
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.lines();

    let nums_size: usize = lines.next().unwrap().parse().unwrap();

    let nums: Vec<i32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let lr: Vec<i32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let l = lr[0];
    let r = lr[1];

    let result = minimum_sum_subarray(&nums, l, r);

    println!("{}", result);
}