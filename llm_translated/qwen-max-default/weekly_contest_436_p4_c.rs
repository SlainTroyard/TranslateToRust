use std::io::{self, BufRead, Write};

// 返回两个数中的较大值
fn max(a: i64, b: i64) -> i64 {
    if a > b { a } else { b }
}

// 返回数组中的最小值
fn min_element(arr: &[i64]) -> i64 {
    *arr.iter().min().unwrap()
}

// 检查函数 - 判断是否能够达到给定的分数low
fn check(points: &[i64], m: i64, low: i64) -> bool {
    let n = points.len() as i64;
    let mut rem = m;
    let mut pre = 0;

    for (i, &point) in points.iter().enumerate() {
        let k = ((low - 1) / point + 1 - pre).max(1);
        if i == n as usize - 1 && k <= 0 {
            break;
        }
        rem -= k * 2 - 1;
        if rem < 0 {
            return false;
        }
        pre = k - 1;
    }
    true
}

// 主函数实现
fn max_score(points: &[i64], m: i64) -> i64 {
    let mut left = 0;
    // 计算二分查找的右边界
    let right = (m + 1) / 2 * min_element(points) + 1;

    // 二分查找最大可能分数
    while left + 1 < right {
        let mid = left + (right - left) / 2;
        if check(points, m, mid) {
            left = mid;
        } else {
            right = mid;
        }
    }

    left
}

fn main() {
    // 读取输入
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut stdout = io::stdout();
    let mut stdout_lock = stdout.lock();

    let mut input = String::new();
    if stdin_lock.read_line(&mut input).is_err() {
        writeln!(stderr(), "Error reading input for n and m").unwrap();
        return;
    }
    let mut iter = input.split_whitespace();
    let n: i64 = iter.next().and_then(|s| s.parse().ok()).unwrap_or_else(|| {
        writeln!(stderr(), "Error reading input for n and m").unwrap();
        std::process::exit(1);
    });
    let m: i64 = iter.next().and_then(|s| s.parse().ok()).unwrap_or_else(|| {
        writeln!(stderr(), "Error reading input for n and m").unwrap();
        std::process::exit(1);
    });

    // 分配内存
    let mut points: Vec<i64> = Vec::with_capacity(n as usize);

    for _ in 0..n {
        input.clear();
        if stdin_lock.read_line(&mut input).is_err() {
            writeln!(stderr(), "Error reading input for points").unwrap();
            return;
        }
        let point: i64 = input.trim().parse().unwrap_or_else(|_| {
            writeln!(stderr(), "Error reading input for points").unwrap();
            std::process::exit(1);
        });
        points.push(point);
    }

    // 调用函数计算结果
    let result = max_score(&points, m);

    // 输出结果
    writeln!(stdout_lock, "{}", result).unwrap();
}