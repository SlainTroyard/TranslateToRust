use std::io::{self, BufRead};
use std::cmp::{min, max};

#[derive(Debug, Copy, Clone)]
struct Interval {
    right: i32,
    left: i32,
}

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

// 主函数实现
fn max_substring_length(s: &str, k: i32) -> bool {
    let n = s.len();
    let s_bytes = s.as_bytes();

    // 为每个字符存储位置
    let mut pos: Vec<Vec<i32>> = vec![Vec::new(); 26];

    // 将每个字符的位置存储到对应数组
    for i in 0..n {
        let c = (s_bytes[i] - b'a') as usize;
        pos[c].push(i as i32);
    }

    // 存储候选区间
    let mut intervals: Vec<Interval> = Vec::with_capacity(26);
    let mut interval_count = 0;

    // 查找满足条件的区间
    for i in 0..26 {
        if !pos[i].is_empty() {
            let mut l = pos[i][0];
            let mut r = *pos[i].last().unwrap();
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
                            r = max(r, *pos[j].last().unwrap());
                            flag = true;
                        }
                    }
                }
            }

            if l > 0 || r < (n as i32) - 1 {
                intervals.push(Interval { right: r, left: l });
                interval_count += 1;
            }
        }
    }

    // 按照右端点排序区间
    intervals.sort_by_key(|interval| interval.right);

    // 贪心选择区间
    let mut R = -1;
    let mut cnt = 0;
    for i in 0..interval_count {
        if intervals[i].left > R {
            R = intervals[i].right;
            cnt += 1;
        }
    }

    cnt >= k
}

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();

    let line = iterator.next().unwrap().expect("Failed to read line");
    let mut parts = line.split_whitespace();

    let s = parts.next().unwrap();
    let k: i32 = parts.next().unwrap().parse().expect("Invalid integer");

    let result = max_substring_length(s, k);

    println!("{}", if result { "true" } else { "false" });
}