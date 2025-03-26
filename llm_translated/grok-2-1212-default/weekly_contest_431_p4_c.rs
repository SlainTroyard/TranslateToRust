use std::io::{self, BufRead};

// Constants for tree operations
const fn father_node(i: usize) -> isize {
    if i == 0 { -1 } else { ((i - 1) >> 1) as isize }
}
const fn left_node(i: usize) -> usize {
    (i << 1) + 1
}
const fn right_node(i: usize) -> usize {
    (i << 1) + 2
}
const fn higher_int(i: i64) -> i64 {
    i >> 32
}
const fn lower_int(i: i64) -> i64 {
    i & 0xFFFFFFFF
}
const fn merge_long(i: i64, j: i64) -> i64 {
    (i << 32) | j
}

// DpNode structure
#[derive(Clone, Copy)]
struct DpNode {
    max: [i64; 4],
    idx: [u64; 4],
}

// PriorityQueue structure
struct PriorityQueue {
    arr: Vec<i64>,
}

impl PriorityQueue {
    fn new() -> Self {
        PriorityQueue { arr: Vec::new() }
    }

    fn push(&mut self, value: i64) {
        let mut son = self.arr.len();
        let mut father = father_node(son) as usize;
        self.arr.push(value);
        while father != usize::MAX && value < self.arr[father] {
            self.arr[son] = self.arr[father];
            son = father;
            father = father_node(son) as usize;
        }
        self.arr[son] = value;
    }

    fn pop(&mut self) {
        let mut father = 0;
        let mut left = left_node(father);
        let mut right = right_node(father);
        let mut son = 0;
        self.arr.swap_remove(0);
        while (left < self.arr.len() && self.arr[self.arr.len() - 1] > self.arr[left])
            || (right < self.arr.len() && self.arr[self.arr.len() - 1] > self.arr[right])
        {
            son = if right < self.arr.len() && self.arr[left] > self.arr[right] {
                right
            } else {
                left
            };
            self.arr[father] = self.arr[son];
            father = son;
            left = left_node(father);
            right = right_node(father);
        }
        if !self.arr.is_empty() {
            self.arr[father] = self.arr[self.arr.len() - 1];
        }
    }

    fn is_empty(&self) -> bool {
        self.arr.is_empty()
    }
}

// Helper function to insert index
fn insert_index(idx: u64, i: i32, count: usize) -> u64 {
    let mut value = [0u16; 4];
    value[0] = (idx >> 48) as u16;
    value[1] = ((idx >> 32) & 0xFFFF) as u16;
    value[2] = ((idx >> 16) & 0xFFFF) as u16;
    value[count] = i as u16;
    for x in (0..count).rev() {
        if value[x] > i as u16 {
            value[x + 1] = value[x];
            value[x] = i as u16;
        } else {
            break;
        }
    }
    (value[0] as u64) << 48 | (value[1] as u64) << 32 | (value[2] as u64) << 16 | (value[3] as u64)
}

// Main function to calculate maximum weight
fn maximum_weight(intervals: &Vec<Vec<i32>>, return_size: &mut usize) -> Vec<i32> {
    let mut left_queue = PriorityQueue::new();
    let mut right_queue = PriorityQueue::new();
    let mut dp = vec![DpNode {
        max: [0; 4],
        idx: [0; 4],
    }; intervals.len()];
    let mut last = DpNode {
        max: [0; 4],
        idx: [0; 4],
    };
    let mut max = 0;
    let mut sel = u64::MAX;

    for i in 0..intervals.len() {
        left_queue.push(merge_long(intervals[i][0] as i64, i as i64));
        right_queue.push(merge_long(intervals[i][1] as i64, i as i64));
    }

    while !left_queue.is_empty() {
        let i = lower_int(left_queue.arr[0]) as usize;
        let edge = higher_int(left_queue.arr[0]);

        while !right_queue.is_empty() && edge > higher_int(right_queue.arr[0]) {
            let j = lower_int(right_queue.arr[0]) as usize;
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
            let prev = if k == 0 { 0 } else { last.max[k - 1] };
            if k == 0 || (k > 0 && prev > 0) {
                let idx = if k == 0 {
                    (i as u64) << 48
                } else {
                    insert_index(last.idx[k - 1], i as i32, k)
                };
                if dp[i].max[k] < intervals[i][2] as i64 + prev
                    || (dp[i].max[k] == intervals[i][2] as i64 + prev && dp[i].idx[k] > idx)
                {
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
        left_queue.pop();
    }

    (0..*return_size)
        .map(|i| ((sel >> ((3 - i) << 4)) & 0xFFFF) as i32)
        .collect()
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of intervals
    let n: usize = lines.next().unwrap()?.parse().unwrap();

    // Read intervals
    let mut intervals = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap()?;
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        intervals.push(nums);
    }

    // Calculate maximum weight
    let mut return_size = 0;
    let result = maximum_weight(&intervals, &mut return_size);

    // Print result
    for num in result {
        print!("{} ", num);
    }
    println!();

    Ok(())
}