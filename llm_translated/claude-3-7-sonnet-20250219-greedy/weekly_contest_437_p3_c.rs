use std::cmp::{min, max};
use std::io::{self, BufRead};

// 表示区间的结构体
#[derive(Clone, Copy)]
struct Interval {
    right: usize,  // 右端点 (用于排序)
    left: usize,   // 左端点
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
    let mut pos: Vec<Vec<usize>> = vec![Vec::with_capacity(n); 26];
    
    // 将每个字符的位置存储到对应数组
    for (i, c) in s.bytes().enumerate() {
        let idx = (c - b'a') as usize;
        pos[idx].push(i);
    }
    
    // 存储候选区间
    let mut intervals = Vec::with_capacity(26);
    
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
            
            if l > 0 || r < n - 1 {
                intervals.push(Interval { right: r, left: l });
            }
        }
    }
    
    // 按照右端点排序区间
    intervals.sort_by_key(|interval| interval.right);
    
    // 贪心选择区间
    let mut r = 0;
    let mut cnt = 0;
    
    for interval in intervals {
        if interval.left >= r {
            r = interval.right + 1;
            cnt += 1;
        }
    }
    
    cnt >= k
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // 读取输入
    let input = lines.next().unwrap()?;
    let parts: Vec<&str> = input.split_whitespace().collect();
    
    if parts.len() != 2 {
        eprintln!("Error reading input");
        return Ok(());
    }
    
    let s = parts[0];
    let k = parts[1].parse::<usize>().unwrap();
    
    // 调用函数计算结果
    let result = max_substring_length(s, k);
    
    // 输出结果
    println!("{}", if result { "true" } else { "false" });
    
    Ok(())
}