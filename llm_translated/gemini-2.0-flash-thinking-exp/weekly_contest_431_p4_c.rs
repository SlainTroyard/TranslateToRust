use std::cmp::Ordering;
use std::io;
use std::io::Write;

#[derive(Clone, Copy, Debug)]
struct DpNode {
    max: [i64; 4],
    idx: [u64; 4],
}

#[derive(Debug)]
struct PriorityQueue {
    arr: Vec<i64>,
}

impl PriorityQueue {
    fn new() -> Self {
        PriorityQueue { arr: Vec::new() }
    }

    fn push(&mut self, value: i64) {
        let mut son = self.arr.len();
        self.arr.push(value);
        while son > 0 {
            let father = (son - 1) / 2;
            if value.cmp(&self.arr[father]) == Ordering::Less {
                self.arr.swap(son, father);
                son = father;
            } else {
                break;
            }
        }
    }

    fn pop(&mut self) -> Option<i64> {
        if self.arr.is_empty() {
            return None;
        }
        if self.arr.len() == 1 {
            return self.arr.pop();
        }

        let last = self.arr.pop().unwrap();
        let mut father = 0;
        self.arr[0] = last;
        let mut left = 2 * father + 1;
        let mut right = 2 * father + 2;
        while left < self.arr.len() {
            let mut son = left;
            if right < self.arr.len() && self.arr[left] > self.arr[right] {
                son = right;
            }
            if self.arr[father] > self.arr[son] {
                self.arr.swap(father, son);
                father = son;
                left = 2 * father + 1;
                right = 2 * father + 2;
            } else {
                break;
            }
        }
        Some(self.arr[0])
    }

    fn is_empty(&self) -> bool {
        self.arr.is_empty()
    }

    fn len(&self) -> usize {
        self.arr.len()
    }
}

fn insert_index(idx: u64, i: i32, count: usize) -> u64 {
    let mut value = [(idx >> 48) as u16, ((idx >> 32) & 0xFFFF) as u16, ((idx >> 16) & 0xFFFF) as u16, (idx & 0xFFFF) as u16];
    value[count] = i as u16;

    for x in (0..count).rev() {
        if value[x] > i as u16 {
            value[x + 1] = value[x];
            value[x] = i as u16;
        }
    }

    ((value[0] as u64) << 48) | ((value[1] as u64) << 32) | ((value[2] as u64) << 16) | (value[3] as u64)
}

fn maximum_weight(intervals: &Vec<Vec<i32>>) -> Vec<i32> {
    let intervals_size = intervals.len();
    let mut dp: Vec<DpNode> = vec![
        DpNode {
            max: [0; 4],
            idx: [0; 4],
        };
        intervals_size
    ];
    let mut last = DpNode {
        max: [0; 4],
        idx: [0; 4],
    };
    let mut left_queue = PriorityQueue::new();
    let mut right_queue = PriorityQueue::new();

    for i in 0..intervals_size {
        left_queue.push(((intervals[i][0] as i64) << 32) | (i as i64));
        right_queue.push(((intervals[i][1] as i64) << 32) | (i as i64));
    }

    let mut max: i64 = 0;
    let mut sel: u64 = 0xFFFFFFFFFFFFFFFF;
    let mut return_size = 0;

    while !left_queue.is_empty() {
        let left_val = left_queue.pop().unwrap();
        let i = (left_val & 0xFFFFFFFF) as i32;
        let edge = (left_val >> 32) as i32;

        while !right_queue.is_empty() && (right_queue.arr[0] >> 32) as i32  < edge {
            let right_val = right_queue.pop().unwrap();
            let j = (right_val & 0xFFFFFFFF) as i32;

            for k in 0..3 {
                if last.max[k] < dp[j as usize].max[k] || (last.max[k] == dp[j as usize].max[k] && last.idx[k] > dp[j as usize].idx[k]) {
                    last.max[k] = dp[j as usize].max[k];
                    last.idx[k] = dp[j as usize].idx[k];
                }
            }
        }

        dp[i as usize] = last;

        for k in 0..4 {
            let prev = if k == 0 { 0 } else { last.max[k - 1] };
            if k == 0 || (k > 0 && prev > 0) {
                let idx = if k == 0 { (i as u64) << 48 } else { insert_index(last.idx[k - 1], i, k) };

                if dp[i as usize].max[k] < intervals[i as usize][2] as i64 + prev || (dp[i as usize].max[k] == intervals[i as usize][2] as i64 + prev && dp[i as usize].idx[k] > idx) {
                    dp[i as usize].max[k] = intervals[i as usize][2] as i64 + prev;
                    dp[i as usize].idx[k] = idx;
                }

                if max < dp[i as usize].max[k] || (max == dp[i as usize].max[k] && sel > dp[i as usize].idx[k]) {
                    return_size = k + 1;
                    max = dp[i as usize].max[k];
                    sel = dp[i as usize].idx[k];
                }
            }
        }
    }

    let mut result = Vec::with_capacity(return_size);
    for i in 0..return_size {
        result.push(((sel >> (3 - i << 4)) & 0xFFFF) as i32);
    }
    result
}

fn main() -> io::Result<()> {
    let mut n_str = String::new();
    io::stdin().read_line(&mut n_str).expect("Failed to read line");
    let n: i32 = n_str.trim().parse().expect("Invalid input");

    let mut vec: Vec<Vec<i32>> = Vec::new();
    for _ in 0..n {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).expect("Failed to read line");
        let values: Vec<i32> = input_line
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid input"))
            .collect();
        vec.push(values);
    }

    let result = maximum_weight(&vec);

    for &val in &result {
        print!("{} ", val);
        io::stdout().flush()?; // Ensure output is immediately printed
    }
    println!();

    Ok(())
}