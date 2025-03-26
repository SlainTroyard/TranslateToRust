// Problem: Weekly Contest 434 Problem 3
use std::io::{self, BufRead, Write};

// 获取两个数中的较大值
fn max(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

// 主函数实现
fn max_frequency(nums: &[i32], k: i32) -> i32 {
    let mut f0 = 0;               // 跟踪到目前为止有多少个等于k的元素
    let mut f1 = [0; 51];         // 保存每个数字的最大频率
    let mut max_f1 = 0;           // f1数组中的最大值
    let mut f2 = 0;               // 特定计算结果
    
    for &x in nums {
        // 更新f2 - 考虑当前元素
        f2 = max(f2, max_f1) + (if x == k { 1 } else { 0 });
        
        // 更新当前数字的频率
        f1[x as usize] = max(f1[x as usize], f0) + 1;
        
        // 如果当前元素等于k，更新f0
        f0 += if x == k { 1 } else { 0 };
        
        // 更新最大频率
        max_f1 = max(max_f1, f1[x as usize]);
    }
    
    // 返回最终结果
    max(max_f1, f2)
}

fn main() {
    // 读取输入
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut stdin_lock = stdin.lock();
    let mut input = String::new();

    // 读取 n 和 k
    if stdin_lock.read_line(&mut input).is_err() {
        writeln!(stderr(), "Error reading input for n and k").unwrap();
        return;
    }
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let k: i32 = iter.next().and_then(|s| s.parse().ok()).unwrap_or(0);

    // 分配内存并读取数组
    let mut nums = vec![0; n];
    for i in 0..n {
        input.clear();
        if stdin_lock.read_line(&mut input).is_err() {
            writeln!(stderr(), "Error reading input for nums[{}]", i).unwrap();
            return;
        }
        nums[i] = input.trim().parse().unwrap_or(0);
    }

    // 调用函数计算结果
    let result = max_frequency(&nums, k);

    // 输出结果
    writeln!(stdout, "{}", result).unwrap();
}