use std::io::{self, Write};

// 返回两个数中的较大值
fn max(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

// 返回两个数中的较小值
fn min(a: i32, b: i32) -> i32 {
    if a < b { a } else { b }
}

// 主函数实现
fn max_difference(s: &str, k: usize) -> i32 {
    const INF: i32 = i32::MAX / 2;
    let mut ans = -INF;
    let len = s.len();
    
    for x in 0..5 {
        for y in 0..5 {
            if y == x {
                continue;
            }
            
            let mut cur_s = [0; 5]; // 当前窗口中各数字的出现次数
            let mut pre_s = [0; 5]; // 窗口左边部分的数字出现次数
            let mut min_s = [[INF, INF], [INF, INF]]; // 最小的差值
            let mut left = 0;
            
            for (i, c) in s.chars().enumerate() {
                cur_s[c as usize - '0' as usize] += 1; // 更新当前窗口中数字的出现次数
                let r = i + 1;
                
                // 当窗口大小 >= k 且满足其他条件时，尝试缩小窗口左边界
                while r - left >= k && cur_s[x] > pre_s[x] && cur_s[y] > pre_s[y] {
                    let p = &mut min_s[pre_s[x] as usize & 1][pre_s[y] as usize & 1];
                    *p = min(*p, pre_s[x] - pre_s[y]); // 更新最小差值
                    pre_s[s.chars().nth(left).unwrap() as usize - '0' as usize] += 1; // 更新窗口左边部分的数字出现次数
                    left += 1;
                }
                
                // 更新答案
                ans = max(ans, cur_s[x] - cur_s[y] - min_s[(cur_s[x] as usize & 1) ^ 1][cur_s[y] as usize & 1]);
            }
        }
    }
    
    ans
}

fn main() {
    // 读取输入
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input: Vec<&str> = input.trim().split_whitespace().collect();
    if input.len() != 2 {
        writeln!(io::stderr(), "Error reading input").unwrap();
        return;
    }
    
    let s = input[0];
    let k: usize = input[1].parse().expect("Failed to parse k");
    
    // 调用函数计算结果
    let result = max_difference(s, k);
    
    // 输出结果
    println!("{}", result);
}