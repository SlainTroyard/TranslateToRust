// Problem: Weekly Contest 435 Problem 4 in Rust

use std::io;
use std::cmp::{max, min};

// 主函数实现
fn max_difference(s: &str, k: i32) -> i32 {
    let inf = i32::MAX / 2;
    let mut ans = -inf;
    let len = s.len();

    for x in 0..5 {
        for y in 0..5 {
            if y == x {
                continue;
            }

            let mut cur_s = [0; 5]; // 当前窗口中各数字的出现次数
            let mut pre_s = [0; 5]; // 窗口左边部分的数字出现次数
            let mut min_s = [[inf; 2]; 2]; // 最小的差值
            let mut left = 0;

            for (i, char_digit) in s.chars().enumerate() {
                let digit = char_digit.to_digit(10).unwrap() as usize;
                cur_s[digit] += 1; // 更新当前窗口中数字的出现次数
                let r = i + 1;

                // 当窗口大小 >= k 且满足其他条件时，尝试缩小窗口左边界
                while r - left >= k as usize && cur_s[x] > pre_s[x] && cur_s[y] > pre_s[y] {
                    let p = &mut min_s[pre_s[x] % 2][pre_s[y] % 2];
                    *p = min(*p, pre_s[x] - pre_s[y]); // 更新最小差值
                    let left_digit = s.chars().nth(left).unwrap().to_digit(10).unwrap() as usize;
                    pre_s[left_digit] += 1; // 更新窗口左边部分的数字出现次数
                    left += 1;
                }

                // 更新答案
                ans = max(ans, cur_s[x] - cur_s[y] - min_s[(cur_s[x] % 2) ^ 1][cur_s[y] % 2]);
            }
        }
    }

    ans
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("Failed to read line");
    let parts: Vec<&str> = input_line.trim().split_whitespace().collect();

    if parts.len() != 2 {
        eprintln!("Error: Invalid input format. Expected string and integer.");
        return;
    }

    let s = parts[0];
    let k = parts[1].parse::<i32>().expect("Invalid integer input for k");

    // 调用函数计算结果
    let result = max_difference(s, k);

    // 输出结果
    println!("{}", result);
}