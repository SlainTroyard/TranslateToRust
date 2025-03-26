use std::io::{self, Write};

// 计算绝对值
fn abs_val(x: i32) -> i32 {
    if x < 0 { -x } else { x }
}

// 返回两个数中的较小值
fn min(a: i32, b: i32) -> i32 {
    if a < b { a } else { b }
}

// 返回两个数中的较大值
fn max(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

// 主函数实现
fn max_distance(s: &str, k: i32) -> i32 {
    let mut ans = 0;
    let mut x = 0;
    let mut y = 0;
    
    for (i, c) in s.chars().enumerate() {
        match c {
            'N' => y += 1,
            'S' => y -= 1,
            'E' => x += 1,
            'W' => x -= 1,
            _ => (),
        }
        
        // 根据当前位置、当前坐标和k值计算最大距离
        let current_max = min(abs_val(x) + abs_val(y) + k * 2, (i + 1) as i32);
        ans = max(ans, current_max);
    }
    
    ans
}

fn main() {
    // 读取输入
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.len() != 2 {
        eprintln!("Error reading input");
        return;
    }
    
    let s = parts[0];
    let k: i32 = parts[1].parse().expect("Failed to parse k");
    
    // 调用函数计算结果
    let result = max_distance(s, k);
    
    // 输出结果
    println!("{}", result);
}