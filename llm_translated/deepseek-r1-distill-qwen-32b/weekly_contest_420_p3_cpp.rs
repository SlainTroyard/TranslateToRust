// Problem: Weekly Contest 420 Problem 3
use std::io;

const MX: usize = 1_000_001;

fn compute_lpf(mx: usize) -> Vec<i32> {
    let mut lpf = vec![0; mx + 1];
    for i in 2..=mx {
        if lpf[i] == 0 {
            for j in (i..=mx).step_by(i) {
                if lpf[j] == 0 {
                    lpf[j] = i as i32;
                }
            }
        }
    }
    lpf
}

fn min_operations(nums: &mut Vec<i32>, lpf: &[i32]) -> i32 {
    let mut ans = 0;
    for i in (0..nums.len() - 1).rev() {
        if nums[i] > nums[i + 1] {
            nums[i] = lpf[nums[i] as usize];
            if nums[i] > nums[i + 1] {
                return -1;
            }
            ans += 1;
        }
    }
    ans
}

fn main() {
    let lpf = compute_lpf(MX);

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let tokens: Vec<&str> = input.split_whitespace().collect();

    let n = tokens[0].parse::<usize>().unwrap();
    let mut nums = tokens[1..n+1]
        .iter()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let ans = min_operations(&mut nums, &lpf);
    println!("{}", ans);
}