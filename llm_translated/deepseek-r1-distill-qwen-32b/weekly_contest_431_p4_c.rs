use std::io;
use std::cmp::Ordering;

struct DpNode {
    max: [i64; 4],
    idx: [u64; 4],
}

struct PriorityQueue {
    arr: Vec<i64>,
    size: usize,
}

impl PriorityQueue {
    fn new() -> Self {
        Self {
            arr: Vec::new(),
            size: 0,
        }
    }

    fn push(&mut self, value: i64) {
        let mut son = self.size;
        self.size += 1;
        self.arr.resize(self.size, 0);
        while son > 0 {
            let father = (son - 1) >> 1;
            if self.arr[father] <= value {
                break;
            }
            self.arr[son] = self.arr[father];
            son = father;
        }
        self.arr[son] = value;
    }

    fn pop(&mut self) -> Option<i64> {
        if self.size == 0 {
            return None;
        }
        let value = self.arr[0];
        self.size -= 1;
        if self.size == 0 {
            self.arr.clear();
            return Some(value);
        }
        self.arr[0] = self.arr[self.size];
        let mut father = 0;
        while father < self.size {
            let left = (father << 1) + 1;
            let right = (father << 1) + 2;
            let mut son = father;
            if left < self.size && self.arr[left] < self.arr[son] {
                son = left;
            }
            if right < self.size && self.arr[right] < self.arr[son] {
                son = right;
            }
            if son == father {
                break;
            }
            self.arr.swap(father, son);
            father = son;
        }
        Some(value)
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.trim().split('\n').map(|s| s.trim());
    let n: usize = lines.next().unwrap().parse().unwrap();
    let mut intervals = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap();
        let mut parts = line.split_whitespace().map(|s| s.parse::<i64>().unwrap());
        let a = parts.next().unwrap();
        let b = parts.next().unwrap();
        let c = parts.next().unwrap();
        intervals.push((a, b, c));
    }

    let mut left_queue = PriorityQueue::new();
    let mut right_queue = PriorityQueue::new();
    for i in 0..n {
        let merged = ((intervals[i].0 as u64) << 32) | (i as u64);
        left_queue.push(merged as i64);
        let merged = ((intervals[i].1 as u64) << 32) | (i as u64);
        right_queue.push(merged as i64);
    }

    let mut dp = vec![DpNode {
        max: [0; 4],
        idx: [0; 4],
    }; n];
    let mut last = DpNode {
        max: [0; 4],
        idx: [0; 4],
    };
    let mut max_val = 0;
    let mut sel = 0;
    let mut return_size = 0;

    while left_queue.size > 0 {
        let val = left_queue.pop().unwrap();
        let edge = (val >> 32) as i64;
        let i = (val & 0xFFFFFFFF) as usize;

        while right_queue.size > 0 {
            let val_r = right_queue.arr[0];
            let end_r = (val_r >> 32) as i64;
            if end_r >= edge {
                break;
            }
            let j = (val_r & 0xFFFFFFFF) as usize;
            for k in 0..3 {
                if last.max[k] < dp[j].max[k] || (last.max[k] == dp[j].max[k] && last.idx[k] > dp[j].idx[k]) {
                    last.max[k] = dp[j].max[k];
                    last.idx[k] = dp[j].idx[k];
                }
            }
            right_queue.pop();
        }

        dp[i] = last;
        for k in 0..4 {
            let prev = if k == 0 { 0 } else { dp[i].max[k - 1] };
            if k == 0 || prev > 0 {
                let idx = if k == 0 {
                    (i as u64) << 48
                } else {
                    insert_index(dp[i].idx[k - 1], i, k)
                };
                let new_val = intervals[i].2 as i64 + prev;
                if dp[i].max[k] < new_val || (dp[i].max[k] == new_val && dp[i].idx[k] > idx) {
                    dp[i].max[k] = new_val;
                    dp[i].idx[k] = idx;
                }
                if max_val < dp[i].max[k] || (max_val == dp[i].max[k] && sel > dp[i].idx[k]) {
                    return_size = k + 1;
                    max_val = dp[i].max[k];
                    sel = dp[i].idx[k];
                }
            }
        }
    }

    let mut result = Vec::with_capacity(return_size);
    for i in 0..return_size {
        let shift = (3 - i) << 4;
        let val = (sel >> shift) & 0xFFFF;
        result.push(val as i32);
    }

    println!("{}", result.join(" "));
}

fn insert_index(idx: u64, i: usize, count: usize) -> u64 {
    let mut values = [0u16; 4];
    values[0] = (idx >> 48) as u16;
    values[1] = (idx >> 32 & 0xFFFF) as u16;
    values[2] = (idx >> 16 & 0xFFFF) as u16;
    values[count] = i as u16;
    let mut x = count;
    while x > 0 && values[x - 1] > values[x] {
        values.swap(x, x - 1);
        x -= 1;
    }
    ((values[0] as u64) << 48) | ((values[1] as u64) << 32) | ((values[2] as u64) << 16) | (values[3] as u64)
}