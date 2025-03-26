use std::io::{self, Write};

/// Counts the number of substrings that are divisible by their length.
fn count_substrings(s: &str) -> u64 {
    let mut ans = 0;
    let mut f = [[0; 9]; 10]; // 初始化计数数组为0

    for (i, c) in s.chars().enumerate() {
        let d = c.to_digit(10).unwrap() as usize; // 当前数字

        for m in 1..10 {
            let mut nf = [0; 9]; // 临时数组，用于保存新的计数结果
            nf[d % m] = 1;       // 单个数字d模m的余数

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
        ans += f[d][0] as u64;
    }

    ans
}

fn main() {
    // 读取输入
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        writeln!(io::stderr(), "Error reading input").unwrap();
        return;
    }
    input = input.trim().to_string(); // 去除换行符和空白字符

    // 调用函数计算结果
    let result = count_substrings(&input);

    // 输出结果
    println!("{}", result);
}