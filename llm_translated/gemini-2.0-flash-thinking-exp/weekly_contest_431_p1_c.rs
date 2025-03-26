// Problem: Weekly Contest 431 Problem 1 in Rust

use std::io;
use std::str::FromStr;

// 计算两个数的最大公因数
fn gcd(a: i32, b: i32) -> i64 {
    let mut a_mut = a;
    let mut b_mut = b;
    while b_mut != 0 {
        let temp = b_mut;
        b_mut = a_mut % b_mut;
        a_mut = temp;
    }
    a_mut as i64
}

// 计算两个数的最小公倍数
fn lcm(a: i32, b: i32) -> i64 {
    (a as i64 / gcd(a, b)) * b as i64
}

// 返回最长乘积等价子数组的长度
fn max_length(nums: &[i32]) -> i32 {
    let mut max_length = 0;
    let nums_len = nums.len();
    for i in 0..nums_len {
        let mut prod: i64 = 1;
        let mut g = nums[i] as i64;
        let mut l = nums[i] as i64;
        for j in i..nums_len {
            if prod > i64::MAX / nums[j] as i64 {
                break; // 防止溢出, 模仿 C 代码的溢出检查方式
            }
            prod *= nums[j] as i64;
            g = gcd(g as i32, nums[j]);
            l = lcm(l as i32, nums[j]);
            if prod == l * g {
                let length = (j - i + 1) as i32;
                if length > max_length {
                    max_length = length;
                }
            }
        }
    }
    max_length
}

fn main() {
    // 读取数组大小
    let mut num_size_str = String::new();
    io::stdin().read_line(&mut num_size_str).expect("Failed to read line");
    let num_size: usize = num_size_str.trim().parse().expect("Invalid input for numSize");

    // 读取数组元素
    let mut nums_str = String::new();
    io::stdin().read_line(&mut nums_str).expect("Failed to read line");
    let nums: Vec<i32> = nums_str
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input for nums"))
        .collect();

    // 确保读取的元素数量与 numSize 一致
    if nums.len() != num_size {
        panic!("Number of input elements does not match numSize");
    }

    // 计算结果
    let result = max_length(&nums);
    println!("{}", result);
}