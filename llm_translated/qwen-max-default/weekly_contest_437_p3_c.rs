use std::cmp::{max, min};
use std::io::{self, Write};

// 表示区间的结构体 (相当于C++中的pair<int, int>)
#[derive(Debug, Clone, Copy)]
struct Interval {
    right: usize,  // 右端点 (用于排序)
    left: usize,   // 左端点
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
fn lower_bound(arr: &[usize], val: usize) -> usize {
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
fn upper_bound(arr: &[usize], val: usize) -> usize {
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
fn max_substring_length(s: &str, k: usize) -> bool {
    let n = s.len();
    
    // 为每个字符存储位置
    let mut pos: Vec<Vec<usize>> = vec![Vec::new(); 26];
    
    // 将每个字符的位置存储到对应数组
    for (i, c) in s.chars().enumerate() {
        let idx = (c as u8 - b'a') as usize;
        pos[idx].push(i);
    }
    
    // 存储候选区间
    let mut intervals: Vec<Interval> = Vec::new();
    
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
            
            if l > 0 || r < n - 1 {
                intervals.push(Interval { right: r, left: l });
            }
        }
    }
    
    // 按照右端点排序区间
    intervals.sort_by(|a, b| a.right.cmp(&b.right));
    
    // 贪心选择区间
    let mut r = usize::MAX;
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
    let mut s = String::new();
    let mut k = String::new();
    
    io::stdin()
        .read_line(&mut s)
        .expect("Failed to read line");
    io::stdin()
        .read_line(&mut k)
        .expect("Failed to read line");
    
    let s = s.trim();
    let k: usize = k.trim().parse().expect("Failed to parse k");
    
    // 调用函数计算结果
    let result = max_substring_length(s, k);
    
    // 输出结果
    println!("{}", if result { "true" } else { "false" });
}