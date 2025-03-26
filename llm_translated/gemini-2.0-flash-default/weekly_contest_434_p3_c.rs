use std::cmp::max;
use std::io;
use std::io::Read;

// 主函数实现
fn max_frequency(nums: &[i32], k: i32) -> i32 {
    let mut f0 = 0; // 跟踪到目前为止有多少个等于k的元素
    let mut f1 = [0; 51]; // 保存每个数字的最大频率
    let mut max_f1 = 0; // f1数组中的最大值
    let mut f2 = 0; // 特定计算结果

    for &x in nums {
        // 更新f2 - 考虑当前元素
        f2 = max(f2, max_f1) + if x == k { 1 } else { 0 };

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 读取输入
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut lines = input.lines();

    let first_line = lines.next().ok_or("No input lines")?;
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().ok_or("Missing n")?.parse()?;
    let k: i32 = parts.next().ok_or("Missing k")?.parse()?;

    let second_line = lines.next().ok_or("Missing second line")?;
    let nums: Vec<i32> = second_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // 验证数组长度
    if nums.len() != n {
        return Err("Array length does not match n".into());
    }

    // 调用函数计算结果
    let result = max_frequency(&nums, k);

    // 输出结果
    println!("{}", result);

    Ok(())
}