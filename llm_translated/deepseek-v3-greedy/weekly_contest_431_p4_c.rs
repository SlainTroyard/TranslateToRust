use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io::{self, BufRead};

#[derive(Clone, Copy)]
struct DpNode {
    max: [i64; 4],
    idx: [u64; 4],
}

impl Default for DpNode {
    fn default() -> Self {
        DpNode {
            max: [0; 4],
            idx: [0; 4],
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct QueueItem {
    value: i64,
    index: usize,
}

impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value).reverse()
    }
}

impl PartialOrd for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn father_node(i: usize) -> usize {
    if i == 0 { usize::MAX } else { (i - 1) >> 1 }
}

fn left_node(i: usize) -> usize {
    (i << 1) + 1
}

fn right_node(i: usize) -> usize {
    (i << 1) + 2
}

fn higher_int(i: i64) -> i32 {
    (i >> 32) as i32
}

fn lower_int(i: i64) -> usize {
    (i & 0xFFFFFFFF) as usize
}

fn merge_long(i: i32, j: usize) -> i64 {
    (i as i64) << 32 | j as i64
}

fn insert_index(idx: u64, i: usize, count: usize) -> u64 {
    let mut value = [
        (idx >> 48) as usize,
        (idx >> 32 & 0xFFFF) as usize,
        (idx >> 16 & 0xFFFF) as usize,
        i,
    ];
    let mut x = count - 1;
    while x != usize::MAX && value[x] > i {
        value[x + 1] = value[x];
        value[x] = i;
        x -= 1;
    }
    (value[0] as u64) << 48 | (value[1] as u64) << 32 | (value[2] as u64) << 16 | value[3] as u64
}

fn maximum_weight(intervals: &Vec<Vec<i32>>, return_size: &mut usize) -> Vec<usize> {
    let n = intervals.len();
    let mut dp = vec![DpNode::default(); n];
    let mut last = DpNode::default();
    let mut left_queue = BinaryHeap::new();
    let mut right_queue = BinaryHeap::new();
    let mut max = 0;
    let mut sel = u64::MAX;

    for i in 0..n {
        left_queue.push(QueueItem {
            value: merge_long(intervals[i][0], i),
            index: i,
        });
        right_queue.push(QueueItem {
            value: merge_long(intervals[i][1], i),
            index: i,
        });
    }

    while let Some(item) = left_queue.pop() {
        let i = lower_int(item.value);
        let edge = higher_int(item.value);

        while let Some(right_item) = right_queue.peek() {
            if edge > higher_int(right_item.value) {
                let j = lower_int(right_item.value);
                for k in 0..3 {
                    if last.max[k] < dp[j].max[k] || (last.max[k] == dp[j].max[k] && last.idx[k] > dp[j].idx[k]) {
                        last.max[k] = dp[j].max[k];
                        last.idx[k] = dp[j].idx[k];
                    }
                }
                right_queue.pop();
            } else {
                break;
            }
        }

        dp[i] = last;
        for k in 0..4 {
            let prev = if k == 0 { 0 } else { last.max[k - 1] };
            if k == 0 || (k > 0 && prev > 0) {
                let idx = if k == 0 { (i as u64) << 48 } else { insert_index(last.idx[k - 1], i, k) };
                if dp[i].max[k] < intervals[i][2] as i64 + prev || (dp[i].max[k] == intervals[i][2] as i64 + prev && dp[i].idx[k] > idx) {
                    dp[i].max[k] = intervals[i][2] as i64 + prev;
                    dp[i].idx[k] = idx;
                }
                if max < dp[i].max[k] || (max == dp[i].max[k] && sel > dp[i].idx[k]) {
                    *return_size = k + 1;
                    max = dp[i].max[k];
                    sel = dp[i].idx[k];
                }
            }
        }
    }

    let mut result = vec![0; *return_size];
    for i in 0..*return_size {
        result[i] = (sel >> (3 - i << 4) & 0xFFFF) as usize;
    }
    result
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut intervals = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        let start: i32 = iter.next().unwrap().parse().unwrap();
        let end: i32 = iter.next().unwrap().parse().unwrap();
        let weight: i32 = iter.next().unwrap().parse().unwrap();
        intervals.push(vec![start, end, weight]);
    }

    let mut return_size = 0;
    let result = maximum_weight(&intervals, &mut return_size);
    for i in 0..return_size {
        print!("{} ", result[i]);
    }
    println!();
}