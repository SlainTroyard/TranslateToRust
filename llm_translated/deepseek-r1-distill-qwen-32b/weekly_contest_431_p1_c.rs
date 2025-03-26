// Problem: Weekly Contest 431 Problem 1
use std::io;

// 计算两个数的最大公因数
fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

// 计算两个数的最小公倍数
fn lcm(a: i64, b: i64) -> i64 {
    (a / gcd(a, b)) * b
}

// 返回最长乘积等价子数组的长度
fn max_length(nums: &[i32]) -> usize {
    let mut max_length = 0;
    for i in 0..nums.len() {
        let mut prod = 1;
        let mut g = nums[i] as i64;
        let mut l = nums[i] as i64;
        for j in i..nums.len() {
            let num = nums[j] as i64;
            // 防止溢出
            if prod > i64::MAX / num {
                break;
            }
            prod *= num;
            g = gcd(g, num);
            l = lcm(l, num);
            if prod == l * g {
                let current_length = j - i + 1;
                if current_length > max_length {
                    max_length = current_length;
                }
            }
        }
    }
    max_length
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut tokens = input.split_whitespace();
    let num_size: usize = tokens.next().expect("No input").parse().expect("Invalid number");
    let nums: Vec<i32> = tokens.take(num_size).map(|s| s.parse().expect("Invalid number")).collect();

    let result = max_length(&nums);
    println!("{}", result);
}