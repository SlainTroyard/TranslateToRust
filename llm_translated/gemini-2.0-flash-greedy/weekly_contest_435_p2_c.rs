use std::cmp::{max, min};
use std::io;
use std::io::Read;

// 计算绝对值
fn abs_val(x: i32) -> i32 {
    if x < 0 {
        -x
    } else {
        x
    }
}

// 主函数实现
fn max_distance(s: &str, k: i32) -> i32 {
    let mut ans = 0;
    let mut x = 0;
    let mut y = 0;
    let length = s.len();

    for (i, c) in s.chars().enumerate() {
        match c {
            'N' => y += 1,
            'S' => y -= 1,
            'E' => x += 1,
            'W' => x -= 1,
            _ => {} // Handle other characters if needed
        }

        // 根据当前位置、当前坐标和k值计算最大距离
        let current_max = min(abs_val(x) + abs_val(y) + k * 2, (i + 1) as i32);
        ans = max(ans, current_max);
    }

    ans
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut iter = input.split_whitespace();
    let s = iter.next().ok_or("Missing string")?;
    let k: i32 = iter.next().ok_or("Missing integer")?.parse()?;

    // 调用函数计算结果
    let result = max_distance(s, k);

    // 输出结果
    println!("{}", result);

    Ok(())
}