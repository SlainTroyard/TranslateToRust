use std::io::{self, BufRead};

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

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // 读取n和k
    let first_line = lines.next().ok_or(io::Error::new(io::ErrorKind::Other, "Missing input for n and k"))??;
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().ok_or(io::Error::new(io::ErrorKind::Other, "Missing n"))?.parse().map_err(|_| io::Error::new(io::ErrorKind::Other, "Invalid n"))?;
    let k: i32 = parts.next().ok_or(io::Error::new(io::ErrorKind::Other, "Missing k"))?.parse().map_err(|_| io::Error::new(io::ErrorKind::Other, "Invalid k"))?;
    
    // 读取数组
    let second_line = lines.next().ok_or(io::Error::new(io::ErrorKind::Other, "Missing input for nums array"))??;
    let nums: Vec<i32> = second_line.split_whitespace()
        .map(|s| s.parse().map_err(|_| io::Error::new(io::ErrorKind::Other, "Invalid number in nums array")))
        .collect::<Result<Vec<_>, _>>()?;
    
    // 检查数组长度是否匹配n
    if nums.len() != n {
        return Err(io::Error::new(io::ErrorKind::Other, "Array length does not match n"));
    }
    
    // 调用函数计算结果
    let result = max_frequency(&nums, k);
    
    // 输出结果
    println!("{}", result);
    
    Ok(())
}