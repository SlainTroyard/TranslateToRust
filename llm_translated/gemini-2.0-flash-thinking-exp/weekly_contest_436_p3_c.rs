// Problem: Weekly Contest 436 Problem 3

use std::io;
use std::str::Chars;

fn count_substrings(s: &str) -> i64 {
    let mut ans: i64 = 0;
    let mut f = [[0i32; 9]; 10]; // 初始化计数数组为0

    for char_digit in s.chars() {
        let d = char_digit.to_digit(10).unwrap() as usize; // 当前数字

        for m in 1..10 {
            let mut nf = [0i32; 9]; // 临时数组，用于保存新的计数结果
            nf[d % m] = 1;   // 单个数字d模m的余数

            for rem in 0..m {
                // 更新计数：将当前数字d添加到已有余数rem后面形成的新余数
                nf[(rem * 10 + d) % m] += f[m][rem];
            }

            // 更新f数组
            for rem in 0..m {
                f[m][rem] = nf[rem];
            }
        }

        // 当前数字被自身整除，增加结果计数
        ans += f[d][0] as i64;
    }

    ans
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 读取输入
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    let s = s.trim(); // Remove trailing newline

    // 调用函数计算结果
    let result = count_substrings(s);

    // 输出结果
    println!("{}", result);

    Ok(())
}