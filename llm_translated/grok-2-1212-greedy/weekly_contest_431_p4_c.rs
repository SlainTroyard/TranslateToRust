use std::io::{self, BufRead};

// Define constants for bit manipulation
const FATHER_NODE: fn(usize) -> isize = |i| if i == 0 { -1 } else { (i as isize - 1) / 2 };
const LEFT_NODE: fn(usize) -> usize = |i| (i << 1) + 1;
const RIGHT_NODE: fn(usize) -> usize = |i| (i << 1) + 2;
const HIGHER_INT: fn(i64) -> i64 = |i| i >> 32;
const LOWER_INT: fn(i64) -> i64 = |i| i & 0xFFFFFFFF;
const MERGE_LONG: fn(i64, i64) -> i64 = |i, j| (i << 32) | j;

// Define DpNode structure
#[derive(Clone, Copy)]
struct DpNode {
    max: [i64; 4],
    idx: [u64; 4],
}

// Define PriorityQueue structure
struct PriorityQueue {
    arr: Vec<i64>,
}

impl PriorityQueue {
    fn new() -> Self {
        PriorityQueue { arr: Vec::new() }
    }

    fn push(&mut self, value: i64) {
        let mut son = self.arr.len();
        let mut father = FATHER_NODE(son) as usize;
        self.arr.push(value);
        while father != usize::MAX && value < self.arr[father] {
            self.arr[son] = self.arr[father];
            son = father;
            father = FATHER_NODE(son) as usize;
        }
        self.arr[son] = value;
    }

    fn pop(&mut self) {
        let mut father = 0;
        let mut left = LEFT_NODE(father);
        let mut right = RIGHT_NODE(father);
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
            left = LEFT_NODE(father);
            right = RIGHT_NODE(father);
        }
        if !self.arr.is_empty() {
            self.arr[father] = self.arr[self.arr.len() - 1];
        }
    }
}

// Function to insert index
fn insert_index(idx: u64, i: i32, count: i32) -> u64 {
    let mut value = [0; 4];
    value[0] = (idx >> 48) as i32;
    value[1] = ((idx >> 32) & 0xFFFF) as i32;
    value[2] = ((idx >> 16) & 0xFFFF) as i32;
    value[count as usize] = i;
    for x in (0..count).rev() {
        if value[x as usize] > i {
            value[x as usize + 1] = value[x as usize];
            value[x as usize] = i;
        } else {
            break;
        }
    }
    (value[0] as u64) << 48 | (value[1] as u64) << 32 | (value[2] as u64) << 16 | value[3] as u64
}

// Main function to calculate maximum weight
fn maximum_weight(intervals: &Vec<Vec<i32>>, return_size: &mut i32) -> Vec<i32> {
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
        left_queue.push(MERGE_LONG(intervals[i][0] as i64, i as i64));
        right_queue.push(MERGE_LONG(intervals[i][1] as i64, i as i64));
    }

    while !left_queue.arr.is_empty() {
        let i = LOWER_INT(left_queue.arr[0]) as usize;
        let edge = HIGHER_INT(left_queue.arr[0]);
        while !right_queue.arr.is_empty() && edge > HIGHER_INT(right_queue.arr[0]) {
            let j = LOWER_INT(right_queue.arr[0]) as usize;
            for k in 0..3 {
                if last.max[k] < dp[j].max[k]
                    || (last.max[k] == dp[j].max[k] && last.idx[k] > dp[j].idx[k])
                {
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
                    insert_index(last.idx[k - 1], i as i32, k as i32)
                };
                if dp[i].max[k] < intervals[i][2] as i64 + prev
                    || (dp[i].max[k] == intervals[i][2] as i64 + prev && dp[i].idx[k] > idx)
                {
                    dp[i].max[k] = intervals[i][2] as i64 + prev;
                    dp[i].idx[k] = idx;
                }
                if max < dp[i].max[k] || (max == dp[i].max[k] && sel > dp[i].idx[k]) {
                    *return_size = k as i32 + 1;
                    max = dp[i].max[k];
                    sel = dp[i].idx[k];
                }
            }
        }
        left_queue.pop();
    }

    let mut result = Vec::with_capacity(*return_size as usize);
    for i in 0..*return_size as usize {
        result.push((sel >> ((3 - i) * 16) & 0xFFFF) as i32);
    }
    result
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of intervals
    let n: usize = lines.next().unwrap()?.trim().parse().unwrap();

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