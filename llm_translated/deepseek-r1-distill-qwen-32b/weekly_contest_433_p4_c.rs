use std::io;
use std::vec::Vec;

fn count(m: i32, k: i32) -> i64 {
    if m > k {
        ((2 * m - k + 1) * k) as i64 / 2
    } else {
        ((m + 1) * m) as i64 / 2
    }
}

fn sum_subarray_mins(nums: &[i32], k: i32) -> i64 {
    let mut res: i64 = 0;
    let mut stack: Vec<i32> = Vec::new();
    stack.push(-1); // Sentinel element

    for r in 0..nums.len() {
        let current = nums[r];
        while stack.len() > 1 && nums[stack[stack.len() - 1] as usize] >= current {
            let i = stack.pop().unwrap();
            let l = stack[stack.len() - 1];
            let m1 = r as i32 - l - 1;
            let m2 = i - l - 1;
            let m3 = r as i32 - i - 1;
            let cnt = count(m1, k) - count(m2, k) - count(m3, k);
            res += nums[i as usize] as i64 * cnt;
        }
        stack.push(r as i32);
    }

    res
}

fn min_max_subarray_sum(nums: &[i32], k: i32) -> i64 {
    let n = nums.len();
    let mut temp = nums.to_vec();
    temp.push(i32::MIN / 2);

    let sum_min = sum_subarray_mins(&temp, k);

    // Negate all elements except the last sentinel
    for i in 0..n {
        temp[i] = -temp[i];
    }
    temp[n] = i32::MIN / 2; // Restore sentinel

    let sum_neg_min = sum_subarray_mins(&temp, k);

    sum_min - sum_neg_min
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut parts = input.trim().split_whitespace();
    let n: usize = parts.next().unwrap().parse().expect("Invalid n");
    let k: i32 = parts.next().unwrap().parse().expect("Invalid k");

    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        let num: i32 = io::stdin()
            .read_line(&mut input)
            .expect("Failed to read number")
            .trim()
            .parse()
            .expect("Invalid number");
        nums.push(num);
    }

    let result = min_max_subarray_sum(&nums, k);
    println!("{}", result);
}