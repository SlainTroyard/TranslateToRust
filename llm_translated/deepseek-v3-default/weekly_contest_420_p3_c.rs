use std::io::{self, BufRead};
use std::cmp;
use std::f64::consts::E;

fn min_operations(nums: &mut Vec<i32>) -> i32 {
    let mut res = 0;
    let mut max_factor;
    let mut j;

    if nums.len() == 1 {
        return res;
    }

    for i in (0..nums.len() - 1).rev() {
        if nums[i] > nums[i + 1] {
            let mut max_val = i32::MIN;
            let mut count = 1;
            let original = nums[i];
            loop {
                max_val = i32::MIN;
                max_factor = (nums[i] as f64).sqrt() as i32 + 1;
                for j in 2..=max_factor {
                    if nums[i] % j == 0 {
                        max_val = cmp::max(max_val, j);
                        if nums[i] % (nums[i] / j) == 0 {
                            max_val = cmp::max(max_val, nums[i] / j);
                        }
                    }
                }
                if max_val == i32::MIN || (count *= max_val) == original {
                    return -1;
                } else {
                    nums[i] /= max_val;
                    if nums[i] <= nums[i + 1] {
                        res += 1;
                        break;
                    }
                }
            }
        }
    }

    res
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let nums_size = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
    let mut nums = Vec::with_capacity(nums_size);
    for _ in 0..nums_size {
        let num = lines.next().unwrap().unwrap().parse::<i32>().unwrap();
        nums.push(num);
    }
    println!("{}", min_operations(&mut nums));
}