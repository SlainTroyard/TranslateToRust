use std::io;
use std::io::Read;
use std::cmp::min;

fn minimum_sum_subarray(nums: &Vec<i32>, l: i32, r: i32) -> i32 {
    let n = nums.len();
    let mut mini = i32::MAX;
    for i in 0..n {
        let mut currsum = 0;
        for j in i..n {
            currsum += nums[j];
            let length = (j - i + 1) as i32;
            if length >= l && length <= r && currsum > 0 {
                mini = min(mini, currsum);
            }
        }
    }
    if mini == i32::MAX {
        -1
    } else {
        mini
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.lines();

    let n: usize = lines.next().unwrap().parse().unwrap();

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