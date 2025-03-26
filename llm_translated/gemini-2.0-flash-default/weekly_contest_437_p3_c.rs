use std::cmp::{max, min};
use std::io;
use std::str;

// 二分查找lower_bound (找到第一个大于等于val的元素位置)
fn lower_bound(arr: &[i32], val: i32) -> usize {
    let mut low = 0;
    let mut high = arr.len();
    while low < high {
        let mid = low + (high - low) / 2;
        if arr[mid] < val {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    low
}

// 二分查找upper_bound (找到第一个大于val的元素位置)
fn upper_bound(arr: &[i32], val: i32) -> usize {
    let mut low = 0;
    let mut high = arr.len();
    while low < high {
        let mid = low + (high - low) / 2;
        if arr[mid] <= val {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    low
}

#[derive(Debug, Clone, Copy)]
struct Interval {
    right: i32,
    left: i32,
}

fn max_substring_length(s: &str, k: i32) -> bool {
    let n = s.len();
    let s_bytes = s.as_bytes();

    // 为每个字符存储位置
    let mut pos: Vec<Vec<i32>> = vec![Vec::new(); 26];

    // 将每个字符的位置存储到对应数组
    for (i, &byte) in s_bytes.iter().enumerate() {
        let c = (byte - b'a') as usize;
        pos[c].push(i as i32);
    }

    // 存储候选区间
    let mut intervals: Vec<Interval> = Vec::new();

    // 查找满足条件的区间
    for i in 0..26 {
        if !pos[i].is_empty() {
            let mut l = pos[i][0];
            let mut r = pos[i][pos[i].len() - 1];
            let mut flag = true;

            while flag {
                flag = false;
                for j in 0..26 {
                    if !pos[j].is_empty() {
                        let low_idx = lower_bound(&pos[j], l);
                        let up_idx = upper_bound(&pos[j], r);
                        let cnt = up_idx - low_idx;

                        if cnt > 0 && cnt < pos[j].len() {
                            l = min(l, pos[j][0]);
                            r = max(r, pos[j][pos[j].len() - 1]);
                            flag = true;
                        }
                    }
                }
            }

            if l > 0 || r < (n as i32) - 1 {
                intervals.push(Interval { right: r, left: l });
            }
        }
    }

    // 按照右端点排序区间
    intervals.sort_by_key(|interval| interval.right);

    // 贪心选择区间
    let mut r = -1;
    let mut cnt = 0;
    for interval in intervals {
        if interval.left > r {
            r = interval.right;
            cnt += 1;
        }
    }

    cnt >= k
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    let s = parts[0];
    let k: i32 = parts[1].parse().unwrap();

    // 调用函数计算结果
    let result = max_substring_length(s, k);

    // 输出结果
    println!("{}", if result { "true" } else { "false" });

    Ok(())
}