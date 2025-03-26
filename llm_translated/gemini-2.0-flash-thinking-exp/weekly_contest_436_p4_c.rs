use std::io;
use std::io::BufRead;

// 返回两个数中的较大值
fn max(a: i32, b: i32) -> i32 {
    a.max(b)
}

// 返回数组中的最小值
fn min_element(arr: &[i32]) -> i32 {
    *arr.iter().min().unwrap()
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
    let right = ((m + 1) / 2) as i64 * min_element(points) as i64 + 1;

    let mut right_val = right; // Make right mutable
    while left + 1 < right_val {
        let mid = left + (right_val - left) / 2;
        if check(points, m, mid) {
            left = mid;
        } else {
            right_val = mid;
        }
    }
    left
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();

    let first_line = iterator.next().unwrap()?;
    let mut nm_iter = first_line.split_whitespace();
    let n: usize = nm_iter.next().unwrap().parse().unwrap();
    let m: i32 = nm_iter.next().unwrap().parse().unwrap();

    let second_line = iterator.next().unwrap()?;
    let points: Vec<i32> = second_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // 调用函数计算结果
    let result = max_score(&points, m);

    // 输出结果
    println!("{}", result);

    Ok(())
}