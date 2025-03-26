use std::io::{self, Write};

// 返回两个数中的较大值
fn max(a: i64, b: i64) -> i64 {
    if a > b { a } else { b }
}

// 返回数组中的最小值
fn min_element(arr: &[i64]) -> i64 {
    *arr.iter().min().unwrap_or(&0)
}

// 检查函数 - 判断是否能够达到给定的分数low
fn check(points: &[i64], m: i64, low: i64) -> bool {
    let n = points.len();
    let mut rem = m;
    let mut pre = 0;

    for i in 0..n {
        let k = ((low - 1) / points[i] + 1 - pre) as i64;
        if i == n - 1 && k <= 0 {
            break;
        }
        let k = max(k, 1);
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
    let mut right = ((m + 1) / 2) * min_element(points) + 1;

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
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().expect("Invalid input for n");
    let m: i64 = iter.next().unwrap().parse().expect("Invalid input for m");

    // 分配内存
    let mut points = Vec::with_capacity(n);

    // 读取points数组
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    for num in input.split_whitespace() {
        points.push(num.parse().expect("Invalid input for points"));
    }

    // 调用函数计算结果
    let result = max_score(&points, m);

    // 输出结果
    println!("{}", result);
}