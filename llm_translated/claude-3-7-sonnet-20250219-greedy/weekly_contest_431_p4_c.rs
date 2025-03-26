use std::io::{self, BufRead};
use std::cmp::Ordering;

// Helper macros to mimic the C code
macro_rules! father_node {
    ($i:expr) => {
        if $i == 0 { -1 } else { (($i - 1) >> 1) as i32 }
    };
}

macro_rules! left_node {
    ($i:expr) => {
        (($i << 1) + 1)
    };
}

macro_rules! right_node {
    ($i:expr) => {
        (($i << 1) + 2)
    };
}

macro_rules! higher_int {
    ($i:expr) => {
        ($i >> 32) as i32
    };
}

macro_rules! lower_int {
    ($i:expr) => {
        ($i & 0xFFFFFFFF) as i32
    };
}

macro_rules! merge_long {
    ($i:expr, $j:expr) => {
        ((($i as i64) << 32) | ($j as i64))
    };
}

// Equivalent to DpNode struct in C
struct DpNode {
    max: [i64; 4],
    idx: [u64; 4],
}

impl DpNode {
    fn new() -> Self {
        DpNode {
            max: [0; 4],
            idx: [0; 4],
        }
    }
}

// Equivalent to PriorityQueue struct in C
struct PriorityQueue {
    arr: Vec<i64>,
}

impl PriorityQueue {
    fn new(capacity: usize) -> Self {
        PriorityQueue {
            arr: Vec::with_capacity(capacity),
        }
    }

    fn push(&mut self, value: i64) {
        let mut son = self.arr.len();
        self.arr.push(0); // Placeholder
        let mut father = if son == 0 { -1 } else { (son - 1) >> 1 };
        
        while father != -1 && value < self.arr[father as usize] {
            self.arr[son] = self.arr[father as usize];
            son = father as usize;
            father = father_node!(son);
        }
        
        self.arr[son] = value;
    }

    fn pop(&mut self) {
        if self.arr.is_empty() {
            return;
        }
        
        let last_value = self.arr.pop().unwrap();
        if self.arr.is_empty() {
            return;
        }
        
        let mut father = 0;
        let mut left = left_node!(father);
        let mut right = right_node!(father);
        
        while (left < self.arr.len() && last_value > self.arr[left]) ||
              (right < self.arr.len() && last_value > self.arr[right]) {
            let son = if right < self.arr.len() && self.arr[left] > self.arr[right] {
                right
            } else {
                left
            };
            
            self.arr[father] = self.arr[son];
            father = son;
            left = left_node!(father);
            right = right_node!(father);
        }
        
        self.arr[father] = last_value;
    }
}

fn insert_index(idx: u64, i: i32, count: i32) -> u64 {
    let mut value = [0i32; 4];
    value[0] = (idx >> 48) as i32;
    value[1] = ((idx >> 32) & 0xFFFF) as i32;
    value[2] = ((idx >> 16) & 0xFFFF) as i32;
    value[count as usize] = i;
    
    let mut x = count - 1;
    while x >= 0 && value[x as usize] > i {
        value[(x + 1) as usize] = value[x as usize];
        value[x as usize] = i;
        x -= 1;
    }
    
    ((value[0] as u64) << 48) | 
    ((value[1] as u64) << 32) | 
    ((value[2] as u64) << 16) | 
    (value[3] as u64)
}

fn maximum_weight(intervals: &Vec<Vec<i32>>) -> Vec<i32> {
    let intervals_size = intervals.len();
    let mut left_queue = PriorityQueue::new(intervals_size);
    let mut right_queue = PriorityQueue::new(intervals_size);
    let mut dp = vec![DpNode::new(); intervals_size];
    let mut last = DpNode::new();
    
    // Initialize queues
    for i in 0..intervals_size {
        left_queue.push(merge_long!(intervals[i][0], i as i32));
        right_queue.push(merge_long!(intervals[i][1], i as i32));
    }
    
    let mut max = 0i64;
    let mut sel = u64::MAX;
    let mut return_size = 0;
    
    while !left_queue.arr.is_empty() {
        let i = lower_int!(left_queue.arr[0]) as usize;
        let edge = higher_int!(left_queue.arr[0]);
        
        while !right_queue.arr.is_empty() && edge > higher_int!(right_queue.arr[0]) {
            let j = lower_int!(right_queue.arr[0]) as usize;
            
            for k in 0..3 {
                if last.max[k] < dp[j].max[k] || 
                   (last.max[k] == dp[j].max[k] && last.idx[k] > dp[j].idx[k]) {
                    last.max[k] = dp[j].max[k];
                    last.idx[k] = dp[j].idx[k];
                }
            }
            
            right_queue.pop();
        }
        
        // Copy last to dp[i]
        dp[i].max.copy_from_slice(&last.max);
        dp[i].idx.copy_from_slice(&last.idx);
        
        for k in 0..4 {
            let prev = if k == 0 { 0 } else { last.max[k - 1] };
            
            if k == 0 || (k > 0 && prev > 0) {
                let idx = if k == 0 {
                    (i as u64) << 48
                } else {
                    insert_index(last.idx[k - 1], i as i32, k as i32)
                };
                
                if dp[i].max[k] < intervals[i][2] as i64 + prev || 
                   (dp[i].max[k] == intervals[i][2] as i64 + prev && dp[i].idx[k] > idx) {
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
        
        left_queue.pop();
    }
    
    let mut result = Vec::with_capacity(return_size);
    for i in 0..return_size {
        result.push(((sel >> ((3 - i) << 4)) & 0xFFFF) as i32);
    }
    
    result
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the number of intervals
    let n: usize = lines.next().unwrap()?.trim().parse().unwrap();
    
    // Read the intervals
    let mut intervals = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap()?;
        let values: Vec<i32> = line
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        intervals.push(values);
    }
    
    // Calculate the result
    let result = maximum_weight(&intervals);
    
    // Print the result
    for (i, &val) in result.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", val);
    }
    println!();
    
    Ok(())
}