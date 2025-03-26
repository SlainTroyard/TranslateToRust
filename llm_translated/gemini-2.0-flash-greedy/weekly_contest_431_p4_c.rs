use std::cmp::Ordering;
use std::io;
use std::io::Read;

// Helper functions (equivalent to C macros)
fn father_node(i: usize) -> Option<usize> {
    if i == 0 {
        None
    } else {
        Some((i - 1) >> 1)
    }
}

fn left_node(i: usize) -> usize {
    (i << 1) + 1
}

fn right_node(i: usize) -> usize {
    (i << 1) + 2
}

fn higher_int(i: i64) -> i64 {
    i >> 32
}

fn lower_int(i: i64) -> i64 {
    i & 0xFFFFFFFF
}

fn merge_long(i: i64, j: i64) -> i64 {
    (i << 32) | (j as i64)
}

// Data structures
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
        while let Some(father) = father_node(son) {
            if value < self.arr[father] {
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
        let mut left = left_node(father);
        let mut right = right_node(father);

        while (left < self.arr.len() && last > self.arr[left])
            || (right < self.arr.len() && last > self.arr[right])
        {
            let son = if right < self.arr.len() && self.arr[left] > self.arr[right] {
                right
            } else {
                left
            };

            self.arr[father] = self.arr[son];
            father = son;
            left = left_node(father);
            right = right_node(father);
        }

        self.arr[father] = last;
        Some(self.arr[0])
    }

    fn len(&self) -> usize {
        self.arr.len()
    }

    fn is_empty(&self) -> bool {
        self.arr.is_empty()
    }

    fn peek(&self) -> Option<&i64> {
        self.arr.get(0)
    }
}

fn insert_index(idx: u64, i: i32, count: usize) -> u64 {
    let mut value = [(idx >> 48) as u16,
                     ((idx >> 32) & 0xFFFF) as u16,
                     ((idx >> 16) & 0xFFFF) as u16,
                     (idx & 0xFFFF) as u16];

    let i_u16 = i as u16;
    value[count] = i_u16;

    let mut x = (count as i32) - 1;
    while x >= 0 && value[x as usize] > i_u16 {
        value[(x + 1) as usize] = value[x as usize];
        value[x as usize] = i_u16;
        x -= 1;
    }

    (value[0] as u64) << 48 | (value[1] as u64) << 32 | (value[2] as u64) << 16 | (value[3] as u64)
}

fn maximum_weight(intervals: &Vec<Vec<i32>>) -> (Vec<i32>, usize) {
    let intervals_size = intervals.len();
    let mut dp = vec![DpNode {
        max: [0; 4],
        idx: [0; 4],
    }; intervals_size];
    let mut last = DpNode {
        max: [0; 4],
        idx: [0; 4],
    };
    let mut left_queue = PriorityQueue::new();
    let mut right_queue = PriorityQueue::new();

    for i in 0..intervals_size {
        left_queue.push(merge_long(intervals[i][0] as i64, i as i64));
        right_queue.push(merge_long(intervals[i][1] as i64, i as i64));
    }

    let mut max = 0;
    let mut sel = 0xFFFFFFFFFFFFFFFF;
    let mut return_size = 0;

    while !left_queue.is_empty() {
        let left_val = left_queue.pop().unwrap();
        let i = lower_int(left_val) as usize;
        let edge = higher_int(left_val);

        while !right_queue.is_empty() && edge > higher_int(right_queue.peek().unwrap()) {
            let right_val = right_queue.pop().unwrap();
            let j = lower_int(right_val) as usize;

            for k in 0..3 {
                if last.max[k] < dp[j].max[k] || (last.max[k] == dp[j].max[k] && last.idx[k] > dp[j].idx[k]) {
                    last.max[k] = dp[j].max[k];
                    last.idx[k] = dp[j].idx[k];
                }
            }
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

                if dp[i].max[k] < intervals[i][2] as i64 + prev || (dp[i].max[k] == intervals[i][2] as i64 + prev && dp[i].idx[k] > idx) {
                    dp[i].max[k] = intervals[i][2] as i64 + prev;
                    dp[i].idx[k] = idx;
                }

                if max < dp[i].max[k] || (max == dp[i].max[k] && sel > dp[i].idx[k]) {
                    return_size = k + 1;
                    max = dp[i].max[k];
                    sel = dp[i].idx[k];
                }
            }
        }
        
    }

    let mut result = Vec::with_capacity(return_size);
    for i in 0..return_size {
        result.push(((sel >> (3 - i << 4)) & 0xFFFF) as i32);
    }

    (result, return_size)
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();
    let n: usize = lines.next().unwrap().parse().unwrap();

    let mut vec: Vec<Vec<i32>> = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap();
        let values: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        vec.push(values);
    }

    let (result, return_size) = maximum_weight(&vec);

    for i in 0..return_size {
        print!("{} ", result[i]);
    }
    println!();

    Ok(())
}