use std::cmp::max;
use std::io;
use std::io::Read;
use std::process;

// 返回数组中的最小值
fn min_element(arr: &[i32]) -> i32 {
    let mut min_val = arr[0];
    for &val in arr.iter().skip(1) {
        if val < min_val {
            min_val = val;
        }
    }
    min_val
}

// 检查函数 - 判断是否能够达到给定的分数low
fn check(points: &[i32], m: i32, low: i64) -> bool {
    let n = points.len();
    let mut rem = m;
    let mut pre = 0;
    for i in 0..n {
        let mut k = ((low - 1) / points[i] as i64 + 1 - pre as i64) as i32;
        if i == n - 1 && k <= 0 {
            break;
        }
        k = max(k, 1);
        rem -= k * 2 - 1;
        if rem < 0 {
            return false;
        }
        pre = k - 1;
    }
    true
}

// 主函数实现
fn max_score(points: &[i32], m: i32) -> i64 {
    let mut left = 0;
    // 计算二分查找的右边界
    let right = (m as i64 + 1) / 2 * min_element(points) as i64 + 1;

    // 二分查找最大可能分数
    let mut left_mut = left;
    let mut right_mut = right;

    while left_mut + 1 < right_mut {
        let mid = left_mut + (right_mut - left_mut) / 2;
        if check(points, m, mid) {
            left_mut = mid;
        } else {
            right_mut = mid;
        }
    }

    left_mut
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 读取输入
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let first_line = lines.next().ok_or("Missing input")?;
    let mut first_split = first_line.split_whitespace();

    let n: i32 = first_split
        .next()
        .ok_or("Missing n")?
        .parse()
        .map_err(|_| "Invalid input for n")?;
    let m: i32 = first_split
        .next()
        .ok_or("Missing m")?
        .parse()
        .map_err(|_| "Invalid input for m")?;

    let second_line = lines.next().ok_or("Missing input for points")?;
    let points: Vec<i32> = second_line
        .split_whitespace()
        .map(|s| s.parse().map_err(|_| "Invalid input for points"))
        .collect::<Result<Vec<i32>, _>>()?;

    if points.len() != n as usize {
        eprintln!("Error: Number of points does not match n");
        process::exit(1);
    }

    // 调用函数计算结果
    let result = max_score(&points, m);

    // 输出结果
    println!("{}", result);

    Ok(())
}