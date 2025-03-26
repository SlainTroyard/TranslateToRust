use std::cmp::{max, min};
use std::io::{self, Write};

// 返回两个数中的较小值
fn min(a: i32, b: i32) -> i32 {
    if a < b { a } else { b }
}

// 返回两个数中的较大值
fn max(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

// 表示区间的结构体 (相当于C++中的pair<int, int>)
#[derive(Debug, Clone, Copy)]
struct Interval {
    right: i32,  // 右端点 (用于排序)
    left: i32,   // 左端点
}

impl PartialEq for Interval {
    fn eq(&self, other: &Self) -> bool {
        self.right == other.right
    }
}

impl Eq for Interval {}

impl PartialOrd for Interval {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Interval {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.right.cmp(&other.right)
    }
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
    let n = s.len() as i32;
    
    // 为每个字符存储位置
    let mut pos: Vec<Vec<i32>> = vec![Vec::new(); 26];
    
    // 将每个字符的位置存储到对应数组
    for (i, c) in s.chars().enumerate() {
        let index = (c as u8 - b'a') as usize;
        pos[index].push(i as i32);
    }
    
    // 存储候选区间
    let mut intervals: Vec<Interval> = Vec::new();
    
    // 查找满足条件的区间
    for i in 0..26 {
        if !pos[i].is_empty() {
            let l = *pos[i].first().unwrap();
            let r = *pos[i].last().unwrap();
            let mut flag = true;
            
            while flag {
                flag = false;
                for j in 0..26 {
                    if !pos[j].is_empty() {
                        let low_idx = lower_bound(&pos[j], l);
                        let up_idx = upper_bound(&pos[j], r);
                        let cnt = up_idx - low_idx;
                        
                        if cnt > 0 && cnt < pos[j].len() {
                            l = min(l, *pos[j].first().unwrap());
                            r = max(r, *pos[j].last().unwrap());
                            flag = true;
                        }
                    }
                }
            }
            
            if l > 0 || r < n - 1 {
                intervals.push(Interval { right: r, left: l });
            }
        }
    }
    
    // 按照右端点排序区间
    intervals.sort_unstable();
    
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

fn main() {
    // 读取输入
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.len() != 2 {
        writeln!(io::stderr(), "Error reading input").unwrap();
        return;
    }
    let s = parts[0];
    let k: i32 = parts[1].parse().expect("Failed to parse k");
    
    // 调用函数计算结果
    let result = max_substring_length(s, k);
    
    // 输出结果
    println!("{}", if result { "true" } else { "false" });
}